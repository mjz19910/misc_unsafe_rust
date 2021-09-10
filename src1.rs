type MainResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> MainResult {
    exec::main()
}

mod exec {
    use super::MainResult;

    pub enum Mode {
        Give3,
        LL,
        None,
    }

    pub fn main() -> MainResult {
        let mut mode = Mode::None;
        drop(mode);
        mode = (Mode::LL, Mode::Give3).0;
        execute_program(mode)
    }

    #[cfg(test)]
    mod test {
        use super::execute_program;
        use super::Mode;
        #[test]
        fn main_test() {
            assert!(execute_program(Mode::LL).is_ok(), "Mode::LL works");
            assert!(execute_program(Mode::Give3).is_ok(), "Mode::Give3 works");
            assert!(execute_program(Mode::None).is_ok(), "Mode::None works");
        }
    }

    fn execute_program(mode: Mode) -> MainResult {
        match mode {
            Mode::Give3 => {
                fn give_3(a_closure: &mut impl FnMut(isize) -> isize) -> isize {
                    a_closure(3)
                }
                use std::sync::{Arc, Mutex};
                let i_ = &mut 0;
                let i = Arc::new(Mutex::new(i_));
                let i_clo = Arc::clone(&i);
                let a_closure = &mut move |x: isize| {
                    let ac = Arc::clone(&i_clo);
                    let mut i = ac.lock().unwrap();
                    **i += x;
                    **i
                };
                let x = give_3(a_closure);
                println!("{}", x);
                let x = give_3(a_closure);
                println!("{}", x);
                let x = give_3(a_closure);
                println!("{}", x);
                println!("{}", i.lock().unwrap());
                Ok(())
            }
            Mode::LL => {
                use super::second::*;
                let mut x = List::new();
                x.push(42);
                x.push(14);
                x.push(34);
                println!("{} {}", x.pop().unwrap(), x.pop().unwrap());
                *x.peek_mut().unwrap() -= 42;
                println!("{:?}", x.pop().unwrap());
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

pub mod first {
    use std::mem;

    #[derive(Debug)]
    pub struct List {
        head: Link,
    }

    #[derive(Debug)]
    enum Link {
        Empty,
        More(Box<Node>),
    }

    #[derive(Debug)]
    struct Node {
        elem: i32,
        next: Link,
    }

    impl Drop for List {
        fn drop(&mut self) {
            let mut cur_link = mem::replace(&mut self.head, Link::Empty);
            // `while let` == "do this thing until this pattern doesn't match"
            while let Link::More(mut boxed_node) = cur_link {
                cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
                // boxed_node goes out of scope and gets dropped here;
                // but its Node's `next` field has been set to Link::Empty
                // so no unbounded recursion occurs.
            }
        }
    }

    impl Default for List {
        fn default() -> List {
            List::new()
        }
    }

    impl List {
        pub fn new() -> Self {
            List { head: Link::Empty }
        }

        pub fn push(&mut self, elem: i32) {
            let new_node = Box::new(Node {
                elem,
                next: mem::replace(&mut self.head, Link::Empty),
            });

            self.head = Link::More(new_node);
        }

        pub fn pop(&mut self) -> Option<i32> {
            match mem::replace(&mut self.head, Link::Empty) {
                Link::Empty => None,
                Link::More(node) => {
                    self.head = node.next;
                    Some(node.elem)
                }
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::List;
        #[test]
        fn basics() {
            let mut list = List::new();

            // Check empty list behaves right
            assert_eq!(list.pop(), None);

            // Populate list
            list.push(1);
            list.push(2);
            list.push(3);

            // Check normal removal
            assert_eq!(list.pop(), Some(3));
            assert_eq!(list.pop(), Some(2));

            // Push some more just to make sure nothing's corrupted
            list.push(4);
            list.push(5);

            // Check normal removal
            assert_eq!(list.pop(), Some(5));
            assert_eq!(list.pop(), Some(4));

            // Check exhaustion
            assert_eq!(list.pop(), Some(1));
            assert_eq!(list.pop(), None);
        }
    }
}

pub mod second {
    pub struct List<T> {
        head: Link<T>,
    }

    type Link<T> = Option<Box<Node<T>>>;

    struct Node<T> {
        elem: T,
        next: Link<T>,
    }

    impl<T> Default for List<T> {
        fn default() -> List<T> {
            List::new()
        }
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            List { head: None }
        }

        pub fn push(&mut self, elem: T) {
            let new_node = Box::new(Node {
                elem,
                next: self.head.take(),
            });

            self.head = Some(new_node);
        }

        pub fn pop(&mut self) -> Option<T> {
            self.head.take().map(|node| {
                self.head = node.next;
                node.elem
            })
        }

        pub fn peek(&self) -> Option<&T> {
            self.head.as_ref().map(|node| &node.elem)
        }

        pub fn peek_mut(&mut self) -> Option<&mut T> {
            self.head.as_mut().map(|node| &mut node.elem)
        }

        pub fn into_iter(self) -> IntoIter<T> {
            IntoIter(self)
        }

        pub fn iter(&self) -> Iter<'_, T> {
            Iter {
                next: self.head.as_deref(),
            }
        }

        pub fn iter_mut(&mut self) -> IterMut<'_, T> {
            IterMut {
                next: self.head.as_deref_mut(),
            }
        }
    }

    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            let mut cur_link = self.head.take();
            while let Some(mut boxed_node) = cur_link {
                cur_link = boxed_node.next.take();
            }
        }
    }

    pub struct IntoIter<T>(List<T>);

    impl<T> Iterator for IntoIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            // access fields of a tuple struct numerically
            self.0.pop()
        }
    }

    pub struct Iter<'a, T> {
        next: Option<&'a Node<T>>,
    }

    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            self.next.map(|node| {
                self.next = node.next.as_deref();
                &node.elem
            })
        }
    }

    pub struct IterMut<'a, T> {
        next: Option<&'a mut Node<T>>,
    }

    impl<'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<Self::Item> {
            self.next.take().map(|node| {
                self.next = node.next.as_deref_mut();
                &mut node.elem
            })
        }
    }

    #[cfg(test)]
    mod test {
        use super::List;

        #[test]
        fn basics() {
            let mut list = List::new();

            // Check empty list behaves right
            assert_eq!(list.pop(), None);

            // Populate list
            list.push(1);
            list.push(2);
            list.push(3);

            // Check normal removal
            assert_eq!(list.pop(), Some(3));
            assert_eq!(list.pop(), Some(2));

            // Push some more just to make sure nothing's corrupted
            list.push(4);
            list.push(5);

            // Check normal removal
            assert_eq!(list.pop(), Some(5));
            assert_eq!(list.pop(), Some(4));

            // Check exhaustion
            assert_eq!(list.pop(), Some(1));
            assert_eq!(list.pop(), None);
        }

        #[test]
        fn peek() {
            let mut list = List::new();
            assert_eq!(list.peek(), None);
            assert_eq!(list.peek_mut(), None);
            list.push(1);
            list.push(2);
            list.push(3);

            assert_eq!(list.peek(), Some(&3));
            assert_eq!(list.peek_mut(), Some(&mut 3));

            list.peek_mut().map(|value| *value = 42);

            assert_eq!(list.peek(), Some(&42));
            assert_eq!(list.pop(), Some(42));
        }

        #[test]
        fn into_iter() {
            let mut list = List::new();
            list.push(1);
            list.push(2);
            list.push(3);

            let mut iter = list.into_iter();
            assert_eq!(iter.next(), Some(3));
            assert_eq!(iter.next(), Some(2));
            assert_eq!(iter.next(), Some(1));
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn iter() {
            let mut list = List::new();
            list.push(1);
            list.push(2);
            list.push(3);

            let mut iter = list.iter();
            assert_eq!(iter.next(), Some(&3));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&1));
        }

        #[test]
        fn iter_mut() {
            let mut list = List::new();
            list.push(1);
            list.push(2);
            list.push(3);

            let mut iter = list.iter_mut();
            assert_eq!(iter.next(), Some(&mut 3));
            assert_eq!(iter.next(), Some(&mut 2));
            assert_eq!(iter.next(), Some(&mut 1));
        }
    }
}

