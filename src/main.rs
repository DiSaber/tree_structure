use crate::node::Node;

mod node;

fn main() {
    // Below are some samples

    let mut root: Node<String> = Node::new(String::from("Root")); // Creates a root node

    root.children.push(Node::new(String::from("Random data 0"))); // Insert a child
    root.children.push(Node::new(String::from("Random data 1")));

    let child: &mut Node<String> = &mut root.children[0]; // 2 children were just inserted so children[0] is safe
    child
        .children
        .push(Node::new(String::from("Random data 0")));
    child
        .children
        .push(Node::new(String::from("Random data 1")));

    let mut new_root: Node<String> = Node::new(String::from("New root")); // Create a new root node
    new_root.children.push(child.clone()); // Clone the child from the original root node

    let mut root_copy: Node<String> = root.clone(); // Clone the root node
    root_copy.value = String::from("Cloned root");

    root_copy.children[0].value = String::from("Copy 0");
    root_copy.children[1].value = String::from("Copy 1");
}
