
//this might need to be fixed
use state;
pub struct Transition {
    _from : state::State,
    _to : state::State,
    _event_name : String,
}

impl Transition {
    pub fn Transition(&mut self, _event_name : String, _from : state::State, _to : state::State) {
        self._event_name = _event_name;
        self._from = _from;
        self._to = _to;
    }

    pub fn from(self) -> state::State {
      return self._from;
    }

    pub fn to(self) -> state::State {
      return self._to;
    }

    pub fn event_name(self) -> String {
        return self._event_name;
    }
}