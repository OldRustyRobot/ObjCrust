#![crate_type = "staticlib"]
#![feature(core)]
#![feature(io)]
#![feature(libc)]
#![feature(std_misc)]
#![feature(path)]

extern crate libc;

use std::ffi::{c_str_to_bytes};

use std::collections::HashMap;
use std::old_io::timer::sleep;
use std::thread::Thread;
use std::time::Duration;
use std::sync::mpsc::{channel, Sender};

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
            let x = Box::new(sender.clone());
            db_sender = std::mem::transmute(x);
        }

        tx.send(sender).unwrap();

        println!("In daemon receiver");
        sleep(Duration::seconds(3));

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

macro_rules! c_path {
    ($v:expr) => (Path::new(c_string!($v)))
}

macro_rules! c_string {
    ($v:expr) => (unsafe { String::from_utf8(c_str_to_bytes(&$v).to_vec()).unwrap() })
}

#[no_mangle]
pub extern fn rw_file_io(path: *const libc::c_char) -> libc::c_int {
    use std::old_io::{File, Truncate, ReadWrite};
    use std::old_io::Writer;

    let path = c_path!(path);
    match File::open_mode(&path, Truncate, ReadWrite) {
        Err(e) => {
            println!("io: Failed to create file: {}", e);
            -1
        },
        Ok(mut f) => {
            f.write_all("HelloWorld".as_bytes()).unwrap();
            println!("io: Everything ok");
            0
        }
    }
}

#[no_mangle]
pub extern fn rw_file_raw(path: *const libc::c_char) -> libc::c_int {
    use libc;
    use std::mem;

    unsafe {
        let fd = libc::open(path,
                            libc::O_RDWR | libc::O_TRUNC | libc::O_CREAT,
                            libc::S_IRUSR | libc::S_IWUSR);

        if fd < 0 {
            return fd;
        }

        let txt = "HelloWorld";
        libc::write(fd, mem::transmute(txt.as_ptr()), txt.len() as libc::size_t);
        libc::close(fd);
    }
    return 0;
}
