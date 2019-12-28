extern crate fmi;

use std::io::{Error, ErrorKind};

fn main() {
    match test() {
        Ok(_) => println!("fmi test complete"),
        Err(err) => println!("fmi test error: {}", err),
    };
}

fn test() -> Result<(), Error> {
    use fmi::ProcessExt;

    fmi::get_debug_privileges().expect("failed to get debug privileges");

    let proc_name = "memtest.exe";

    let pid = fmi::find_process(|p: &fmi::Process| p.name() == proc_name).ok_or(Error::new(
        ErrorKind::NotFound,
        format!("process not found: {}", proc_name),
    ))?;

    println!("pid of {}: {}", proc_name, pid);

    let handle = fmi::open_process(pid)?;
    let base = fmi::get_module_base(handle, proc_name)?;

    println!("module base: {:#X}", base);

    let num_offset: usize = 0x20E1;
    let num = fmi::read_int(handle, base + num_offset)?;

    println!("read num: 0x{:#X}", num);

    fmi::close_handle(handle)?;

    Ok(())
}
