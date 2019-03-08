//put channel class here
use serde::{Serialize, Deserialize};
use std::collections::VecDeque;

pub struct Channel {
    _name : String,
    _capacity : usize,
   // _queue : VecDeque<json>,
   _queue : VecDeque<f32>, //until I can figure out the json thing
}

impl Channel {

    pub fn Channel(&mut self, name : String, capacity : usize) {
        self._name = name;
        self._capacity = capacity;
        self._queue = VecDeque::with_capacity(capacity);
    }

    pub fn send(&mut self, value : f32) {
        self._queue.push_front(value);
        while ( self._queue.len() > self.capacity() ) {
            self._queue.pop_back();
        }
    }

    /*
    pub fn send(&mut self, value : f32) -> Channel {
        self._queue.push_front(value);
        while ( self._queue.len() > self.capacity() ) {
            self._queue.pop_back();
        }
        return *self;
    }*/

 /*   pub fn flush(&mut self) -> Channel {
        self._queue.clear();
        return *self;
    }
*/
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