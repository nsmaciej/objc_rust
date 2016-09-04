#[macro_use]
extern crate objc;

use objc::runtime::*;

#[link(name = "sample")]
extern {
    fn dummy();
}

fn main() {
    let sample_c = Class::get("Sample").unwrap();
    unsafe {
        dummy();
        let sample: *mut Object = msg_send![sample_c, new];
        let _: () = msg_send![sample, sayHello];
        let _: () = msg_send![sample, release];
    }
    println!("Hello rust!");
}
