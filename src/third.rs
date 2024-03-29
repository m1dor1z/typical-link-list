use std::rc::Rc;

#[allow(dead_code)]
#[derive(Debug)]
pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Rc<Node<T>>>;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Node<T> {
    payload: T,
    next: Link<T>
}

impl<T> Node<T> {
    fn new(payload: T, next: Link<T>) -> Self {
        Node { payload, next }
    }
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    fn prepend(&self, payload: T) -> List<T> {
        List { 
            head: Some(Rc::new(Node::new(payload, self.head.clone())))
        }
    }

    fn tail(&self) -> List<T> {
        List { 
            head: self.head.as_ref().and_then(|node| node.next.clone()) 
        }
    }

    fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.payload)
    }

    fn into_iter(&self) -> IterBorrow<'_, T> {
        IterBorrow { next: self.head.as_deref() }
    }
}

pub struct IterBorrow<'a, T> {
    next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for IterBorrow<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.payload
        })
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
}
