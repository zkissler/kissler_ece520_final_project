//put state class here
use event;
pub struct State {
    _name : String,
    _id : usize,
    _id_counter : usize,   // don't know how static works with rust
   // _state_machine_ptr : StateMachine *,
}

impl State {
    pub fn State(&mut self, name:String) {
        self._name = name;
        self._id = self._id_counter;
        //self._state_machine_ptr = NULL;
    }

    pub fn emit(e : event::Event) {

    }

    pub fn name(self) -> String {
        return self._name;
    }

    pub fn id(&self) -> usize {
        return self._id;
    }
}