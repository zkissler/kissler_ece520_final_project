use process;

//#[derive(Clone)]
pub struct BasicProcess2 {
    _name : String,
    pub _process : process::Process,
}

impl BasicProcess2 {
    pub fn new(name : &String) -> BasicProcess2{
        BasicProcess2 {
            _name : name.to_string(),
            _process :  process::Process::new(name.to_string(), process::process_type::BASIC2),
        }
    }
}
/*
impl process::process_trait for BasicProcess2 {
    fn update(&self) {
        println!("2: {} ", self._process.name().to_string());//, _process.milli_time());
    }  
}*/