pub mod third {
    use std::rc::Rc;

    pub struct List<T> {
        head: Link<T>,
    }

    type Link<T> = Option<Rc<Node<T>>>;

    struct Node<T> {
        elem: T,
        next: Link<T>,
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            List { head: None }
        }

        pub fn prepend(&self, elem: T) -> List<T> {
            List {
                head: Some(Rc::new(Node {
                    elem: elem,
                    next: self.head.clone(),
                })),
            }
        }

        pub fn tail(&self) -> List<T> {
            List {
                head: self.head.as_ref().and_then(|node| node.next.clone()),
            }
        }

        pub fn head(&self) -> Option<&T> {
            self.head.as_ref().map(|node| &node.elem)
        }

        pub fn iter(&self) -> Iter<'_, T> {
            Iter {
                next: self.head.as_deref(),
            }
        }
    }

    impl<T> Drop for List<T> {
        fn drop(&mut self) {
            let mut head = self.head.take();
            while let Some(node) = head {
                if let Ok(mut node) = Rc::try_unwrap(node) {
                    head = node.next.take();
                } else {
                    break;
                }
            }
        }
    }

    pub struct Iter<'a, T> {
        next: Option<&'a Node<T>>,
    }

    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            self.next.map(|node| {
                self.next = node.next.as_deref();
                &node.elem
            })
        }
    }

    #[cfg(test)]
    mod test {
        use super::List;

        #[test]
        fn basics() {
            let list = List::new();
            assert_eq!(list.head(), None);

            let list = list.prepend(1).prepend(2).prepend(3);
            assert_eq!(list.head(), Some(&3));

            let list = list.tail();
            assert_eq!(list.head(), Some(&2));

            let list = list.tail();
            assert_eq!(list.head(), Some(&1));

            let list = list.tail();
            assert_eq!(list.head(), None);

            // Make sure empty tail works
            let list = list.tail();
            assert_eq!(list.head(), None);
        }

        #[test]
        fn iter() {
            let list = List::new().prepend(1).prepend(2).prepend(3);

            let mut iter = list.iter();
            assert_eq!(iter.next(), Some(&3));
            assert_eq!(iter.next(), Some(&2));
            assert_eq!(iter.next(), Some(&1));
        }
    }
}

