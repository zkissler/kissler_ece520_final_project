//put channel class here
use serde::{Serialize, Deserialize};
use std::collections::VecDeque;

pub struct Channel {
    _name : String,
    _capacity : usize,
    _queue : VecDeque<f32>, //until I can figure out json
}

impl Channel {

    pub fn new(&mut self, name : String, capacity : usize) -> Channel {
        Channel {
            _name = name,
            _capacity = capacity,
            _queue = VecDeque::with_capacity(capacity),
        }
    }

    pub fn send(&mut self, value : f32) {
        self._queue.push_front(value);
        while ( self._queue.len() > self.capacity() ) {
            self._queue.pop_back();
        }
    }

    pub fn flush(&mut self) {
        self._queue.clear();
    }

    //change to json if possible
    /*pub fn latest(&self) -> f32 {
        //throw exception if queue size is 0
        let i : f32 = self._queue.front() as f32;
        return i;
    }

    //change to json if possible
    pub fn earliest(&self) -> f32 {
        //throw exception if queue size is 0
        return self._queue.back();
    }*/

    pub fn size(&self) -> usize {
        return self._queue.len();
    }

    pub fn empty(&self) -> bool {
        return self._queue.is_empty();
    }

    pub fn nonempty(&self) -> bool {
        return !self._queue.is_empty();
    }

    pub fn name(self) -> String {
        return self._name;
    }

    pub fn capacity(&self) -> usize {
        return self._capacity;
    }


}