// https://www.markdownguide.org/cheat-sheet/
static MD_STRING: &str = "
# Hello
---

This is a markdown document
- list item 1
- list item 2
- list item 3
";

static HTML_EXPECTED_STRING: &str = "
<h1 id=\"hello\">Hello</h1>
<hr>
<p>This is a markdown document</p>
<ul>
<li>list item 1</li>
<li>list item 2</li>
<li>list item 3</li>
</ul>
";

fn main() {
    println!("Rust MD Parser");
    println!("==============");

    println!("parsing: ");
    println!("{}", MD_STRING);

    println!("expecting: ");
    println!("{}", HTML_EXPECTED_STRING);
}

fn parse(md_string: String) -> String {
    let mut html_string = String::from("");

    let md_chars = md_string.chars().peekable();

    for c in md_chars {
        match c {
            '#' => {

            }
            '-' => {

            }
            '*' => {

            }
            '>' => {

            }
            '`' => {

            }
            '[' => {

            }
            '!' => {
                
            }
            '\n' => {

            }
            _ => {

            }
        }
    }


    let md_lines = md_string.lines();
    for line in md_lines {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() { 
            continue;
        }

        let peekable_current_line_chars = trimmed_line.chars().peekable();
        for (i, c) in peekable_current_line_chars.enumerate() {
            match c {
                // A heading could be present
                '#' => {

                }
                _ => {}
            }
        }
    }

    html_string
}