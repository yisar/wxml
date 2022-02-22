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
                let tag = self.camel_case(name);
                self.code = format!("{}<{}", self.code, tag);
                for attr in token.attributes.unwrap() {
                    if let Kind::Attribute(name, value) = attr.kind {
                        let real_name = self.wried_prop(name);
                        self.code = format!("{} {}=\"{}\"", self.code, real_name, value)
                    }
                }
                self.code += ">";
                for child in node.children.unwrap() {
                    self.generate_node(child)
                }
                self.code = format!("{}</{}>", self.code, tag);
            }
            Kind::SelfCloseTag(name) => {
                let tag = self.first_upper(name);
                self.code = format!("{}<{}", self.code, tag);
                for attr in token.attributes.unwrap() {
                    if let Kind::Attribute(name, value) = attr.kind {
                        self.code = format!("{} {}=\"{}\"", self.code, name, value)
                    }
                }
                self.code += ">";
            }
            Kind::Text(text) => {
                self.code = format!("{}{}", self.code, text);
            }
            _ => {}
        };
    }
}

impl Generator {
    fn first_upper(&mut self, s: String) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }

    fn wried_prop(&mut self, p: String) -> String {
        if p.starts_with("bind") {
            let n = p.replace("bind", "");
            return format!(
                "on{}",
                match n.as_str() {
                    "tap" => "click".to_string(),
                    "click" => "keydown".to_string(),
                    _ => n,
                }
            );
        } else {
            p
        }
    }

    fn camel_case(&mut self, s: String) -> String {
        let arr: Vec<&str> = s.split("-").collect();
        let mut out = "".to_string();
        for s in arr {
            out = format!("{}{}", out, self.first_upper(s.to_string()));
        }
        out
    }
}
