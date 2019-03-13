use process;
use manager;

pub struct Cruise {
    _name : String,
    _desired_speed : usize,
    pub _process : process::Process,
}

impl Cruise {
    pub fn new(name : &String, m : manager::Manager) -> Cruise {
        Cruise {
            _name : name.to_string(),
            _desired_speed : 0,
            _process :  process::Process::new(name.to_string(), process::process_type::CRUISE, m),
        }
    }
}
