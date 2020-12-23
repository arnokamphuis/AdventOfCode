use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

// Types
pub type LLNodeRef = Rc<RefCell<LLNode>>;
pub type LLNodeOption = Option<LLNodeRef>;

#[derive(PartialEq, Debug)]
pub struct LLNode {
    pub data: usize,
    pub next: LLNodeOption,
    pub prev: LLNodeOption
}

impl LLNode {
    pub fn new(value: usize) -> LLNodeRef {
        Rc::new(RefCell::new(LLNode {
            data: value,
            next: None,
            prev: None,
        }))
    }
}

// LLNode iterator
pub struct ListNodeIterator {
    current: LLNodeOption
}

impl ListNodeIterator {
    pub fn new(start_at: LLNodeOption) -> Self {
        ListNodeIterator {
            current: start_at,
        }                
    }
}

impl Iterator for ListNodeIterator {
    type Item = LLNodeRef;

    fn next(&mut self) -> LLNodeOption {
        let current = &self.current;
        let mut result = None;

        self.current = match current {
            Some(ref current) => {
                result = Some(Rc::clone(current));
                match &current.borrow().next {
                    Some(next_node) => {
                        Some(Rc::clone(next_node))
                    },
                    _ => None
                }
            },
            _ => None
        };

        result
    }
}

#[derive(PartialEq, Debug)]
pub struct LinkedList {
    start: usize,
    nodes: HashMap<usize, LLNodeRef>,
}

impl LinkedList {

    pub fn new(value: usize) -> Self {
        let new_node = LLNode::new(value);
        new_node.borrow_mut().next = Some(Rc::clone(&new_node));
        new_node.borrow_mut().prev = Some(Rc::clone(&new_node));
        let mut map: HashMap<usize, LLNodeRef> = HashMap::new();
        map.insert( value, new_node);

        LinkedList {
            start: value,
            nodes: map,
        }
    }

    pub fn append(&mut self, value: usize, after: usize) {
        if self.nodes.contains_key(&after) {
            let new_node = LLNode::new(value);

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

    pub fn iter_node(&self, value: usize) -> ListNodeIterator {
        if self.nodes.contains_key(&value) {
            let node = self.nodes.get(&value).unwrap();
            return ListNodeIterator::new(Some(Rc::clone(node)));
        }
        ListNodeIterator::new(None)
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn start(&self) -> usize {
        self.start
    }
}