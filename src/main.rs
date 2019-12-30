extern crate fmi;

mod module;
#[macro_use]
mod pattern;

use module::ProgramModule;
use pattern::MemoryPattern;
use std::io::{Error, ErrorKind};

fn main() {
    match test() {
        Ok(_) => println!("fmi test complete"),
        Err(err) => println!("fmi test error: {}", err),
    };
}

fn test() -> Result<(), Error> {
    use fmi::ProcessExt;

    /*
    memtest.memtest::main+BB - 8B 45 B0              - mov eax,[rbp-50]
    memtest.memtest::main+BE - 89 45 C4              - mov [rbp-3C],eax
    memtest.memtest::main+C1 - 48 8B 4D C8           - mov rcx,[rbp-38]
    memtest.memtest::main+C5 - 8B 55 C4              - mov edx,[rbp-3C]
    memtest.memtest::main+C8 - E8 13FFFFFF           - call memtest.memtest::update_val
    */
    let num_pattern = MemoryPattern::from_string(
        0,
        r#"
            8B 45 B0
            89 45 C4
            48 8B 4D C8
            8B 55 C4
            E8 13 FF FF FF
        "#,
    )?;

    fmi::get_debug_privileges()?;

    let proc_name = "memtest.exe";

    let processes = fmi::get_process_list(Some(|p: &fmi::Process| p.name() == proc_name));

    if processes.len() == 0 {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("process not found: {}", proc_name),
        ));
    }

    let mut module = ProgramModule::new(processes[0].pid(), proc_name)?;

    module.load_snapshot()?;

    let num_address_idx = module.search_memory(&num_pattern);

    module.free_snapshot();

    if let Some(idx) = num_address_idx {
        let offset: u8 = fmi::read(module.handle, module.get_offset(idx + 8))?;

        println!("offset read {:#X}", offset);

        let offset: usize = offset as usize;

        let num_address: i32 = fmi::read(module.handle, module.get_offset(offset))?;

        println!("found num address: {}", num_address);

        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "memory pattern not found"))
    }
}
