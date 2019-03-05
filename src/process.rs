//use manager;
//use channel;
use std::time::Instant;

pub struct Process {
    _name: String,
	_status : status_type,
    _period : Instant,          // request time between updates 
	_previous_update : Instant, // duration from start to update before last 
    _last_update : Instant,     // duration from start to last update 
    _start_time : Instant,    // time of most recent start 
    _num_updates: usize,                                 // number of times update() has been called 
    //_manager_ptr : Manager *,                           // a pointer to the manager    
}

//C style enums, only need to put in the first value and it will count up
enum status_type { 
	UNINITIALIZED = 0, 
	STOPPED, 
	RUNNING, 
}

impl Process {
    //have more than 1 constructor in rust?
    pub fn process() {

    }

    //what about virtual functions? just don't put them in parent class?

   // pub fn name(self) -> status_type {
     //   return self._name;
    //}

    pub fn name(self) -> status_type {
        return self._status;
    }

    pub fn period(self) -> Instant {
        return self._period;
    }

    pub fn num_updates(self) -> usize {
        return self._num_updates;
    }

    pub fn start_time(self) -> Instant {
        return self._start_time;
    }

    pub fn last_update(self) -> Instant {
        return self._last_update;
    }

    pub fn previous_update(self) -> Instant {
        return self._previous_update;
    }
}