#[macro_use]
extern crate objc;

use objc::runtime::*;

#[link(name = "sample")]
extern {
    #[link_name = "OBJC_CLASS_$_Sample"]
    static SAMPLE_C: Class;
}

fn main() {
    unsafe {
        let sample: *mut Object = msg_send![&SAMPLE_C, new];
        let _: () = msg_send![sample, sayHello];
        let _: () = msg_send![sample, release];
    }
    println!("Hello rust!");
}
