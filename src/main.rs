use huffman::Node;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tree = Node::new_from_str("Lorem ipsum dolor sit amet")?;
    println!("{:#?}", tree);
    Ok(())
}
