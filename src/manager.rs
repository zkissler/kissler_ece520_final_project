//! Contains the 'Manager' structure and implementation

use std::time::Instant;
use std::time::Duration;
use std::collections::HashMap;
use process;
use event;
//use process::process_trait;
use basic_process;
use std::rc::Rc;

/// The 'Manager' structure contains all the data required to create an Elma process.
#[derive(Clone)]
pub struct Manager {
    /// The name of the Manager
    pub _name : String,
    /// Stores all the Processes for this Manager
    pub _processes : Vec<process::Process>,
    /// Stores the current size
    pub size : usize,
    /// Stores the capacity of the current Manager
    pub capacity : usize,
    /// Returns the elapsed amount of time
    pub _elapsed : Duration,
    /// Stores the start time
    pub _start_time : Instant,
    /// A map of the event_names to the handlers
    pub _event_handlers : HashMap<String, fn()>,
}

impl Manager {
    /// Creates a new Manager instance
    pub fn new(name : String) -> Manager {
        Manager {
            _name : name,
            _processes : Vec::with_capacity(10),
            size : 0,
            capacity : 10,
            _elapsed : Duration::from_millis(0),
            _start_time : Instant::now(),
            _event_handlers : HashMap::new(),
        }
    }
    /// The 'watch' function puts the event name and the associated handler in the event handler map
    pub fn watch(&mut self, event_name : String, handler : fn()) {
        self._event_handlers.insert(event_name, handler);
    }

    /// Emits the particular Event and calls the handler function
    pub fn emit(&self, event : event::Event) {
        let e = event;
        if self._event_handlers.contains_key(&e.clone().name()) {
            if e.propagate() {
                self._event_handlers[&e.clone().name()]();
            }
        }

    }

    //priority missing
    /// Adds the passed in Process to the scheduler for this Manager
	pub fn schedule(&mut self, mut process : process::Process, ms_time : u64) {
		process._period = Duration::from_millis(ms_time);
        self._processes.push(process);
		//update the "manager pointer" in the process to point to this manager -- not sure how this would work
	}

    /// Starts and stops all the Processes and calls their 'update()' function if the proper amount of time has passed
    pub fn run(&mut self, run_time : u64) {
        let dur = Duration::from_millis(run_time);
        self._start_time = Instant::now();
        self.start();
        while self._start_time.elapsed() < dur {
            self.update();
            //println!("{:?}", now.elapsed());
            self._elapsed = self._start_time.elapsed()
       }
       self.stop();
    }

    /// Calls the 'update()' function for all the Processes
    pub fn update(&mut self) {
		//_client.process_responses() -- if we get this far
		for _p in &mut self._processes {
           // println!("{:?}  {:?}  {:?}", self._elapsed, _p.last_update(), _p.period());
			if self._elapsed > _p.last_update() + _p.period() {
				(*_p).update_all(self._elapsed);  
			}
		}
	}
	
    /// Calls the 'init()' function for all the Processes
	pub fn init(&mut self) {
        println!("in manager init");
		for _p in &mut self._processes {
			(*_p).init_all();
		}
	}
	
    /// Calls the 'start()' function for all the Processes
	pub fn start(&mut self) {
		for _p in &mut self._processes {
			(*_p).start_all(self._elapsed);
		}
	}
	
    /// Calls the 'stop()' function for all the Processes
	pub fn stop(&mut self) {
		for _p in &mut self._processes {
			(*_p).stop_all();
		}
	}

    /// Returns the name of the current Manager
    pub fn name(&self) -> String {
        return self._name.to_string();
    }
}
