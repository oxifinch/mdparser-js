pub mod parsing;
pub mod formatting;
pub mod meta;
pub mod tests;

#[test]
fn test_markdown() {
    let content = tests::read_markdown_file("./testfiles/markdown_01.md");
    let chapter = parsing::parse_markdown(content);
    chapter.print_info();
}
