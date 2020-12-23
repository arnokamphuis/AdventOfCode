use super::node::{ LLNode, LLNodeRef, LLNodeOption, ListNodeIterator };
use std::cell::RefCell;
use std::rc::Rc;

use std::collections::HashMap;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

#[derive(PartialEq, Debug)]
pub struct LinkedList {
    nodes: HashMap<usize, LLNodeRef>,
}

impl LinkedList {

    pub fn new_empty() -> Self {
        LinkedList {
            nodes: HashMap::new(),
        }
    }

    pub fn new(value: usize) -> Self {
        let new_node = LLNode::new(value);
        new_node.borrow_mut().next = Some(Rc::clone(&new_node));
        new_node.borrow_mut().prev = Some(Rc::clone(&new_node));

        LinkedList {
            nodes: map!( value => new_node ),
        }
    }

    pub fn append(&mut self, value: usize, after: usize) {
        if self.nodes.contains_key(&after) {
            let mut new_node = LLNode::new(value);

            if let Some(ref after_node) = self.get(after) {

                let next_node = Rc::clone(&after_node.borrow().next.as_ref().unwrap());
                new_node.borrow_mut().next = Some(Rc::clone(&next_node)); 
                new_node.borrow_mut().prev = Some(Rc::clone(&after_node)); 
                next_node.borrow_mut().prev = Some(Rc::clone(&new_node));
                after_node.borrow_mut().next = Some(Rc::clone(&new_node));
                self.nodes.insert(value,Rc::clone(&new_node));
            }
        }
    }

    pub fn remove(&mut self, value: usize) {
        if self.nodes.contains_key(&value) {
            if let Some(ref value_node) = self.get(value) {

                let next_node = Rc::clone(&value_node.borrow().next.as_ref().unwrap());
                let prev_node = Rc::clone(&value_node.borrow().prev.as_ref().unwrap());

                next_node.borrow_mut().prev = Some(Rc::clone(&prev_node)); 
                prev_node.borrow_mut().next = Some(Rc::clone(&next_node)); 

                self.nodes.remove(&value);
            }
        }
    }

    pub fn get(&mut self, value: usize) -> LLNodeOption {
        if self.nodes.contains_key(&value) {
            return Some(Rc::clone(self.nodes.get(&value).unwrap()));
        }
        None
    }

    pub fn print_keys(&self) {
        for (k,_) in &self.nodes {
            println!("{}", k);
        }
    }


    pub fn iter_node(&self, value: usize) -> ListNodeIterator {
        if self.nodes.contains_key(&value) {
            let node = self.nodes.get(&value).unwrap();
            return ListNodeIterator::new(Some(Rc::clone(node)));
        }
        ListNodeIterator::new(None)
    }
    
}