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

    /* test basic process functionality */
    let a = String::from("Process A");
    let a1 = String::from("Process B");
    let mut m1 = manager::Manager::new("Manager 1".to_string());
    let p1 = basic_process::BasicProcess::new(&a, m1.clone());
    let p2 = basic_process2::BasicProcess2::new(&a1, m1.clone());
    m.schedule(p2._process, 1);
    m.schedule(p1._process, 5);

    m.run(11 as u64);
    m.stop();

    //create function
    //call process watch function
    let a3 = String::from("Cruise Control");
    let mut m2 = manager::Manager::new("Manager 2".to_string());
    let p3 = cruise::Cruise::new(&a, m.clone());
    
}

pub fn change_speed(event : event::Event) {

}