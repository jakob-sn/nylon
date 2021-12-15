use std::fs;
use std::error::Error;
use std::iter::Peekable;
use std::str::Chars;
use ast;
use ast::TopNode;

#[derive(Debug, PartialEq)]
pub enum Text {
    Text(String),
    Inline(String),
    StatementBlock(String)
}

pub struct Parser {
    content: String,
    split: Vec<Text>
}

impl Parser {
    pub fn new(input: String) -> Self {
        Parser { content: input, split: Vec::new() }
    }

    pub fn new_from_file(filepath: String) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(filepath)?.parse()?;

        Ok(Parser { content, split: Vec::new() })
    }

    fn split_content(&mut self) {
        let mut it = self.content.chars().peekable();

        self.split = Self::parse_text_block(&mut it);
    }

    pub fn parse(&mut self) -> Result<ast::TopNode, Error> {
        self.split_content();

        let mut statement_block_parser = grammar::StatementBlockParser::new();

        

        todo!()
    }

    fn parse_text_block(it: &mut Peekable<Chars>) -> Vec<Text> {
        let mut block = String::new();

        while let Some(c) = it.next() {
            if c == '=' {
                if let Some(n) = it.peek() {
                    match *n {
                        '=' => {
                            // Escaped '=', still text
                            block.push(c);
                        }
                        '{' => {
                            // '={' start of code block
                            it.next();

                            let mut v = vec![Text::Text(block)];
                            v.append(&mut Self::parse_statement_block(it));
                            return v;
                        }
                        _ => {
                            // Inline statement
                            let mut v = vec![Text::Text(block)];
                            v.append(&mut Self::parse_inline_statement(it));
                            return v;
                        }
                    }
                }
            } else {
                block.push(c);
            }
        }

        return vec![Text::Text(block)];
    }

    fn parse_inline_statement(it: &mut Peekable<Chars>) -> Vec<Text> {
        let mut stmt = String::new();

        // Identifier
        while let Some(c) = it.next() {
            stmt.push(c);
            if c == '(' {
                break;
            }
        }

        // Arguments
        let mut paren_depth = 1;
        while let Some(c) = it.next() {
            stmt.push(c);

            if c == '(' {
                paren_depth += 1;
            } else if c == ')' {
                paren_depth -= 1;
            }

            if paren_depth == 0 {
                break;
            }
        }

        let mut v = vec![Text::Inline(stmt)];
        v.append(&mut Self::parse_text_block(it));
        v
    }

    fn parse_statement_block(it: &mut Peekable<Chars>) -> Vec<Text> {
        let mut stmt = String::from("={");
        let mut paren_depth = 1;

        while let Some(c) = it.next() {
            stmt.push(c);

            if c == '{' {
                paren_depth += 1;
            } else if c == '}' {
                paren_depth -= 1;
            }

            if paren_depth == 0 {
                break;
            }
        }

        let mut v = vec![Text::StatementBlock(stmt)];
        v.append(&mut Self::parse_text_block(it));
        v
    }
}

impl From<String> for Parser {
    fn from(input: String) -> Self {
        Parser { content: input, split: Vec::new() }
    }
}

#[cfg(test)]
mod test {
    use parser::*;

    #[test]
    fn split_content() {
        let src = String::from("={
	set_font(test_font);
	var = \"test\";
}

=title({ title = \"Test document\", author = \"Author\" })

=section(Hello team)

This is a paragraph?

Bold text: =bold(this is theoretically bold).

=noindent() no indentation

={ use nd = noindent; }

=nd() no indent again

");
        let mut parsed = Parser::new(src);
        parsed.split_content();
        let parsed = parsed.split;

        let expected = vec![
            Text::Text(String::from("")),
            Text::StatementBlock(String::from(
                "={\n\tset_font(test_font);\n\tvar = \"test\";\n}",
            )),
            Text::Text(String::from("\n\n")),
            Text::Inline(String::from("title({ title = \"Test document\", author = \"Author\" })")),
            Text::Text(String::from("\n\n")),
            Text::Inline(String::from("section(Hello team)")),
            Text::Text(String::from("\n\nThis is a paragraph?\n\nBold text: ")),
            Text::Inline(String::from("bold(this is theoretically bold)")),
            Text::Text(String::from(".\n\n")),
            Text::Inline(String::from("noindent()")),
            Text::Text(String::from(" no indentation\n\n")),
            Text::StatementBlock(String::from("={ use nd = noindent; }")),
            Text::Text(String::from("\n\n")),
            Text::Inline(String::from("nd()")),
            Text::Text(String::from(" no indent again\n\n")),
        ];

        assert_eq!(parsed, expected);
    }
}
