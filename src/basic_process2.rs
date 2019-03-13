use process;
use manager;

pub struct BasicProcess2 {
    _name : String,
    pub _process : process::Process,
}

impl BasicProcess2 {
    pub fn new(name : &String, m : manager::Manager) -> BasicProcess2{
        BasicProcess2 {
            _name : name.to_string(),
            _process :  process::Process::new(name.to_string(), process::process_type::BASIC2, m),
        }
    }
}
