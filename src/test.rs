pub trait Process {
    /* fn initt();
    pub fn start();
    pub fn stop(); */
    fn update(&self);
    fn set_period(&mut self, u64);
    
}

pub enum AnyProcess {
    BasicProcess(BasicProcess),
    BasicProcess2(BasicProcess2),
}

#[derive(Clone)]
pub enum status_type { 
	UNINITIALIZED = 0, 
	STOPPED, 
	RUNNING, 
}

#[derive(Clone)]
pub struct BasicProcess {
    pub _name: String,
	pub _status : status_type,
    pub period : u64,
   // pub _manager_ptr : manager::Manager,
}

impl BasicProcess {
    /// Creates a new Process instance
    pub fn new(name: String) -> BasicProcess {
        BasicProcess {
            _name : name,
            _status : status_type::UNINITIALIZED,
            period : 0,
        }
        
    }
    //pub fn get(&self) -> Self {}
}

impl Process for BasicProcess {
    fn update(&self) {
        println!("BasicProcess update")
    }

    fn set_period(&mut self, ms_time : u64) {
        self.period = ms_time;
    }

    fn period(ms_time : u64, p : &Process) {
        p.
    }
    
}

#[derive(Clone)]
pub struct BasicProcess2 {
    pub _name: String,
	pub _status : status_type,
    pub period : u64,
   // pub _manager_ptr : manager::Manager,
}

impl BasicProcess2 {
    /// Creates a new Process instance
    pub fn new(name: String) -> BasicProcess2 {
        BasicProcess2 {
            _name : name,
            _status : status_type::UNINITIALIZED,
            period : 0,
        }
    }

}

impl Process for BasicProcess2 {
    fn update(&self) {
        println!("BasicProcess2 update")
    }

    fn set_period(&mut self, ms_time : u64) {
        self.period = ms_time;
    }
}

impl Process for AnyProcess {
    fn update(&self) {
        match *self {
            AnyProcess::BasicProcess(ref b1) => b1.update(),
            AnyProcess::BasicProcess2(ref b2) => b2.update(),
        }
    }

    fn set_period(&mut self, ms_time : u64) {
        //self.period = ms_time;
    }

    /*fn set_period(&self) {
        match *self {
            AnyProcess::BasicProcess(ref b1) => b1.set_period();
            AnyProcess::BasicProcess2(ref b12) => b2.set_period();
        }
    }*/
}
