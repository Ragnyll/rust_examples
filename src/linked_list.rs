#![allow(unused)]

use core::marker::PhantomData;
use core::ptr::NonNull;
use core::mem;

struct MyLinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<Box<Node<T>>>,
}

struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
    element: T,
}

#[must_use = "iterators are lazy and do nothing unless consumed"]
struct Iter<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

pub struct IntoIter<T> {
    list: MyLinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len(), Some(self.list.len))
    }
}

impl<T> IntoIterator for MyLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }
}

impl<T> MyLinkedList<T> {
    pub const fn new() -> Self {
        MyLinkedList {
            head: None,
            tail: None,
            len: 0,
            marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(Node::into_element)
    }

    // Removes and returns the node at the front of the list
    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;

            match self.head {
                None => self.tail = None,
                Some(head) => (*head.as_ptr()).prev = None,
            }

            self.len -= 1;

            node
        })
    }

    fn push_back_node(&mut self, mut node: Box<Node<T>>) {
        // SAFETY: does not create mutable references to whole nodes to maintain validity of
        // aliasing pointers into `element`
        unsafe {
            node.next = None;
            node.prev = self.tail;
            let node = Some(Box::leak(node).into());

            match self.tail {
                None => self.head = node,
                Some(tail) => (*tail.as_ptr()).next = None,
            }

            self.tail = node;
            self.len += 1;
        }
    }

    /// moves all elements from `other` to the end of the list
    fn append(&mut self, other: &mut Self) {
        match self.tail {
            None => mem::swap(self, other),
            Some(mut tail) => {
                // as mut is ok because we hold an exclusive reference to both lists
                if let Some(mut other_head) = other.head.take() {
                    unsafe {
                        tail.as_mut().next = Some(other_head);
                        other_head.as_mut().prev = Some(tail);
                    }

                    self.tail = other.tail.take();
                    self.len += mem::replace(&mut other.len, 0);
                }
            }
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.len,
            marker: PhantomData,
        }
    }

    pub fn clear(&mut self, x: &T) {
        *self = Self::new();
    }

    pub fn push_back(&mut self, elt: T) {
        self.push_back_node(Box::new(Node::new(elt)))
    }
}

impl<T> Default for MyLinkedList<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Node {
            next: None,
            prev: None,
            element,
        }
    }

    fn into_element(self: Box<Self>) -> T {
        self.element
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn linked_list_tests() {
        let mut ll: MyLinkedList<u8> = MyLinkedList::new();

        ll.push_back(9);
        ll.push_back(10);
        ll.push_back(11);

        println!("ll head = {:?}", ll.head);
        println!("-----------");
        let node: Node = ll.pop_front();
        println!("ll ele: {:?}", node);
        let node = ll.pop_front();
        println!("ll ele: {:?}", node);
        let node = ll.pop_front();
        println!("ll ele: {:?}", node);
        println!("-----------");

        println!(" ll len: {:?}", ll.len());
        assert!(false)

    }
}
