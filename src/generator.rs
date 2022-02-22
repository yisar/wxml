use crate::lexer::Kind;
use crate::parser::Node;

#[derive(Clone, Debug, PartialEq)]
pub struct Generator {
    pub ast: Node,
    pub code: String,
}

impl Generator {
    pub fn new(ast: Node) -> Generator {
        Generator {
            ast,
            code: "".to_string(),
        }
    }

    pub fn generate_fre(&mut self) {
        let root = self.ast.clone();
        return self.generate_node(root);
    }

    pub fn generate_node(&mut self, node: Node) {
        // println!("{:#?}", node);
        let token = node.token;
        match token.kind {
            Kind::OpenTag(name) => {
                self.code = format!("{}<{}>", self.code, name);
                match node.children {
                    Some(children) => {
                        for child in children {
                            self.generate_node(child)
                        }
                    }
                    None => {}
                }
                self.code = format!("{}</{}>", self.code, name);
            },
            Kind::SelfCloseTag(name) => {
                self.code = format!("{}<{}/>", self.code, name)
            },
            Kind::Text(text) => {
                self.code = format!("{}{}", self.code, text);
            }
            _ => {}
        };

    }
}
