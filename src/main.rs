//! This is the main function that tests the Elma Rust Implementation
pub mod event;
pub mod process;
pub mod manager;
//mod channel;
pub mod basic_process;
pub mod basic_process2;
use std::rc::Rc;
use std::cell::RefCell;

/// The main function implements all the tests for Elma
pub fn main() {

    /* test basic process functionality */
    let a = String::from("Process 1");
    let b = String::from("Process 2");
    let mut m1 = manager::Manager::new("Manager 1".to_string());
    let mut p1 = basic_process::BasicProcess::new(&a, m1.clone());
    let mut p2 = basic_process2::BasicProcess2::new(&b, m1.clone());

    //might need to add all the events here for a particular process
    let e1 = event::Event::new("Hello".to_string());
    let e2 = event::Event::new("This is a basic event".to_string());
    let f: fn() = print_hello;
    p1._process.watch("Hello".to_string(), f);
    
    p1._process.emit(e1);
    p1._process.emit(e2);
    m1.schedule(p1._process, 1);
    m1.schedule(p2._process, 5);
    m1.init();

    m1.run(11 as u64);


    /* This block of code implements the "virtual" functions using traits for two 
       basic processes defined in test.rs. The result of the run is:

       Process 1
       Process 1
       Process 1
       Process 1
       Process 2
       Process 1
       Process 1
       .........
    */

    /*
    let a = String::from("Process 1");
    let b = String::from("Process 2");
    let mut m1 = manager::Manager::new("Manager 1".to_string());
    let p1 = test::BasicProcess::new(a);
    let p2 = test::BasicProcess2::new(b);
    let p1_1 = Rc::new(p1);
    let p2_1 = Rc::new(p2);
    m1.schedule(p1_1, 1);
    m1.schedule(p2_1, 5);

    m1.run(11 as u64);
    */

}

pub fn print_hello() {
    println!("Hello!");
}
