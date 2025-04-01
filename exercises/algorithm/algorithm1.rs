/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
 

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
}

impl<T: Clone> LinkedList<T> {
    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self 
    where
        T: PartialOrd,
    {
        let mut merged_list = LinkedList::new();
        
        unsafe {
            let mut a_ptr = list_a.start;
            let mut b_ptr = list_b.start;
            
            while a_ptr.is_some() && b_ptr.is_some() {
                let a_node = a_ptr.unwrap().as_ptr();
                let b_node = b_ptr.unwrap().as_ptr();
                
                if (*a_node).val <= (*b_node).val {
                    merged_list.add((*a_node).val.clone());
                    a_ptr = (*a_node).next;
                } else {
                    merged_list.add((*b_node).val.clone());
                    b_ptr = (*b_node).next;
                }
            }
            
            while let Some(node) = a_ptr {
                merged_list.add((*node.as_ptr()).val.clone());
                a_ptr = (*node.as_ptr()).next;
            }
            
            while let Some(node) = b_ptr {
                merged_list.add((*node.as_ptr()).val.clone());
                b_ptr = (*node.as_ptr()).next;
            }
        }
        
        merged_list
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

 