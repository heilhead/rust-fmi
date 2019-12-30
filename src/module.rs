use super::pattern::{MatchType, MemoryPattern};
use fmi::{ModuleBounds, ProcessHandle, ProcessId};
use std::io::Error;
use std::ops::Drop;

pub struct ProgramModule {
    pub handle: ProcessHandle,
    pub bounds: ModuleBounds,
    pub snapshot: Option<Vec<u8>>,
}

impl Drop for ProgramModule {
    fn drop(&mut self) {
        fmi::close_handle(self.handle).expect("failed to close program handle");
    }
}

impl ProgramModule {
    pub fn new(pid: ProcessId, module_name: &str) -> Result<ProgramModule, Error> {
        let handle = fmi::open_process(pid)?;
        let bounds = fmi::get_module_bounds(handle, module_name)?;
        let snapshot = None;

        Ok(ProgramModule {
            handle,
            bounds,
            snapshot,
        })
    }

    pub fn load_snapshot(&mut self) -> Result<(), Error> {
        let (start, end) = self.bounds;
        self.snapshot = Some(fmi::read_buf(self.handle, start, end - start)?);
        Ok(())
    }

    pub fn free_snapshot(&mut self) {
        self.snapshot = None;
    }

    pub fn search_memory(&self, pattern: &MemoryPattern) -> Option<usize> {
        assert!(self.snapshot.is_some());

        let data = &pattern.data;
        let data_len = data.len();
        let offset = &pattern.offset;
        let memory = self.snapshot.as_ref().unwrap();
        let memory_len = memory.len();

        if data_len + offset > memory.len() {
            return None;
        }

        let mut pos = 0usize;

        while pos + data_len < memory_len {
            let first_byte_match = memory[pos + offset] == data[0].0;
            let last_byte_match = memory[pos + offset + data_len - 1] == data[data_len - 1].0;

            if first_byte_match && last_byte_match {
                let mut full_match = true;

                for i in 1..data_len - 1 {
                    let mem_byte = memory[pos + offset + i];
                    let pat_byte = data[i].0;

                    let byte_match = match data[i].1 {
                        MatchType::None => true,
                        MatchType::Both => mem_byte == pat_byte,
                        MatchType::Left | MatchType::Right => (mem_byte & pat_byte) == pat_byte,
                    };

                    if !byte_match {
                        full_match = false;
                        break;
                    }
                }

                if full_match {
                    return Some(pos);
                }
            }

            pos += 1;
        }

        None
    }

    pub fn get_offset(&self, offset: usize) -> usize {
        self.bounds.0 + offset
    }
}
