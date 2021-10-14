use std::collections::HashMap;
use std::hash::Hash;

type Frequency = u32;

#[derive(Debug, Clone)]
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

impl<T> Node<T>
where
    T: Copy + Eq + Hash,
{
    // create a tree by converting a vector
    pub fn new_from_vec(vec: &[T]) -> Result<Node<T>, &'static str> {
        // count frequency of each element and store it in a map
        let mut map: HashMap<T, Frequency> = HashMap::new();
        for e in vec {
            *map.entry(*e).or_insert(0) += 1;
        }

        // convert map into list of nodes
        let mut list: Vec<Node<T>> = Vec::new();
        for (e, f) in map {
            list.push(Node::Tail { val: e, freq: f });
        }

        // repeat until there is only one node left
        while list.len() > 1 {
            // sort list by frequency
            list.sort_by_key(|a| a.get_freq());

            // combine the first two Nodes
            let left = list.remove(0);
            let right = list.remove(0);
            list.insert(
                0,
                Node::Link {
                    freq: left.get_freq() + right.get_freq(),
                    left: Box::new(left),
                    right: Box::new(right),
                },
            );
        }

        // return first (and only) node
        // error if no node is in list
        match list.get(0) {
            Some(node) => Ok(node.clone()),
            None => Err("Creation of Huffman tree failed! No elements were passed."),
        }
    }

    // getter method for frequency value
    fn get_freq(&self) -> Frequency {
        match self {
            Self::Tail { freq, .. } => *freq,
            Self::Link { freq, .. } => *freq,
        }
    }
}

impl Node<char> {
    // create a tree by converting a &str
    pub fn new_from_str(s: &str) -> Result<Node<char>, &'static str> {
        let char_vec: Vec<char> = s.chars().collect();
        Self::new_from_vec(&char_vec)
    }
}
