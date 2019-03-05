
pub struct Event {
    //can you do json? or use previous method
    _propagate: bool,
    _empty: bool,
    _name: String,
}


impl Event {

    pub fn Event(&mut self, name : String) {
        self._name = name;
    }

    pub fn empty(&self) -> bool {
        return self._empty;
    }

    pub fn name(self) -> String {
        return self._name;
    }

    pub fn propagate(&self) -> bool {
        return self._propagate;
    }

    pub fn reset(&mut self) {
        self._propagate = true;
    }

}