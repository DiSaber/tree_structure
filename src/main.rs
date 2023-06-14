mod node;

use crate::node::Node;

fn main() {
    println!("Hello, world!");
    let mut root: Node<String> = Node::new(String::from("Root")); // Creates a root node

    root.push_child(Node::new(String::from("Random data 0"))); // Insert a child
    root.push_child(Node::new(String::from("Random data 1")));

    let child: &mut Node<String> = &mut root.children[0]; // 2 children were just inserted so children[0] is safe
    child.push_child(Node::new(String::from("Random data 0")));
    child.push_child(Node::new(String::from("Random data 1")));

    let mut new_root: Node<String> = Node::new(String::from("New root"));
    new_root.push_child(child.clone());

    let mut root_copy: Node<String> = root.clone(); // Copy the root node
    root_copy.value = String::from("Cloned root");

    root_copy.children[0].value = String::from("Copy 0");
    root_copy.children[1].value = String::from("Copy 1");
}
