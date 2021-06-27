use skalver_mdparser::tests::*;
use skalver_mdparser::parsing::*;

fn main() {
    println!("Hello, world!");
    let content = read_markdown_file("../../../testfiles/markdown_01.md");
    let chapter = parse_markdown(content);
    println!("{}", chapter);
}
