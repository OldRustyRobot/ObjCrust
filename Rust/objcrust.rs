
#![crate_type = "staticlib"]
#![allow(ctypes)]

extern crate native;
//extern crate green;
//extern crate rustuv;
extern crate collections;
extern crate url;
extern crate glob;

use collections::hashmap::HashMap;
use url::decode;
use glob::glob;
use std::comm::channel;

#[no_mangle]
fn test_main() {
    println!("Hello from Rust!");

    let url = decode(&"https://example.com/Rust%20(programming%20language)");
    println!("{}", url);

    for path in glob("/var/mobile/Applications/D224A5B9-D0DD-4ED9-888F-5599C64D7035/*") {
        println!("{}", path.display());
    }

    spawn(proc() {
        println!("Hello from task!");
    });

    let (tx, rx) = channel();
    tx.send(200);
    spawn(proc() {
        let t = rx.recv();
        println!("Got {} from main thread", t);
    }); 

    let mut x = HashMap::new();

    x.insert("k1", 48);
    x.insert("k2", 42);
    
    let k4 = "k2";
    let z = match x.find(&k4) {
        Some(num) => *num,
        None => 0
    };

    println!("Answer to everything is {}", z);
}

#[no_mangle]
pub extern fn try_init() {
    //green::start(0, std::ptr::null(), rustuv::event_loop, test_main);
    native::start(0, std::ptr::null(), test_main);
}

pub struct Pair {
    foo: uint,
    bar: uint,
}

pub struct Complex {
    real: f64,
    img: f64,
}

#[no_mangle]
pub extern fn get_num() -> uint {        
    32
}

#[no_mangle]
pub extern fn get_float() -> f64 {
    42.42
}

#[no_mangle]
pub extern fn inc_num(x: uint) -> uint {
    x + 1
}

#[no_mangle]
pub extern fn add_nums(num1: uint, num2: uint) -> uint {
    num1 + num2
}

#[no_mangle]
pub extern fn get_num_ptr(num: &uint) -> uint {
    *num
}

#[no_mangle]
pub extern fn inc_num_ptr(num: &mut uint) -> uint {
    *num += 1;
    *num
}

#[no_mangle]
pub extern fn inc_float_ptr(num: &mut f64) -> f64 {
    *num += 1.0;
    *num
}

#[no_mangle]
pub extern fn get_pair() -> Pair {
    Pair {
        foo: 42,
        bar: 10,
    }
}

#[no_mangle]
pub extern fn inc_pair(pair: Pair) -> Pair {
    Pair {
        foo: pair.foo + 1,
        bar: pair.bar + 1,
    }
}

#[no_mangle]
pub extern fn inc_pair_ptr(pair: &mut Pair) -> Pair {
    pair.foo += 1;
    pair.bar += 1;
    *pair
}

#[no_mangle]
pub extern fn get_complex() -> Complex {
    Complex {
        real: 10.0,
        img: 42.0,
    }
}

#[no_mangle]
pub extern fn inc_complex(comp: Complex) -> Complex {
    Complex {
        real: comp.real + 1.0,
        img: comp.img + 1.0,
    }
}

#[no_mangle]
pub extern fn inc_complex_ptr(comp: &mut Complex) {
    comp.real += 1.0;
    comp.img += 1.0;
}
