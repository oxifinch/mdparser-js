use super::formatting::MarkdownFormat;
use super::meta::ChapterContent;
use wasm_bindgen::prelude::*;

pub fn parse_chapter_content(chapter: ChapterContent) -> String {
    let serialized = match serde_json::to_string(&chapter) {
        Ok(s) => s,
        Err(_) => {
            String::from("{}")
        }
    };
    serialized
}

#[wasm_bindgen]
pub fn parse_markdown(data: String) -> String {
    let mut tokens: Vec<String> = Vec::new();
    let mut chapter_content = ChapterContent::new();

    let mut p = false;
    let mut ul = false;
    let mut ol = false;
    let mut bq = false;
    let mut cb = false;
    for buf_line in data.lines() {
        let mut line = String::from(buf_line.trim_start());
        chapter_content.fetch_tag_from_line(&line);
        line = line.add_line_breaks().convert_tags();
        let compiled_line: String;

        // Check the first non-whitespace word to determine what type of line this is.
        let first_word = match line.trim().split_whitespace().nth(0) {
            Some(w) => w,
            _ => {
                // Empty line found - should a tag be closed here?
                // Check if paragraph first
                match p {
                    true => {
                        compiled_line = String::from("</p>\n");
                        p = false;
                    },
                    false => {
                        // Check if blockquote
                        match bq {
                            true => {
                                compiled_line = String::from("</blockquote>\n");
                                bq = false;
                            },
                            false => {
                                // Check if custom block
                                match cb {
                                    true => {
                                        compiled_line = String::from("</div>\n");
                                        cb = false;
                                    },
                                    false => {
                                        // Check if unordered list 
                                        match ul {
                                            true => {
                                                compiled_line = String::from("</ul>\n");
                                                ul = false;
                                            },
                                            false => {
                                                // Check if ordered list
                                                match ol {
                                                    true => {
                                                        compiled_line = String::from("</ol>\n");
                                                        ol = false;
                                                    },
                                                    // Neither p, bq, ul or ol is active, therefore no tags need
                                                    // closing, and this line will just be blank.
                                                    false => {
                                                        compiled_line = String::from("\n");
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                tokens.push(compiled_line);
                continue;
            }
        };
        
        // Go through all known tag symbols and create valid HTML versions.
        match first_word {
            // TODO: Headers get included in bq if there is no blank line between them. That
            // probably should't happen.
            "#" => {
                let line = line.replacen("#", "", 1);
                compiled_line = format!("<h1>{}</h1>\n", line.trim());
                tokens.push(compiled_line.emphasize_text().remove_empty_elements());
                continue;
            },
            "##" => {
                let line = line.replacen("##", "", 1);
                compiled_line = format!("<h2>{}</h2>\n", line.trim());
                tokens.push(compiled_line.emphasize_text().remove_empty_elements());
                continue;
            },
            "###" => {
                let line = line.replacen("###", "", 1);
                compiled_line = format!("<h3>{}</h3>\n", line.trim());
                tokens.push(compiled_line.emphasize_text().remove_empty_elements());
                continue;
            },
            "####" => {
                let line = line.replacen("####", "", 1);
                compiled_line = format!("<h4>{}</h4>\n", line.trim());
                tokens.push(compiled_line.emphasize_text().remove_empty_elements());
                continue;
            },
            "#####" => {
                let line = line.replacen("#####", "", 1);
                compiled_line = format!("<h5>{}</h5>\n", line.trim());
                tokens.push(compiled_line.emphasize_text().remove_empty_elements());
                continue;
            },
            "######" => {
                let line = line.replacen("######", "", 1);
                compiled_line = format!("<h6>{}</h6>\n", line.trim());
                tokens.push(compiled_line.emphasize_text().remove_empty_elements());
                continue;
            },
            "---" => {
                compiled_line = String::from("<hr>\n");
                tokens.push(compiled_line.emphasize_text().remove_empty_elements());
                continue;
            },
            "-" | "*" => {
                if !ul {
                    ul = true;
                    let line = line
                        .replacen("-", "", 1)
                        .replacen("*", "", 1);
                    if p {
                        compiled_line = format!("</p><ul>\n<li>{}</li>\n", line.trim());
                        p = false;
                    } else {
                        compiled_line = format!("<ul>\n<li>{}</li>\n", line.trim());
                    }
                    tokens.push(compiled_line.emphasize_text().remove_empty_elements());
                    continue;
                } else if ul {
                    let line = line.replace("-", "").replace("*", "");
                    compiled_line = format!("<li>{}</li>\n", line.trim());
                    tokens.push(compiled_line.emphasize_text().remove_empty_elements());
                    continue;
                }
            },
            "```" => {
                if !cb {
                    cb = true;
                    let line = line.replace("```", "");
                    if ul {
                        ul = false;
                        compiled_line = format!("</ul><div>{} \n", line.trim());
                        tokens.push(compiled_line.emphasize_text().remove_empty_elements());
                        continue;
                    } else {
                        compiled_line = format!("<div>{} \n", line.trim());
                        tokens.push(compiled_line.emphasize_text().remove_empty_elements());
                        continue;
                    }
                } else if cb {
                    cb = false;
                    let line = line.replace("```", "");
                    if p {
                        p = false;
                        compiled_line = format!("{} </p></div>\n", line.trim());
                        tokens.push(compiled_line.emphasize_text().remove_empty_elements());
                        continue;
                    } else {
                        let line = line.replace("```", "");
                        compiled_line = format!("{} </div>\n", line.trim());
                        tokens.push(compiled_line.emphasize_text().remove_empty_elements());
                        continue;
                    }
                }
            },
            ">" => {
                if !bq {
                    bq = true;
                    let line = line.replace(">", "");
                    if !p {
                        compiled_line = format!("<blockquote>{} ", line.trim());
                        bq = true;
                    } else {
                        compiled_line = format!("</p>\n<blockquote>{} ", line.trim());
                        p = false;
                    }
                    tokens.push(compiled_line.emphasize_text().remove_empty_elements());
                    continue;
                }
            },
            // TODO: How would I check for numbers to make ordered lists?
            
            // If no tag/special formatting is found, assume it is a paragraph or block quote.
            _ => {
                if bq {
                    compiled_line = format!("{} ", line.trim());
                    tokens.push(compiled_line.emphasize_text().remove_empty_elements());
                    continue;
                } else if !p {
                    p = true;
                    line = line.add_line_breaks();
                    compiled_line = format!("<p>{} ", line.trim());
                    tokens.push(compiled_line.emphasize_text().remove_empty_elements());
                    continue;
                } else {
                    compiled_line = format!("{} ", line.trim());
                    tokens.push(compiled_line.emphasize_text().remove_empty_elements());
                    continue;
                }
            }
        }
    }
    // Close tags if they have not already been closed.
    if p {
        tokens.push(String::from("</p>"));
    }

    if bq {
        tokens.push(String::from("</blockquote>"));
    }

    if ul {
        tokens.push(String::from("</ul>"));
    }

    if ol {
        tokens.push(String::from("</ol>"));
    }

    let mut html_string = String::new();
    for line in tokens {
        html_string.push_str(line.as_str());
    }
    chapter_content.html = html_string;
    let json = parse_chapter_content(chapter_content);
    json
}
