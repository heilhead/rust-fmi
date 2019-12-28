extern crate sysinfo;

mod memory;
mod process;
mod util;

pub use memory::*;
pub use process::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
