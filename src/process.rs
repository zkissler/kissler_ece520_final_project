/** @file process.rs
 *  @brief This file contains the code for the process interface
 */
use manager;
use event;
use std::time::Instant;
use std::time::Duration;
use std::ptr;


#[derive(Clone)]
pub struct Process {
    _name: String,
	_status : status_type,
    pub _period : Duration,          // request time between updates 
	_previous_update : Duration, // duration from start to update before last 
    _last_update : Duration,     // duration from start to last update 
    _start_time : Instant,    // time of most recent start 
    _num_updates : usize,                                 // number of times update() has been called 
    _process_type : process_type,
    pub _manager_ptr : manager::Manager,
}

#[derive(Clone)]
pub enum status_type { 
	UNINITIALIZED = 0, 
	STOPPED, 
	RUNNING, 
}

#[derive(Clone)]
pub enum process_type { 
	BASIC = 0, 
	BASIC2,  
}


impl Process {

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

    pub fn watch(&self, event_name : String, handler : &Fn()) {
        //tells the manager that it needs to watch for the event
        self._manager_ptr.watch(event_name, handler);
    }

    pub fn emit(&self, e : event::Event) {
        //tells the manager that it needs to emit a the event
        self._manager_ptr.emit(e);
    }

    pub fn update_all(&mut self, elapsed : Duration) {
        self._previous_update = self._last_update;
        self._last_update = elapsed;
        self.update();
        self._num_updates = self._num_updates + 1;
    }

    pub fn update(&self) {
        match self._process_type {
            process_type::BASIC => {
                println!("{} ", self.name().to_string());
            },
            process_type::BASIC2 => {
               println!("{} ", self.name().to_string());
            },

        }
    }
	
	pub fn init(&self) { 
         match self._process_type { 
             process_type::BASIC => {}, 
             process_type::BASIC2 => {}, 
  
         }
         

    } 
	
	pub fn start(&self) { 
         match self._process_type { 
             process_type::BASIC => {}, 
             process_type::BASIC2 => {}, 
  
         } 
    } 
	
	pub fn stop(&self) { 
         match self._process_type { 
             process_type::BASIC => {}, 
             process_type::BASIC2 => {}, 
  
         } 
    }

    pub fn init_all(&mut self) {
        self._status = status_type::STOPPED;
        self.init();
    }

    pub fn start_all(&mut self, elapsed : Duration) {
        self._status = status_type::RUNNING;
        self._start_time = Instant::now();
        self._last_update = elapsed;
        self._num_updates = 0;
        self.start();
    }

    pub fn stop_all(&mut self) {
        self._status = status_type::STOPPED;
        self.stop();
    }

    pub fn name(&self) -> String {
        return self._name.to_string();
    }

    pub fn period(&self) -> Duration {
        return self._period;
    }

    pub fn num_updates(self) -> usize {
        return self._num_updates;
    }

    pub fn start_time(self) -> Instant {
        return self._start_time;
    }

    pub fn last_update(&self) -> Duration {
        return self._last_update;
    }

    pub fn previous_update(self) -> Duration {
        return self._previous_update;
    }

    pub fn milli_time(&self) -> Duration {
        return self._last_update;
    }
}
