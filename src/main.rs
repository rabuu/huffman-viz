use huffman::Node;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tree = Node::from_str("A")?;
    println!("{:#?}", tree);
    Ok(())
}
