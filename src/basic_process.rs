use process;
use manager;

pub struct BasicProcess {
    _name : String,
    pub _process : process::Process,
}

impl BasicProcess {
    pub fn new(name : &String, m : manager::Manager) -> BasicProcess{
        BasicProcess {
            _name : name.to_string(),
            _process :  process::Process::new(name.to_string(), process::process_type::BASIC, m),
        }
    }
}
