//put use statements here
use std::time::Instant;

pub struct Manager {
/* convert to rust types */
    vector<Process *> _processes; 
    map<string, Channel *> _channels; 
    map<string, vector<std::function<void(Event&)>>> event_handlers; 
    _start_time : Instant; 
    _elapsed : Instant; 
}
impl Manager {
    pub fn manager() {
    //constructor
    }

    pub fn schedule(Process& process, period : Instant) -> Manager& {
        //need manager reference
        process._period = period;
        _processes.push_back(&process);
        process._manager_ptr = this; 
        return *this;
    }

    pub fn add_channel(Channel& channel) -> Manager& {
        _channels[channel.name()] = &channel;
        return *this;
    }

    pub fn channel(str channel) -> Channel& {
        if ( _channels.find(name) != _channels.end() ) { 
            return *(_channels[name]); 
        } else { 
            throw Exception("Tried to access an unregistered or non-existant channel."); 
        } 

    }

    /*pub fn watch(str event_name, std::function<void(Event&)> handler) -> Manager& {
        event_handlers[event_name].push_back(handler);
        return *this;
    }*/

    pub fn emit(const Event& event) -> Manager& {

    }

    pub fn all(std::function< void(Process&) > f) -> Manager& {

    }

    pub fn init() -> Manager& {

    }

    pub fn start() -> Manager& {

    }

    pub fn update() -> Manager& {

    }


    pub fn stop() -> Manager& {

    }

    pub fn start_time(&self) -> Instant {
    return self._start_time;
    }

    pub fn elapsed(&self) -> Instant {
    return self._elapsed;
    }

    pub fn run(runtime : Instant) -> Manager& {

    }
}