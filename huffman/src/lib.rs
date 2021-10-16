use std::collections::HashMap;
use std::hash::Hash;

type Frequency = u32;

#[derive(Debug, Clone)]
pub struct Tree<T> {
    pub arena: Vec<Node<T>>,
}

impl<T> Tree<T>
where
    T: Eq + Hash,
{
    pub fn new(vec: Vec<T>) -> Tree<T> {
        // count frequency of each element and store it in a map
        let mut map: HashMap<T, Frequency> = HashMap::new();
        for e in vec {
            *map.entry(e).or_insert(0) += 1;
        }

        // convert map into list of nodes
        let mut arena: Vec<Node<T>> = Vec::new();
        for (e, f) in map {
            arena.push(Node::Tail { val: e, freq: f });
        }

        arena.sort_by_key(|node| node.get_freq());

        Tree { arena }
    }

    pub fn step(&mut self) -> Result<(), &'static str> {
        if self.arena.len() < 2 {
            return Err("Step impossible, not enough nodes in arena.");
        }

        // sort nodes in arena by frequency
        self.arena.sort_by_key(|node| node.get_freq());

        // combine the first two nodes
        let left = self.arena.remove(0);
        let right = self.arena.remove(0);
        self.arena.insert(
            0,
            Node::Link {
                freq: left.get_freq() + right.get_freq(),
                left: Box::new(left),
                right: Box::new(right),
            },
        );
        Ok(())
    }

    pub fn build(&mut self) {
        while self.arena.len() > 1 {
            self.step().unwrap();
        }
    }
}

impl Tree<char> {
    pub fn new_from_string(string: String) -> Self {
        Self::new(string.chars().collect())
    }
}

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

impl<T> Node<T> {
    // getter method for frequency value
    pub fn get_freq(&self) -> Frequency {
        match self {
            Self::Tail { freq, .. } => *freq,
            Self::Link { freq, .. } => *freq,
        }
    }
}
