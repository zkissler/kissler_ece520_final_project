//use manager;
//use channel;
use std::time::Instant;
use std::time::Duration;

//functions anything that uses this trait must provide
//pub trait process_trait {
    //fn init(&self);
    //fn start(&self);
  //  fn update(&self);
    //fn stop(&self);

//}

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
}

//C style enums, only need to put in the first value and it will count up
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

    pub fn new(name: String, p_type : process_type) -> Process {
        Process {
            _name : name,
            _status : status_type::UNINITIALIZED,
            _period : Duration::from_millis(0),
            _previous_update : Duration::from_millis(0),
            _last_update : Duration::from_millis(0),
            _start_time : Instant::now(),
            _num_updates : 0,
            _process_type : p_type,
            //_update : *Fn(update()),//Box::new(),
            //manager ptr = null;
        }

    }
    pub fn update(&self) {
        match self._process_type {
            process_type::BASIC => {
                println!("{} ", self.name().to_string());
            },
            process_type::BASIC2 => {
               println!("2 {} ", self.name().to_string());
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

    pub fn _init(&mut self) {
        self._status = status_type::STOPPED;
        self.init();
    }

    pub fn _start(&mut self, elapsed : Duration) {
        self._status = status_type::RUNNING;
        self._start_time = Instant::now();
        self._last_update = elapsed;
        self._num_updates = 0;
        self.start();
    }

    pub fn _udpate(&mut self, elapsed : Duration) {
        self._previous_update = self._last_update;
        self._last_update = elapsed;
        self.update();
        self._num_updates = self._num_updates + 1;
    }

    pub fn _stop(&mut self) {
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
}
