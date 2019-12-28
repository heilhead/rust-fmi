#[macro_use]
extern crate text_io;

fn main() {
    let mut count = 0;
    let mut num = 0i32;

    loop {
        println!("enter next number:");

        count += 1;

        let tmp_num: i32 = read!();

        println!("current num: {}", num);
        println!("new num    : {}", tmp_num);
        println!("count      : {}", count);

        num = tmp_num;
    }
}
