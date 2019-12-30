use super::process::ProcessHandle;
use std::io::Error;
use winapi::um::memoryapi::ReadProcessMemory;

pub fn read<T>(handle: ProcessHandle, address: usize) -> Result<T, Error>
where
    T: Sized + Default,
{
    let mut buf: T = Default::default();
    let len_required = std::mem::size_of::<T>();
    let mut len_actual: usize = 0;

    let success = unsafe {
        ReadProcessMemory(
            handle,
            address as _,
            &mut buf as *mut _ as _,
            len_required,
            &mut len_actual,
        )
    };

    if success == 0 || len_actual != len_required {
        Err(Error::last_os_error())
    } else {
        Ok(buf)
    }
}

pub fn read_slice<T>(handle: ProcessHandle, address: usize, buf: &mut [T]) -> Result<(), Error>
where
    T: Sized,
{
    let len_required = buf.len() * std::mem::size_of::<T>();
    let mut len_actual: usize = 0;

    let success = unsafe {
        ReadProcessMemory(
            handle,
            address as _,
            buf as *mut _ as _,
            len_required,
            &mut len_actual,
        )
    };

    if success == 0 || len_actual != len_required {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn read_buf(handle: ProcessHandle, address: usize, length: usize) -> Result<Vec<u8>, Error> {
    let mut result: Vec<u8> = Vec::with_capacity(length);
    unsafe { result.set_len(length) };

    read_slice(handle, address, &mut result[..])?;

    Ok(result)
}
