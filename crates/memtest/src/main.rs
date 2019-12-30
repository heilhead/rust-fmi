#[macro_use]
extern crate text_io;

struct MyStruct {
    pub val: i32,
}

fn update_val(ms: &mut MyStruct, val: i32) {
    ms.val = val;
}

fn main() {
    let mut count = 0;
    let mut num = 0i32;
    let mut my_struct = Box::new(MyStruct { val: 0 });

    loop {
        println!("enter next number:");

        count += 1;

        // let tmp_num: i32 = read!();
        num = read!();

        // println!("current num: {}", num);
        // println!("current msn: {}", my_struct.val);
        // println!("new num    : {}", tmp_num);
        // println!("count      : {}", count);

        update_val(&mut my_struct, num);

        // num = tmp_num;
    }
}
