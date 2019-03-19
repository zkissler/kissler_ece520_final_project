//! Contains the 'Transition' structure and implementation

use state;

/// The 'Transition' structure contains all the data required to provide a transition between States in a State Machine
pub struct Transition {
    /// The previous State
    _from : state::State,
    /// The next State
    _to : state::State,
    /// The name of the Transition
    _name : String,
}

impl Transition {
    /// Creates a new Transition instance
    pub fn new(&mut self, name : String, _from : state::State, _to : state::State) -> Transition {
        Transition {
            _name = name,
            _from = _from,
            _to = _to,
        }
    }

    /// Returns the previous State
    pub fn from(self) -> state::State {
      return self._from;
    }

    /// Returns the next State
    pub fn to(self) -> state::State {
      return self._to;
    }

    /// Returns the name of the Transition
    pub fn name(self) -> String {
        return self._name;
    }
}