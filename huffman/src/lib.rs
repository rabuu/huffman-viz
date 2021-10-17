use std::collections::HashMap;
use std::hash::Hash;

type Frequency = u32;

#[derive(Debug, Clone)]
pub struct Tree<T> {
    arena: Vec<Node<T>>,
}

impl<T> Tree<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new(vec: Vec<T>) -> Tree<T> {
        // count frequency of each element and store it in a map
        let mut map: HashMap<T, Frequency> = HashMap::with_capacity(vec.len());
        for e in vec {
            *map.entry(e).or_insert(0) += 1;
        }

        // convert map into list of nodes
        let mut arena: Vec<Node<T>> = Vec::new();
        for (e, f) in map {
            arena.push(Node::Tail { val: e, freq: f });
        }

        // sort nodes in arena by frequency
        arena.sort_by_key(|node| node.get_freq());

        Tree { arena }
    }

    // do one step of the algorithm
    pub fn step(&mut self) -> Result<(), &'static str> {
        if self.is_built() {
            return Err("Step impossible, tree already completely built.");
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

    // build the entire tree
    pub fn build(&mut self) {
        while !self.is_built() {
            self.step().unwrap();
        }
    }

    // check if tree is completely built
    pub fn is_built(&self) -> bool {
        self.arena.len() < 2
    }

    // getter method for arena
    pub fn get_arena(&self) -> &[Node<T>] {
        &self.arena
    }

    // generate code table
    pub fn generate_code_table(&self) -> Result<HashMap<T, Vec<bool>>, &'static str> {

        if !self.is_built() {
            return Err("Cannot generate a code table from a tree that is not built.");
        }

        fn add_table_entry<E>(table: &mut HashMap<E, Vec<bool>>, node: &Node<E>, pre_bits: &[bool])
        where E: Eq + Hash + Clone
        {
            match node {
                Node::Tail { val, .. } => {
                    table.insert(val.clone(), pre_bits.to_vec());
                }
                Node::Link { left, right, .. } => {
                    add_table_entry( table, left, &[pre_bits, &[true]].concat());
                    add_table_entry( table, right, &[pre_bits, &[false]].concat());
                }
            };
        }

        let mut table: HashMap<T, Vec<bool>> = HashMap::new();

        if let Some(Node::Link {..}) = self.arena.get(0) {
            for node in self.arena.iter() {
                add_table_entry( &mut table, node, &[]);
            }
        } else if let Some(Node::Tail {val, ..}) = &self.arena.get(0) {
            table.insert(val.clone(), vec![true]);
        }

        Ok(table)
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
