use super::process::ProcessHandle;
use std::io::{Error, ErrorKind};
use winapi::shared::basetsd::SIZE_T;

pub fn read_int(handle: ProcessHandle, address: usize) -> Result<u32, Error> {
  use winapi::um::memoryapi::ReadProcessMemory;

  let buf = 0u32;
  let size_required: SIZE_T = 4;
  let mut size_actual: SIZE_T = 0;

  let success = unsafe {
    ReadProcessMemory(
      handle,
      address as _,
      &buf as *const u32 as *mut winapi::ctypes::c_void,
      size_required,
      &mut size_actual,
    )
  };

  if success == 0 || size_actual != size_required {
    Err(Error::last_os_error())
  } else {
    Ok(buf)
  }
}
