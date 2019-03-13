//mod transition;
mod event;
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

fn main() {

    let a = String::from("Process A");
    let a1 = String::from("Process B");
    let mut m = manager::Manager::new("Manager 1".to_string());
    let p = basic_process::BasicProcess::new(&a, m.clone());
    //m.update();
    let p2 = basic_process2::BasicProcess2::new(&a1, m.clone());
    m.schedule(p2._process, 1);
    m.schedule(p._process, 5);
    //p.update();

    m.run(11 as u64);

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