//! Contains the 'Event' structure and implementation

/// The 'Event' structure contains all the data required to create an Event for an Elma process
#[derive(Clone)]
pub struct Event {
    //json value
    /// Boolean value that stores whether or not the Event should be propagated
    pub _propagate: bool,
    /// Boolean value that stores whether the Event is empty
    pub _empty: bool,
    /// Stores the name of the Event
    pub _name: String,
}

impl Event {
    /// Creates a new Event instance
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
    
    /// Returns the state of the Event empty member variable
    pub fn empty(&self) -> bool {
        return self._empty;
    }

    /// Returns the name of the Event
    pub fn name(self) -> String {
        return self._name;
    }
    
    /// Returns the propagate Event member variable
    pub fn propagate(&self) -> bool {
        return self._propagate;
    }
    
    /// Resets the Event
    pub fn reset(&mut self) {
        self._propagate = true;
    }

}