#![allow (unused_parens)]

use std::sync::Arc;

#[derive (Clone)]
#[derive (Debug)]
#[derive (PartialEq)]
pub enum Dict <T> {
    Null,
    Cons {
        name: String,
        entry: T,
        next: Arc <Dict <T>>,
    }
}

impl <T> Dict <T> {
    pub fn new () -> Self {
        Dict::Null
    }
}

impl <T> Dict <T>
where T : Clone {
    pub fn extend (&self, name: &str, entry: T) -> Self {
        Dict::Cons {
            name: name.to_string (),
            entry,
            next: Arc::new (self.clone ()),
        }
    }
}

impl <T> Dict <T> {
    pub fn unit (name: &str, entry: T) -> Self {
        Dict::Cons {
            name: name.to_string (),
            entry,
            next: Arc::new (Dict::new ()),
        }
    }
}

impl <T> Dict <T> {
    pub fn find (&self, name: &str) -> Option <&T> {
        match self {
            Dict::Null => None,
            Dict::Cons {
                name: head_name,
                entry: head_entry,
                next,
            } => {
                if head_name == name {
                    Some (head_entry)
                } else {
                    next.find (name)
                }
            }
        }
    }
}

impl <T> Dict <T>
where T : Clone {
    pub fn insert (&self, name: &str, entry: T) -> Self {
        match self {
            Dict::Null => {
                Dict::unit (name, entry)
            }
            Dict::Cons {
                name: head_name,
                entry: head_entry,
                next,
            } => {
                if head_name == name {
                    next.extend (head_name, entry)
                } else {
                    let rest = next.insert (name, entry);
                    rest.extend (head_name, head_entry.clone ())
                }
            }
        }
    }
}
