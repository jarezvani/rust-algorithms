struct Node {
    value: i32,
    next: Option<Box<Node>>
}

fn main() {
    println!("\nManually linking nodes to create singly-linked list and calling print_list\n");
    let node3 = Node { value: 3, next: None };
    let node2 = Node { value: 2, next: Some(Box::new(node3)) };
    let node1 = Node { value: 1, next: Some(Box::new(node2)) };

    fn print_list(mut head: Option<Box<Node>>) {
        while let Some(node) = head {
            println!("Current node: {0}", node.value);
            head = node.next;
        }
    }

    print_list(Some(Box::new(node1)));

    println!("\n\nSame as above, but using build_list fn to create list from input arr\n");

    // takes in array of signed ints to build linked list in same order as arr
    fn build_list(arr: &[i32]) -> Option<Box<Node>> {
        if arr.is_empty() {
            return None;
        }

        let mut head = Some(Box::new(Node {
            value: arr[arr.len() - 1],
            next: None,
        }));

        for i in (0..arr.len() - 1).rev() {
            head = Some(Box::new(Node {
                value: arr[i],
                next: head,
            }));
        }

        head
    }
    
    let new_node = build_list(&[1, 2, 3, 4, 5]);
    print_list(new_node);
}
