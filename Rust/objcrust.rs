#![crate_type = "staticlib"]
#![feature(box_syntax)]
#![allow(unstable)]

extern crate libc;

use std::collections::HashMap;
use std::sync::mpsc::{channel, Sender};
use std::thread::Thread;
use std::time::Duration;

pub static mut db_sender: *mut Sender<i32> = 0 as *mut Sender<i32>;

#[no_mangle]
pub extern fn rust_main() {
    println!("Hello from Rust!");

    // Hashmap - testing rand module works
    let mut dict = HashMap::new();
    dict.insert(3, 4);
    dict.insert(4, 6);

    // Using channels
    let (tx, rx) = channel();

    Thread::spawn(move || {
        let (sender, receiver) = channel::<i32>();

        // Testing if global variable will live
        // all the time as boxed value
        unsafe {
            let x = box sender.clone();
            db_sender = std::mem::transmute(x);
        }

        tx.send(sender).unwrap();

        println!("In daemon receiver");
        std::io::timer::sleep(Duration::seconds(3));

        loop {
            let i = receiver.recv().unwrap();
            if i == 0 {
                break;
            } else {
                println!("Task got {}", i);
            }
        }

        println!("Exiting daemon receiver");
    });

    rx.recv().unwrap();

    unsafe {
        for i in range(1, 10) {
            (*db_sender).send(i).unwrap();
        }

        (*db_sender).send(0).unwrap();
    }
}

#[derive(Copy)]
pub struct Pair {
    foo: u32,
    bar: u32,
}

#[derive(Copy)]
pub struct Complex {
    real: f64,
    img: f64,
}

#[no_mangle]
pub extern fn get_num() -> u32 {
    32
}

#[no_mangle]
pub extern fn get_float() -> f64 {
    42.42
}

#[no_mangle]
pub extern fn inc_num(x: u32) -> u32 {
    x + 1
}

#[no_mangle]
pub extern fn add_nums(num1: u32, num2: u32) -> u32 {
    num1 + num2
}

#[no_mangle]
pub extern fn get_num_ptr(num: &u32) -> u32 {
    *num
}

#[no_mangle]
pub extern fn inc_num_ptr(num: &mut u32) -> u32 {
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
