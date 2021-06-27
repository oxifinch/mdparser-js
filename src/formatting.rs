pub trait MarkdownFormat {
    fn add_line_breaks(self) -> String;
    fn emphasize_text(self) -> String;
    fn remove_empty_elements(self) -> String;
    fn convert_tags(self) -> String;
}

impl MarkdownFormat for String {
    fn add_line_breaks(self) -> String {
        match &self.find("  ") {
            Some(_) => {
                String::from(&self.replace("  ", "<br>"))
            },
            None => {
                return self
            }
        }
    }
    // TODO: Does this cause an unnecessary amount of heap allocations? Is the
    // formatted_string actually being modified here, and if not, is there a
    // less costly implementation?
    fn emphasize_text(self) -> String {
        let mut formatted_string: String = String::from(&self);
        loop {
            match &formatted_string.find("`") {
                Some(_) => {
                    formatted_string = String::from(formatted_string
                            .replacen("`", "<span>", 1)
                            .replacen("`", "</span>", 1));
                },
                None => {
                    break;
                }
            }
        }

        loop {
            match &formatted_string.find("***") {
                Some(_) => {
                    formatted_string = String::from(formatted_string
                        .replacen("***", "<strong><em>", 1)
                        .replacen("***", "</em></strong>", 1));
                },
                None => {
                    break;
                }
            }
        }

        loop {
            match &formatted_string.find("**") {
                Some(_) => {
                    formatted_string = String::from(formatted_string
                        .replacen("**", "<strong>", 1)
                        .replacen("**", "</strong>", 1));
                },
                None => {
                    break;
                }
            }
        }

        loop {
            match &formatted_string.find("*") {
                Some(_) => {
                    formatted_string = String::from(formatted_string
                        .replacen("*", "<em>", 1)
                        .replacen("*", "</em>", 1));
                },
                None => {
                    break;
                }
            }
        }
        formatted_string
    }

    fn remove_empty_elements(self) -> String {
        String::from(&self
        .replace("<p></p>", "")
        .replace("<ul></ul>", "")
        .replace("<ol></ol>", "")
        .replace("<blockquote></blockquote>", "")
        .replace("<div></div>", "")
        .replace("<em><strong></strong><em>", "")
        .replace("<em></em>", "")
        .replace("<strong></strong>", "")
        .replace("<span></span>", ""))
    }

    fn convert_tags(self) -> String {
        let mut formatted_string: String = String::from(&self);
        loop {
            match &formatted_string.find("{{") {
                Some(_) => {
                    match &formatted_string.find("}}") {
                        Some(_) => {
                            formatted_string = String::from(formatted_string
                                .replacen("{{", "<span class=\"chapter_meta_tag\">", 1)
                                .replacen("}}", "</span>", 1));
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
        formatted_string
    }
}
