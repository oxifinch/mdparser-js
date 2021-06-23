pub struct ChapterMetaData {
    link_name: String,
    tags: Vec<String>
}

impl ChapterMetaData {
    pub fn new(chapter_name: &str) -> ChapterMetaData {
        let link_name: String = chapter_name
            .trim()
            .to_lowercase()
            .replace(" ", "_");
        println!("[ DEBUG ] Generated link name: {}", link_name);
        let tags: Vec<String> = Vec::new();

        ChapterMetaData {
            link_name,
            tags
        }
    }

    pub fn get_tag_from_line(&mut self, line: &str) {
        let line = line.clone();
        let tag_start = match line.find("{{") {
            Some(i) => i,
            None => {
                return;
            }
        };
        let tag_end = match line.find("}}") {
            Some(i) => i,
            None => {
                return;
            }
        };
    }
}
