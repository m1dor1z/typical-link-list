#[derive(Debug)]
pub struct List<T: std::fmt::Debug> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    payload: T,
    next: Link<T>
}

impl<T> Node<T> {
    fn new(next: Link<T>, value: T) -> Node<T> {
        Node { payload: value, next }
    }
}

impl<T> List<T> 
where
    T: std::fmt::Debug
{
    fn new() -> Self {
        List { head: None }
    }

    fn push(&mut self, value: T) {
        let new_node = Node::new(self.head.take(), value);
        self.head = Some(Box::new(new_node));
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.payload
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().take().map(|node| {
            &node.payload
        })
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().take().map(|node| {
            &mut node.payload
        })
    }

    fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }

    fn into_iter(self) -> IntoIter<T> {
       IntoIter(self)
    }
}

impl<T> Drop for List<T>
where
    T: std::fmt::Debug
{
    fn drop(&mut self) {
        let mut curr = self.head.take();
        while let Some(mut boxed_node) = curr {
            curr = boxed_node.next.take();
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
            &node.payload
        })
    }
}

pub struct IntoIter<T: std::fmt::Debug>(List<T>);

impl<T> Iterator for IntoIter<T>
where
    T: std::fmt::Debug
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        list.push(10);
        list.push(20);
        list.push(30);
        list.push(40);

        println!("Printed: {:#?}", list);

        for (index, e) in list.into_iter().enumerate() {
            println!("Node: {} -> {:?}", index + 1, e);
        }
    }
}
