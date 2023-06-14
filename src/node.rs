pub struct Node<T: Clone> {
    pub value: T,
    pub children: Vec<Box<Node<T>>>,
}

impl<T: Clone> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            children: Vec::new(),
        }
    }

    pub fn push_child(&mut self, node: Node<T>) {
        self.children.push(Box::new(node));
    }
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            children: self.children.clone(),
        }
    }
}
