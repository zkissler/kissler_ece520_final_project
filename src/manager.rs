//put use statements here
use std::time::Instant;
use std::time::Duration;
use process;
//use process::process_trait;
use basic_process;

pub struct Manager {
    _processes : Vec<process::Process>,
    curr : usize,
    size : usize,
    capacity : usize,
}

impl Manager {

    pub fn new() -> Manager {
        Manager {
            _processes : Vec::with_capacity(10),
            curr : 0,
            size : 0,
            capacity : 10,
        }
    }

    pub fn schedule(&mut self, mut process : process::Process, ms_time : u64) {
        //process._period = Duration::from_millis(ms_time);
        process.update();
        //basic_process::BasicProcess::update(&process);
        //we don't know what the top level process type is
        //self._processes.push(process);
        self.size = self._processes.len();
       // process._manager_ptr = this; 
    }

    pub fn run(&self, run_time : u64) {
        let dur = Duration::from_millis(run_time);
        let now = Instant::now();
        //start function

        while (now.elapsed() < dur) {
            //call update function
            println!("{:?}", now.elapsed());
       }
       //stop function
    }
}
