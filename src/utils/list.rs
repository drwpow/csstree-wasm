use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct List<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub prev: Option<Rc<RefCell<Node<T>>>>,
    pub next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            data,
            prev: None,
            next: None,
        }))
    }
}

pub impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn size(&self) -> usize {
        let mut size = 0;
        let mut current = self.head.clone();
        while let Some(node) = current {
            size += 1;
            current = node.borrow().next.clone();
        }
        size
    }

    pub fn first(&self) -> Option<T>
    where
        T: Clone,
    {
        self.head.as_ref().map(|node| node.borrow().data.clone())
    }

    pub fn last(&self) -> Option<T>
    where
        T: Clone,
    {
        self.tail.as_ref().map(|node| node.borrow().data.clone())
    }

    pub fn append(&mut self, data: T) {
        let new_node = Node::new(data);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    pub fn prepend(&mut self, data: T) {
        let new_node = Node::new(data);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    pub fn remove(&mut self, node: Rc<RefCell<Node<T>>>) -> Option<T> {
        let prev = node.borrow().prev.clone();
        let next = node.borrow().next.clone();

        if let Some(prev_node) = prev {
            prev_node.borrow_mut().next = next.clone();
        } else {
            self.head = next.clone();
        }

        if let Some(next_node) = next {
            next_node.borrow_mut().prev = prev.clone();
        } else {
            self.tail = prev.clone();
        }

        Some(Rc::try_unwrap(node).ok().unwrap().into_inner().data)
    }

    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut vec = Vec::new();
        let mut current = self.head.clone();
        while let Some(node) = current {
            vec.push(node.borrow().data.clone());
            current = node.borrow().next.clone();
        }
        vec
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        let mut list = List::new();
        for data in vec {
            list.append(data);
        }
        list
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = ListIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIntoIterator { list: self }
    }
}

pub struct ListIntoIterator<T> {
    list: List<T>,
}

impl<T> Iterator for ListIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.head.take().map(|head| {
            let next = head.borrow().next.clone();
            if let Some(next_node) = next {
                next_node.borrow_mut().prev = None;
            } else {
                self.list.tail = None;
            }
            self.list.head = next;
            Rc::try_unwrap(head).ok().unwrap().into_inner().data
        })
    }
}
