use super::util::*;
use std::io::{Error, ErrorKind};
use winapi::shared::minwindef::DWORD;
use winapi::um::winnt::HANDLE;

pub use sysinfo::Process;
pub use sysinfo::ProcessExt;

pub type ProcessId = usize;
pub type ProcessHandle = *mut winapi::ctypes::c_void;

pub fn close_handle(handle: HANDLE) -> Result<(), Error> {
    if unsafe { winapi::um::handleapi::CloseHandle(handle) } == 0 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn find_process<F>(filter: F) -> Option<ProcessId>
where
    F: Fn(&sysinfo::Process) -> bool,
{
    use sysinfo::SystemExt;

    let mut system = sysinfo::System::new();

    system.refresh_all();

    for (pid, process) in system.get_process_list() {
        if filter(process) {
            return Some(*pid);
        }
    }

    None
}

pub fn open_process(pid: ProcessId) -> Result<ProcessHandle, Error> {
    use winapi::um::processthreadsapi::OpenProcess;
    use winapi::um::winnt::PROCESS_ALL_ACCESS;

    let handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, pid as DWORD) };

    if handle == std::ptr::null_mut() {
        Err(Error::new(
            ErrorKind::Other,
            format!("failed to attach to process: {}", pid),
        ))
    } else {
        Ok(handle)
    }
}

pub fn set_privilege(
    token_handle: HANDLE,
    privilege_name: &'static str,
    enable: bool,
) -> Result<(), Error> {
    use std::ffi::CString;
    use winapi::shared::ntdef::LUID;
    use winapi::um::securitybaseapi::AdjustTokenPrivileges;
    use winapi::um::winbase::LookupPrivilegeValueA;
    use winapi::um::winnt::{LUID_AND_ATTRIBUTES, SE_PRIVILEGE_ENABLED, TOKEN_PRIVILEGES};

    let mut luid = LUID {
        LowPart: 0,
        HighPart: 0,
    };

    let result = unsafe {
        LookupPrivilegeValueA(
            std::ptr::null(),
            CString::new(privilege_name)?.into_raw(),
            &mut luid,
        )
    };

    if result == 0 {
        return Err(Error::last_os_error());
    }

    let mut tp = TOKEN_PRIVILEGES {
        PrivilegeCount: 1,
        Privileges: [LUID_AND_ATTRIBUTES {
            Attributes: if enable { SE_PRIVILEGE_ENABLED } else { 0 },
            Luid: luid,
        }],
    };

    let result = unsafe {
        AdjustTokenPrivileges(
            token_handle,
            0,
            &mut tp,
            std::mem::size_of::<TOKEN_PRIVILEGES>() as u32,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };

    if result == 0 {
        return Err(Error::last_os_error());
    }

    Ok(())
}

pub fn get_debug_privileges() -> Result<(), Error> {
    use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
    use winapi::um::winnt::{SE_DEBUG_NAME, TOKEN_ADJUST_PRIVILEGES};

    let mut token_handle: HANDLE = std::ptr::null_mut();

    let result = unsafe {
        OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_ADJUST_PRIVILEGES,
            &mut token_handle,
        )
    };

    if result == 0 {
        return Err(Error::last_os_error());
    }

    set_privilege(token_handle, SE_DEBUG_NAME, true)?;

    close_handle(token_handle)?;

    Ok(())
}

pub fn get_module_base(
    process_handle: ProcessHandle,
    required_module_name: &str,
) -> Result<usize, Error> {
    use winapi::shared::minwindef::HMODULE;
    use winapi::um::psapi::{EnumProcessModules, GetModuleBaseNameA};

    let mut size_needed: DWORD = 0;

    let result =
        unsafe { EnumProcessModules(process_handle, std::ptr::null_mut(), 0, &mut size_needed) };

    if result == 0 {
        return Err(Error::last_os_error());
    }

    let handle_size = std::mem::size_of::<HMODULE>() as u32;
    let module_count = size_needed / handle_size;

    let mut modules: Vec<HMODULE> = vec![std::ptr::null_mut(); module_count as usize];

    let result = unsafe {
        EnumProcessModules(
            process_handle,
            modules.as_mut_ptr(),
            module_count * handle_size,
            &mut size_needed,
        )
    };

    if result == 0 {
        return Err(Error::new(
            ErrorKind::Other,
            "failed to enumerate process modules",
        ));
    }

    const MODULE_NAME_LEN: usize = 50;
    let mut module_name_buf: [i8; MODULE_NAME_LEN] = [0; MODULE_NAME_LEN];

    for module_handle in modules {
        let read_len = unsafe {
            GetModuleBaseNameA(
                process_handle,
                module_handle,
                &mut module_name_buf[0],
                MODULE_NAME_LEN as DWORD,
            )
        };

        if read_len == 0 {
            continue;
        }

        let cur_name =
            std::str::from_utf8(&realign_unchecked(&module_name_buf)[..read_len as usize])
                .map_err(|_| Error::new(ErrorKind::Other, "failed to convert string"))?;

        if cur_name == required_module_name {
            return Ok(module_handle as usize);
        }
    }

    Err(Error::new(
        ErrorKind::NotFound,
        format!("module not found: {}", required_module_name),
    ))
}
