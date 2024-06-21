use swc_common::{BytePos};
use swc_ecma_parser::{lexer::Lexer, Parser, TsConfig, Syntax, StringInput};
use swc_ecma_ast::*;

const TS_CONFIG: TsConfig = TsConfig {
   tsx: true,
   dts: false,
   decorators: true,
   no_early_errors: false,
   disallow_ambiguous_jsx_like: false
};

pub fn parse_program(source_code: &str) -> Program {
   let lexer = Lexer::new(
      Syntax::Typescript(TS_CONFIG),
      Default::default(),
      StringInput::new(source_code, BytePos::DUMMY, BytePos::DUMMY),
      None,
  );

  let mut parser = Parser::new_from(lexer);
  let program = parser.parse_program().expect("Failed to parse program");

  program
}
