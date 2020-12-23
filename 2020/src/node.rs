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

impl Drop for LLNode {
    fn drop(&mut self) {
        // println!("LLNode with this data -> '{}' just dropped", self.data);
    }
}

// LLNode iterator
pub struct ListNodeIterator {
    current: LLNodeOption
}

impl ListNodeIterator {
    pub fn new(start_at: LLNodeOption) -> Self {
        ListNodeIterator {
            current: start_at
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