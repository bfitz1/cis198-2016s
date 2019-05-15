use std::cell::RefCell;
use std::rc::Rc;

use super::room::Room;

pub struct Hall {
    pub left: Option<Rc<RefCell<Room>>>,
    pub right: Option<Rc<RefCell<Room>>>,
}

impl Hall {
    pub fn new() -> Hall {
        Hall { left: None, right: None }
    }

    /// Given a Room `room`, find the room at the other end of Hall `self`.
    pub fn other(&self, room: &Room) -> Option<Rc<RefCell<Room>>> {
        if let Some(left) = self.left.as_ref().filter(|r| *r.borrow() == *room) {
            Some(Rc::clone(left))
        } else if let Some(right) = self.right.as_ref().filter(|r| *r.borrow() == *room) {
            Some(Rc::clone(right))
        } else {
            None
        }
    }
}