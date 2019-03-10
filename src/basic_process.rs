use process;

//#[derive(Clone)]
pub struct BasicProcess {
    _name : String,
    pub _process : process::Process,
}

impl BasicProcess {
    pub fn new(name : &String) -> BasicProcess{
        BasicProcess {
            _name : name.to_string(),
            _process :  process::Process::new(name.to_string(), process::process_type::BASIC),
        }
    }
}
/*
impl process::process_trait for BasicProcess {
    fn update(&self) {
        println!("{} ", self._process.name().to_string());//, _process.milli_time());
    }  
}*/
/*
impl process::process_trait for BasicProcess{
    fn update(&self) {
        println!("{} ", self._process.name().to_string());//, _process.milli_time());
    }

    fn init(&self) {}
    fn start(&self) {}
    fn stop(&self) {}     
}*/
/*
impl update for BasicProcess {
    fn update(&self) {
        println!("{} ", self._process.name().to_string());//, _process.milli_time());
    }  
}*/