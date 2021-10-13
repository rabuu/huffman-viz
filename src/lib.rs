use std::collections::HashMap;
use std::hash::Hash;

type Frequency = u32;

pub enum Node<T> {
    Tail {
        val: T,
        freq: Frequency,
    },
    Link {
        freq: Frequency,
        left: Box<Node<T>>,
        right: Box<Node<T>>,
    },
}

impl<T: Copy + Eq + Hash> Node<T> {
    pub fn from_vec(vec: &Vec<T>) -> Node<T> {

        // create map & fill it with all the given elements and their frequency
        let mut map: HashMap<T, Frequency> = HashMap::new();
        for e in vec {
            *map.entry(*e).or_insert(0) += 1;
        }

        Node::Tail {
            val: vec[0],
            freq: 0,
        }
    }
}

// example how a huffman tree could look like
fn _foo() {
    let _ = Node::Link {
        freq: 5,
        left: Box::new(Node::Link {
            freq: 3,
            left: Box::new(Node::Tail { val: 'O', freq: 1 }),
            right: Box::new(Node::Tail { val: 'P', freq: 2 }),
        }),
        right: Box::new(Node::Tail { val: 'R', freq: 2 }),
    };
}
