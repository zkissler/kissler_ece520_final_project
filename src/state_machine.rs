//! Contains the 'State Machine' structure and implementation

use state;
use transition;

/// The 'State Machine' structure contains all the data required to create an Elma State Machine
pub struct StateMachine {
    /// Stores all the Transitions for this State Machine
    _transitions : Vec<transition::Transition>,
    /// Stores the initial State of the State Machine
    _initial : state::State,
    /// Stores the current State of the State Machine
    _current : state::State,

}

impl StateMachine {
    /// Creates a new State Machine instance
    pub fn new(&mut self, _event_name : String, from : state::State, to : state::State) -> StateMachine {
        StateMachine {
            _transitions : Vec::with_capacity(15),
            _initial : from,
            _current : to,
        }
    }

    pub fn current(&self) -> State {
        return self._current;
    }

    pub fn set_initial(&mut self, initial : state::State) {
        self._initial = initial;
    }

    pub fn add_transition(&mut self, _event_name : String, from : state::State, to : state::State) {
        let t = transition::Transition::new(_event_name, from, to);
        self._transitions.push(t);
        //set to & from state machine pointers
    }

}