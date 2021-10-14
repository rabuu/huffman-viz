use huffman::Tree;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tree = Tree::new_from_string(String::from("lorem ipsum"));
    tree.build();
    println!("{:#?}", tree);
    Ok(())
}
