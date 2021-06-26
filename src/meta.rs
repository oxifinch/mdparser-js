pub struct ChapterData {
    pub html: String,
    tags: Vec<String>,
}

impl ChapterData {
    pub fn new() -> ChapterData {
        let html = String::from("");
        let tags: Vec<String> = Vec::new();
        ChapterData {
            html,
            tags
        }
    }
    pub fn fetch_tag_from_line(&mut self, line: &str) {
        // loop {
        //     find "{{" and "}}" 
        //     match => 
        //        1. create a copy of line
        //        2. delete until and including {{ 
        //        3. delete from and including }}
        //        4. Make a string of the result and add it to self
        //     
        //     no more matches => break
        // }

        //println!("[ DEBUG ] Running fetch_tag_from_line.");
        let mut current_line = String::from(line);
        loop {
            match &current_line.find("{{") {
                Some(lb) => {
                    match &current_line.find("}}") {
                        Some(rb) => {
                            println!("[ DEBUG ] Current line before: '{}'", current_line);
                            let l: usize = lb.to_owned();
                            let r: usize = rb.to_owned() + 2;
                            let full_tag = &line[l..r];
                            let tag = &full_tag[2..full_tag.len() - 2];
                            self.tags.push(String::from(tag));
                            current_line = String::from(
                                &current_line.replace(&full_tag, ""));
                            println!("[ DEBUG ] Current line after: '{}'", current_line);
                            println!("");
                            break;
                        },
                        None => {
                            break;
                        }
                    }
                },
                None => {
                    break;
                }
            }
        }
    }

    pub fn print_info(&self) {
        println!("[ HTML CONTENT ] {}", &self.html);
        println!("[ TAGS ]");
        for tag in &self.tags {
            println!("{}", tag);
        }
    }
}
