//! Contains the 'Process' structure and implementation

use manager;
use event;
use std::time::Instant;
use std::time::Duration;
use std::ptr;

/// The 'Process' structure contains all the data required to create an Elma process.
#[derive(Clone)]
pub struct Process {
    /// The name of the Process
    pub _name: String,
    /// Keeps track fo the 'status_type' enum
	pub _status : status_type,
    /// Request time between updates
    pub _period : Duration,
    /// Duration from start to update before last
	pub _previous_update : Duration,
    /// Duration from start to last update
    pub _last_update : Duration,
    /// Time of the most recent start
    pub _start_time : Instant,
    /// The number of times the 'update()' method has been called
    pub _num_updates : usize,
    /// Keeps track of the 'process_type' enum
    pub _process_type : process_type,
    /// Keeps track of the Manager for this Process
    pub _manager_ptr : manager::Manager,
}

/// Status of the process, as managed by a Manager. Get the status using the status() getter.
#[derive(Clone)]
pub enum status_type { 
	UNINITIALIZED = 0, 
	STOPPED, 
	RUNNING, 
}

/// Keeps track of the type of Process. To be used in the start, stop, init, and update functions
#[derive(Clone)]
pub enum process_type { 
	BASIC = 0, 
	BASIC2,
    CRUISE,  
}


impl Process {
    /// Creates a new Process instance
    pub fn new(name: String, p_type : process_type, m : manager::Manager) -> Process {
        Process {
            _name : name,
            _status : status_type::UNINITIALIZED,
            _period : Duration::from_millis(0),
            _previous_update : Duration::from_millis(0),
            _last_update : Duration::from_millis(0),
            _start_time : Instant::now(),
            _num_updates : 0,
            _process_type : p_type,
            _manager_ptr : m,
        }

    }

    /// Tells the Manager to watch for the given Event and respond with the appropriate handler function
    pub fn watch(&mut self, event_name : String, handler : fn()) {
        self._manager_ptr.watch(event_name, handler);
    }

    /// Tells the manager to emit the passed in Event
    pub fn emit(&self, e : event::Event) {
        //tells the manager that it needs to emit a the event
        self._manager_ptr.emit(e);
    }

    /// Calls the 'update()' function for the given Process
    pub fn update_all(&mut self, elapsed : Duration) {
        self._previous_update = self._last_update;
        self._last_update = elapsed;
        self.update();
        self._num_updates = self._num_updates + 1;
    }

    /// Manager interface for the 'update()' method
    pub fn update(&self) {
        match self._process_type {
            process_type::BASIC => {
                println!("{} ", self.name().to_string());
            },
            process_type::BASIC2 => {
               println!("{} ", self.name().to_string());
            },
            process_type::CRUISE => {
                //let e = event::Event::new()
                //emit(Event("desired speed", 40));
            },

        }
    }
	
    /// Manager interface for the 'init()' method
	pub fn init(&mut self) { 
         match self._process_type { 
             process_type::BASIC => {
                /* This doesn't work
                 let f: fn() = print_basic;
                 self.watch("This is a basic event".to_string(), f);
                 */
             }, 
             process_type::BASIC2 => {}, 
             process_type::CRUISE => {}, 
  
         }
         

    } 
	
    /// Manager interface for the 'start()' method
	pub fn start(&self) { 
         match self._process_type { 
             process_type::BASIC => {}, 
             process_type::BASIC2 => {}, 
             process_type::CRUISE => {},
         } 
    } 
	
    /// Manager interface for the 'stop()' method
	pub fn stop(&self) { 
         match self._process_type { 
             process_type::BASIC => {}, 
             process_type::BASIC2 => {}, 
             process_type::CRUISE => {},
         } 
    }

    /// Calls the 'init()' function for the given Process
    pub fn init_all(&mut self) {
        self._status = status_type::STOPPED;
        self.init();
    }

    /// Calls the 'start()' function for the given Process
    pub fn start_all(&mut self, elapsed : Duration) {
        self._status = status_type::RUNNING;
        self._start_time = Instant::now();
        self._last_update = elapsed;
        self._num_updates = 0;
        self.start();
    }

    /// Calls the 'stop()' function for the given Process
    pub fn stop_all(&mut self) {
        self._status = status_type::STOPPED;
        self.stop();
    }

    /// Returns the name of the process
    pub fn name(&self) -> String {
        return self._name.to_string();
    }

    /// Returns the period for the given Process
    pub fn period(&self) -> Duration {
        return self._period;
    }

    /// Returns the number of updates for the given Process
    pub fn num_updates(self) -> usize {
        return self._num_updates;
    }

    /// Returns the time the Process was last started by the Manager
    pub fn start_time(self) -> Instant {
        return self._start_time;
    }

    /// Returns the duration of time between the start time and the most recent time the Manager called the 'update()' method
    pub fn last_update(&self) -> Duration {
        return self._last_update;
    }

    /// Returns the duration of time between the start time and the second most recent time the Manager called the 'update()' method 
    pub fn previous_update(self) -> Duration {
        return self._previous_update;
    }

    /// Returns the time since the last update, in milliseconds
    pub fn milli_time(&self) -> Duration {
        return self._last_update;
    }
}
pub fn print_basic() {
    println!("Here is my basic event");
}
