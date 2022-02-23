pub mod generator;
pub mod lexer;
pub mod parser;

fn main() {
  let mut parser = parser::Parser::new(
    "<view><!-- <button type=\"warn\" bindtap=\"toast\" style=\"margin-top:30px\">showToast</button>
    <button type=\"primary\" bindtap=\"motal\">showMotal</button> --></view>",
  );
  let res = parser.parse_all();
  match res {
    Ok(ast) => {
      let mut gen = generator::Generator::new(ast);
      let code = gen.generate_fre();
      println!("{:#?}", code)
    }
    Err(_) => {}
  }
}
