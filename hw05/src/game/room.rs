use std::rc::Rc;
use std::cell::RefCell;
use std::mem;

use super::curio::Curio;
use super::hall::Hall;

pub struct Room {
    pub name: String,
    pub contents: Vec<Curio>,
    pub halls: Vec<Rc<Hall>>,
    pub wumpus: bool,
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.name[..].eq_ignore_ascii_case(&other.name[..])
    }
}

impl Room {
    pub fn neighbors_string(&self) -> String {
        let mut names = Vec::new();

        for hall in self.halls.iter() {
            hall.left.as_ref().map(|r| {
                if *r.borrow() != *self { names.push(r.borrow().name.clone()); }
                r
            });
            hall.right.as_ref().map(|r| {
                if *r.borrow() != *self { names.push(r.borrow().name.clone()); }
                r
            });
        }

        format!("{}", names.join(", "))
    }
}