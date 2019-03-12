//put use statements here
use std::time::Instant;
use std::time::Duration;
use process;
//use process::process_trait;
use basic_process;

#[derive(Clone)]
pub struct Manager {
    _processes : Vec<process::Process>,
    curr : usize,
    size : usize,
    capacity : usize,
    _elapsed : Duration,
}

impl Manager {

    pub fn new() -> Manager {
        Manager {
            _processes : Vec::with_capacity(10),
            curr : 0,
            size : 0,
            capacity : 10,
            _elapsed : Duration::from_millis(0),
        }
    }

	pub fn schedule(&mut self, mut process : process::Process, ms_time : u64) {
		//set the period of the process to the current period
		process._period = Duration::from_millis(ms_time);
        self._processes.push(process);
		//add the process to the vector of processes
		//update the "manager pointer" in the process to point to this manager -- not sure how this would work
	}

    pub fn run(&self, run_time : u64) {
        let dur = Duration::from_millis(run_time);
        let now = Instant::now();
        //start();

        while now.elapsed() < dur {
            //update();
            println!("{:?}", now.elapsed());
       }
       //stop();
    }

    pub fn update(&mut self) {
		//_client.process_responses() -- if we get this far
		for _p in &self._processes {
            //are these references correct? don't need (*_p)
			if self._elapsed > _p.last_update() + _p.period() {
				//(*p)._update(_elapsed);  /* need to implement this function still in process.rs */
			}
		}
	}
	
	pub fn init(&self) {
		for _p in &self._processes {
			//(*p)._init();  /* need to implement this function still in process.rs */
		}
	}
	
	pub fn start(&self) {
		for _p in &self._processes {
		//	(*p)._start();  /* need to implement this function still in process.rs */
		}
	}
	
	pub fn stop(&self) {
		for _p in &self._processes {
			//(*p)._stop();  /* need to implement this function still in process.rs */
		}
	}
}
