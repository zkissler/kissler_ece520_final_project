/// \filename manager.rs
/// \brief description
use std::time::Instant;
use std::time::Duration;
use std::collections::HashMap;
use process;
use event;
//use process::process_trait;
use basic_process;

#[derive(Clone)]
pub struct Manager {
    _name : String,
    _processes : Vec<process::Process>,
    curr : usize,
    size : usize,
    capacity : usize,
    _elapsed : Duration,
    _start_time : Instant,
    _event_handlers : HashMap<String, fn()>,
    //map<string, vector<std::function<void(Event&)>>> event_handlers;
}

impl Manager {

    pub fn new(name : String) -> Manager {
        Manager {
            _name : name,
            _processes : Vec::with_capacity(10),
            curr : 0,
            size : 0,
            capacity : 10,
            _elapsed : Duration::from_millis(0),
            _start_time : Instant::now(),
            _event_handlers : HashMap::new(),
        }
    }

    pub fn watch(&mut self, event_name : String, handler : fn()) {
        self._event_handlers.insert(event_name, handler);
    }

    pub fn emit(&self, event : event::Event) {
        let e = event;
        if self._event_handlers.contains_key(&e.clone().name()) {
            if e.propagate() {
                self._event_handlers[&e.clone().name()]();
            }
        }

    }

//priority
	pub fn schedule(&mut self, mut process : process::Process, ms_time : u64) {
		process._period = Duration::from_millis(ms_time);
        self._processes.push(process);
		//update the "manager pointer" in the process to point to this manager -- not sure how this would work
	}

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

    pub fn update(&mut self) {
		//_client.process_responses() -- if we get this far
		for _p in &mut self._processes {
           // println!("{:?}  {:?}  {:?}", self._elapsed, _p.last_update(), _p.period());
			if self._elapsed > _p.last_update() + _p.period() {
				(*_p).update_all(self._elapsed);  
			}
		}
	}
	
	pub fn init(&mut self) {
		for _p in &mut self._processes {
			(*_p).init_all();
		}
	}
	
	pub fn start(&mut self) {
		for _p in &mut self._processes {
			(*_p).start_all(self._elapsed);
		}
	}
	
	pub fn stop(&mut self) {
		for _p in &mut self._processes {
			(*_p).stop_all();
		}
	}

    pub fn name(&self) -> String {
        return self._name.to_string();
    }
}
