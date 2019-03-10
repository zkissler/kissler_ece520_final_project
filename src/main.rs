//mod transition;
//mod event;
//mod state;
mod process;
mod basic_process;
mod basic_process2;
mod manager;
//mod channel;

//for json if we use it
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
//extern crate serialize;

//use process::process_trait;

fn main() {

    let a = String::from("Process A");
    let mut m = manager::Manager::new();
    let p = basic_process::BasicProcess::new(&a);
    let p2 = basic_process2::BasicProcess2::new(&a);
    m.schedule(p2._process, 1);
    m.schedule(p._process, 1);
    //p.update();

    //m.run(5 as u64);

    //basic test case
    //let m = manager::Manager; // Make a manager
    //basic_example:: BasicProcess p("A"), q("B"); 
    
    // Make two processes
    //let p = basic_process::BasicProcess("A");
    //let p = basic_process::BasicProcess("B");

    //m.schedule(p, 1_ms); // Schedule the first to run every millisecond
    //m.schedule(q, 5_ms); // Schedule the second to run every 5 milliseconds
    //m.init();            // Initialize the manager (which calls init for the processes)
    //m.run(11_ms);       // Run for 11 milliseconds



}