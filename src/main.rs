//! This is the main function that tests the Elma Rust Implementation
pub mod event;
pub mod process;
pub mod manager;
//mod channel;
pub mod basic_process;
pub mod basic_process2;
pub mod state_machine;
pub mod state;
pub mod transition;

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

    /* calls the process watch function with the associated function */
    let f: fn() = print_hello;
    p1._process.watch("Hello".to_string(), f);

    let f1: fn() = print_basic;
    p2._process.watch("This is a basic event".to_string(), f1);

    /* emits the events */
    p1._process.emit(e1);
    p2._process.emit(e2);

    m1.schedule(p1._process, 1);
    m1.schedule(p2._process, 5);
    m1.init();

    m1.run(11 as u64);


    /* Example of State Machine Functionality */
    let state1 = state::State::new("State 1".to_string());
    let state2 = state::State::new("State 2".to_string());

    let mut m2 = manager::Manager::new("Manager 1".to_string());
    let mut s1 = state_machine::StateMachine::new(state1.clone(), state2.clone());

    s1.set_initial(state1.clone());
    let e5 = s1.current();
    println!("Current State: {}", e5.name());
    s1.add_transition("switch".to_string(), state1, state2);
    let e3 = event::Event::new("switch".to_string());
    m2.start();
    m2.emit(e3);


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
pub fn print_basic() {
    println!("Here is my basic event");
}