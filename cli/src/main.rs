use clap::Parser;
use huffman::{Node, Tree};

#[derive(Debug, Parser)]
struct Cli {
    s: String,
}

#[derive(Debug)]
enum Location {
    Root,
    Left,
    Right,
}

fn main() {
    let cli = Cli::parse();
    let mut tree = Tree::new_from_string(cli.s);
    tree.build();

    for node in tree.get_arena() {
        print_subtree(node, Location::Root);
    }
}

fn print_subtree(node: &Node<char>, loc: Location) {
    match node {
        Node::Tail { val, freq: _ } => match loc {
            Location::Root => println!("[{val}]"),
            Location::Left => println!("[{val},edge label={{node[midway,left] {{0}}}}]"),
            Location::Right => println!("[{val},edge label={{node[midway,right] {{1}}}}]"),
        },
        Node::Link {
            freq: _,
            left,
            right,
        } => {
            match loc {
                Location::Root => println!("[{{}}"),
                Location::Left => println!("[{{}},edge label={{node[midway,left] {{0}}}}"),
                Location::Right => println!("[{{}},edge label={{node[midway,right] {{1}}}}"),
            }

            print_subtree(left, Location::Left);
            print_subtree(right, Location::Right);

            println!("]")
        }
    }
}
