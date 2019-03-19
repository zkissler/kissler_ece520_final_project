//! Contains the 'State' structure and implementation

use event;

/// The 'State' structure contains all the data required to create a State for a State Machine
#[derive(Clone)]
pub struct State {
    /// The name of the State
    _name : String,
    /// The id of the State
    _id : usize,
    /// Stores the id count
    _id_counter : usize,   // don't know how static works with rust
   // _state_machine_ptr : StateMachine *,
}

impl State {
    /// Creates a new State instance
    pub fn new(name:String) -> State {
        State {
            _name : name,
            _id : 0,
            _id_counter : 0,
        }
    }

    /// Emits the passed in Event
    pub fn emit(e : event::Event) {

    }

    /// Returns the name of the State
    pub fn name(self) -> String {
        return self._name;
    }

    /// The id of the State
    pub fn id(&self) -> usize {
        return self._id;
    }
}