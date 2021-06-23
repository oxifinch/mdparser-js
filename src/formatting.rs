pub fn add_line_breaks(line: &str) -> String {
    let line = line.trim_start().replace("  ", "<br>");
    String::from(line)
}

pub fn emphasize_text(line: &str) -> String {
    let mut line = String::from(line);

    // Replace backticks with span 
    loop {
        match line.find("`") {
            Some(_) => {
                line = line
                    .replacen("`", "<span>", 1)
                    .replacen("`", "</span>", 1);
            },
            None => {
                break;
            }
        }
    }

    // First loop replaces triple-asterisks (bold italic)
    loop {
        match line.find("***") {
            Some(_) => {
                line = line
                    .replacen("***", "<strong><em>", 1)
                    .replacen("***", "</em></strong>", 1);
            },
            None => {
                break;
            }
        }
    }

    // Second loop replaces double-asterisks (bold)
    loop {
        match line.find("**") {
            Some(_) => {
                line = line
                    .replacen("**", "<strong>", 1)
                    .replacen("**", "</strong>", 1);
            },
            None => {
                break;
            }
        }
    }

    // Lastly, the third loop replaces single-asterisks (italic)
    loop {
        match line.find("*") {
            Some(_) => {
                line = line
                    .replacen("*", "<em>", 1)
                    .replacen("*", "</em>", 1);
            },
            None => {
                break;
            }
        }
    }
    line
}

pub fn remove_empty_tags(line: &str) -> String {
    let mut line = String::from(line);

    line = line
        .replace("<p></p>", "")
        .replace("<ul></ul>", "")
        .replace("<ol></ol>", "")
        .replace("<blockquote></blockquote>", "")
        .replace("<div></div>", "")
        .replace("<em><strong></strong><em>", "")
        .replace("<em></em>", "")
        .replace("<strong></strong>", "")
        .replace("<span></span>", "");
    line
}
