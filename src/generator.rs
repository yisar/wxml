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

    pub fn generate_fre(&mut self) -> String {
        let root = self.ast.clone();
        return self.generate_node(root);
    }

    pub fn generate_node(&mut self, node: Node) -> String {
        let token = node.token;
        let mut directs = vec![];
        let mut code = "".to_string();

        match token.kind {
            Kind::OpenTag(name) => {
                let tag = self.camel_case(name);
                code = format!("{}<{}", code, tag);
                for attr in token.attributes.unwrap() {
                    if let Kind::Attribute(name, value) = attr.kind {
                        let prop = self.wried_prop(name);
                        let expression = self.take_expression(value);

                        match prop.as_str() {
                            "wx:key" => code = format!("{} {}=\"{}\"", code, "key", expression),
                            "wx:if" => {
                                directs.push(("if", expression));
                            }
                            "wx:for" => directs.push(("for", expression)),
                            _ => code = format!("{} {}=\"{}\"", code, prop, expression),
                        }
                    }
                }
                code += ">";
                for child in node.children.unwrap() {
                    let str = self.generate_node(child);
                    code = format!("{}{}", code, str);
                }
                code = format!("{}</{}>", code, tag);
            }
            Kind::SelfCloseTag(name) => {
                let tag = self.first_upper(name);
                code = format!("{}<{}", code, tag);
                for attr in token.attributes.unwrap() {
                    if let Kind::Attribute(name, value) = attr.kind {
                        let prop = self.wried_prop(name);
                        let expression = self.take_expression(value);
                        match prop.as_str() {
                            "wx:key" => code = format!("{} {}=\"{}\"", code, "key", expression),
                            "wx:if" => {
                                directs.push(("if", expression));
                            }
                            "wx:for" => directs.push(("for", expression)),
                            _ => code = format!("{} {}=\"{}\"", code, prop, expression),
                        }
                    }
                }
                code += "/>";
            }
            Kind::Text(text) => {
                code = format!("{}{}", code, text);
            }
            _ => {}
        };
        let c = self.generate_directs(directs, code);
        return c;
    }

    pub fn generate_directs(&mut self, directs: Vec<(&str, String)>, code: String) -> String {
        for direct in directs {
            match direct.0 {
                "if" => {
                    return code;
                }
                "for" => return format!("{{{}.map((item)=>{}}};", direct.1, code),
                _ => {
                    return code;
                }
            }
        }
        return code;
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

    fn take_expression(&mut self, e: String) -> String {
        // todo
        return e.replace("{{", "").replace("}}", "");
    }
}
