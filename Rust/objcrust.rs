#![crate_type = "staticlib"]

extern crate std;
extern crate native;
extern crate libc;
extern crate collections;

use std::collections::hashmap::HashMap;
use libc::c_char;
use std::comm::{channel, Sender};

use std::iter;
use std::rt;
use std::rt::unwind::try;
use std::rt::task::Task;
use std::rt::local::Local;
use std::c_str::CString;
use native::task;

fn ignore_sigpipe() {
    use libc;
    use libc::funcs::posix01::signal::signal;
    unsafe {
        assert!(signal(libc::SIGPIPE, libc::SIG_IGN) != -1);
    }
}

#[allow(dead_code)]
fn run_proc_in_task(f: || -> ()) {
    let something_around_the_top_of_the_stack = 1;
    let addr = &something_around_the_top_of_the_stack as *int;
    let my_stack_top = addr as uint;
    let my_stack_bottom = my_stack_top + 20000 - 512*1024;

    ignore_sigpipe();

    rt::init(0, std::ptr::null());
    let mut task = task::new((my_stack_bottom, my_stack_top));
    task.name = Some(std::str::Slice("<extra-task>"));
    let t = task.run(|| {
        unsafe {
            rt::stack::record_stack_bounds(my_stack_bottom, my_stack_top);
        }
        f();
    });
    drop(t);
}

// Creates a "fake" task which will be presented always
// It is required to have at least one task before
// using std libs as most of them rely on correct
// runtime initialization
#[no_mangle]
pub extern fn register_task(name: *c_char) {
    let something_around_the_top_of_the_stack = 1;
    let addr = &something_around_the_top_of_the_stack as *int;
    let my_stack_top = addr as uint;
    let my_stack_bottom = my_stack_top + 20000 - 512*1024;

    ignore_sigpipe();

    rt::init(0, std::ptr::null());
    let mut task = task::new((my_stack_bottom, my_stack_top));
    task.name = Some(std::str::Owned(unsafe {CString::new(name, false).as_str().unwrap().to_string()}));

    unsafe {
        rt::stack::record_stack_bounds(my_stack_bottom, my_stack_top);
    }

    Local::put(task);
}

#[no_mangle]
pub extern fn deregister_task() {
    let task: Box<Task> = Local::take();
    drop(task);
}

#[no_mangle]
pub extern fn run_rust_main() {
    // Testing hack to get command line arguments into Rust
    println!("Args are {}", std::os::args());
    let _ = unsafe { try(|| { rust_main() }) };
}

pub static mut db_sender: *Sender<int> = 0 as *Sender<int>;

#[no_mangle]
pub extern fn rust_main() {
    println!("Hello from Rust!");
    
    // Hashmap - testing rand module works
    let mut dict = HashMap::new();
    dict.insert(3, 4);
    dict.insert(4, 6);
    
    // Using channels
    let (tx, rx) = channel();

    spawn(proc () {
        let (sender, receiver) = channel::<int>();

        // Testing if global variable will live
        // all the time as boxed value
        unsafe {
            let x = box sender.clone();
            db_sender = std::mem::transmute(x);
        }

        tx.send(sender);

        println!("In daemon receiver");
        std::io::timer::sleep(3000);

        let mut z = 0;

        loop {
            let i = receiver.recv();
            if i == 0 {
                break;
            } else {
                println!("Task got {} [{}]", i, z);
                z += 1;
            }
        }

        println!("Exiting daemon receiver");
    });


    let _ = rx.recv();

    unsafe {
        for i in iter::range(0, 10) {
            (*db_sender).send(i);
        }
    }

    let (tx, rx) = channel();
    tx.send(200);
    spawn(proc() {
        let t = rx.recv();
        println!("Got {} from main thread", t);
        fail!()
    });
    
    // Testing exception handling and reporting
    fail!();
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