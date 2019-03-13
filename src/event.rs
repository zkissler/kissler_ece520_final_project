
pub struct Event {
    //json value
    _propagate: bool,
    _empty: bool,
    _name: String,
}


impl Event {

    //! Construct a new event
    //! \param value A json object
    pub fn new(name: String) -> Event {
        Event {
            _propagate : true,
            _empty : false,
            _name : name,
            //init json value 
        }
    }

 /*   pub fn value(&self) -> json {
        return self._value;
    }*/

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