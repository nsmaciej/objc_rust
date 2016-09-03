#![feature(link_args)]
#[macro_use]
extern crate objc;

use objc::runtime::*;

#[cfg(feature = "link")]
#[link_args = "-ObjC"]
#[link(name = "sample")]
extern {}

#[cfg(feature = "object")]
#[link_args = "libsample.o"]
extern {}

fn main() {
    let sample_c = Class::get("Sample").unwrap();
    unsafe {
        let sample: *mut Object = msg_send![sample_c, new];
        let _: () = msg_send![sample, sayHello];
        let _: () = msg_send![sample, release];
    }
    println!("Hello rust!");
}
