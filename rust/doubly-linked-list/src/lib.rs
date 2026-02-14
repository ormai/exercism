//! A double-ended linked list implemented with raw pointers and unsafe.

use std::ptr;

mod pre_implemented;

/// A node in the list
struct Node<T> {
    /// Data owned by the node
    data: T,

    /// Link to the next node.
    next: *mut Node<T>,

    /// Link to the previous node.
    prev: *mut Node<T>,
}

/// Double-ended linked list.
pub struct LinkedList<T> {
    /// Points to the front of the list.
    /// If the list is empty it will be `null` and will be equal to `tail`.
    /// If the list has only one element `head` will be equal to `tail`.
    ///
    /// `head.prev` is always `null`.
    head: *mut Node<T>,

    // Points to the back of the list.
    /// If the list is empty it will be `null` and will be equal to `tail`.
    /// If the list has only one element `tail` will be equal to `head`.
    ///
    /// `tail.next` is always `null`.
    tail: *mut Node<T>,
}

/// Cursor for traversing and mutating the list.
pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,

    /// `Node` at which the cursor points to.
    current: *mut Node<T>,
}

pub struct Iter<'a, T> {
    list: &'a LinkedList<T>,
    current: *const Node<T>,
}

impl<'a, T> LinkedList<T> {
    /// Creates an empty `LinkedList<T>`.
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_null() || self.tail.is_null()
    }

    /// Counts the elements in the list by traversing the the list from front to back.
    pub fn len(&self) -> usize {
        let mut cur = self.head;
        let mut len = 0;
        while !cur.is_null() {
            cur = unsafe { (*cur).next };
            len += 1;
        }
        len
    }

    /// Returns a `Cursor` positioned on the front element
    pub fn cursor_front(&'a mut self) -> Cursor<'a, T> {
        let head = self.head;
        Cursor {
            list: self,
            current: head,
        }
    }

    /// Returns a `Cursor` positioned on the back element
    pub fn cursor_back(&'a mut self) -> Cursor<'a, T> {
        let tail = self.tail;
        Cursor {
            list: self,
            current: tail,
        }
    }

    /// Return an `Iter` that moves from front to back
    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter {
            list: self,
            current: ptr::null(),
        }
    }
}

impl<T> Cursor<'_, T> {
    /// Returns a mutable references to the current element.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.current.is_null() {
            None
        } else {
            Some(unsafe { &mut (*self.current).data })
        }
    }

    /// Moves the cursor to the next element closer to the back and returns a references to its data.
    pub fn next(&mut self) -> Option<&mut T> {
        if self.current.is_null() {
            return None;
        }
        unsafe {
            let next = (*self.current).next;
            self.current = next;
            if next.is_null() {
                None
            } else {
                Some(&mut (*next).data)
            }
        }
    }

    /// Moves the cursor to the next element closer to the front and returns a references to its data.
    pub fn prev(&mut self) -> Option<&mut T> {
        if self.current.is_null() {
            return None;
        }
        unsafe {
            let prev = (*self.current).prev;
            self.current = prev;
            if prev.is_null() {
                None
            } else {
                Some(&mut (*prev).data)
            }
        }
    }

    /// Removes and returns (moves out of the list) the element at the current position and moves the cursor:
    /// - to the front if the current element is the tail
    /// - to nowhere if the element is the only one
    /// - to the back othewise
    pub fn take(&mut self) -> Option<T> {
        if self.current.is_null() {
            None
        } else {
            let current = unsafe { Box::from_raw(self.current) };

            if self.list.head == self.list.tail {
                assert_eq!(self.list.head, self.current);
                assert_eq!(self.list.tail, self.current);
                self.list.head = ptr::null_mut();
                self.list.tail = ptr::null_mut();
                self.current = ptr::null_mut();
            } else if self.list.tail == self.current {
                assert!(current.next.is_null());
                assert!(!current.prev.is_null());
                // Pop the back and move toward the front
                unsafe {
                    (*(*self.current).prev).next = ptr::null_mut();
                }
                self.list.tail = current.prev;
                self.current = current.prev;
            } else if self.list.head == self.current {
                assert!(current.prev.is_null());
                assert!(!current.next.is_null());
                // Pop the front and move toward the back
                unsafe {
                    (*(*self.current).next).prev = ptr::null_mut();
                }
                self.list.head = current.next;
                self.current = current.next;
            } else {
                // Pop the current and move toward the back
                assert!(!current.next.is_null());
                assert!(!current.prev.is_null());

                unsafe {
                    (*(*self.current).prev).next = current.next;
                    (*(*self.current).next).prev = current.prev;
                };

                self.current = current.next;
            }

            Some(current.data)
        }
    }

    /// Inserts [data] after the current position.
    pub fn insert_after(&mut self, data: T) {
        if self.list.is_empty() {
            self.insert_first(data);
        } else {
            let new = Box::into_raw(Box::new(Node {
                data,
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            }));
            unsafe {
                if (*self.current).next.is_null() {
                    // Push back
                    self.list.tail = new;
                } else {
                    // Insert after in the middle of two existing nodes
                    (*(*self.current).next).prev = new;
                    (*new).next = (*self.current).next;
                }
                (*self.current).next = new;
                (*new).prev = self.current;
            }
        }
    }

    /// Inserts [data] before the current position.
    pub fn insert_before(&mut self, data: T) {
        if self.list.is_empty() {
            self.insert_first(data);
        } else {
            let new = Box::into_raw(Box::new(Node {
                data,
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            }));
            unsafe {
                if (*self.current).prev.is_null() {
                    // Push front
                    self.list.head = new;
                } else {
                    // Insert before in the middle of two existing nodes
                    (*(*self.current).prev).next = new;
                    (*new).prev = (*self.current).prev;
                }
                (*self.current).prev = new;
                (*new).next = self.current;
            }
        }
    }

    /// Inserts the first element in an empty list.
    ///
    /// Panics if the list is not empty;
    fn insert_first(&mut self, data: T) {
        assert!(self.list.head.is_null());
        assert!(self.list.tail.is_null());
        assert!(self.current.is_null());
        let new = Box::into_raw(Box::new(Node {
            data,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        }));
        self.current = new;
        self.list.head = new;
        self.list.tail = new;
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.current.is_null() {
            if self.list.head.is_null() {
                None
            } else {
                self.current = self.list.head;
                unsafe { Some(&(*self.current).data) }
            }
        } else {
            unsafe {
                if (*self.current).next.is_null() {
                    None
                } else {
                    self.current = (*self.current).next;
                    Some(&(*self.current).data)
                }
            }
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head;
        while !current.is_null() {
            let ptr = unsafe { Box::from_raw(current) }; // dropped after scope ends
            current = ptr.next;
        }
    }
}
