use super::lexer::{TokenType,AsmLexer};

#[derive(Debug, Clone)]
pub struct Program {
  pub instructions: Vec<Instruction>,
}

impl Program {
  pub fn from_lexer(lex: &mut AsmLexer) -> Self {
    let mut instructions: Vec<Instruction> = vec![];
    loop {
      match lex.current() {
        TokenType::Text(_) => {
          instructions.push(Instruction::from_lexer(lex));
          lex.next();
        }
        TokenType::Number(_) => {}
        TokenType::Semicolon => { lex.next(); }
        TokenType::Label(_) => todo!(),
        TokenType::EOF => break
      }
    }
    Self { instructions }
  }
}

type UnaryOperand = String;
type BinaryOperand = (String, String);
type TernaryOperand = (String, String, String);

#[derive(Debug, Clone)]
pub enum Instruction {
  // Movement
  MOV(BinaryOperand),
  // Arithmetic
  ADD(TernaryOperand),
  SUB(TernaryOperand),
  // Logic
  AND(TernaryOperand),
  OR(TernaryOperand),
  XOR(TernaryOperand),
  EQ(TernaryOperand),
  // Branching
  JMP(UnaryOperand),
  // Control flow
  CALL(UnaryOperand),
  RET,
  HALT,
  // Time
  WAIT(UnaryOperand),
  // Misc
  ERR
}

impl Instruction {
  pub fn from_lexer(lex: &mut AsmLexer) -> Self {
    match lex.current() {
      TokenType::Text(tok) => {
        match tok.lexeme.as_str() {
          "MOV" => Instruction::MOV((lex.next_lexeme(), lex.next_lexeme())),
          "ADD" => Instruction::ADD((lex.next_lexeme(), lex.next_lexeme(), lex.next_lexeme())),
          "SUB" => Instruction::SUB((lex.next_lexeme(), lex.next_lexeme(), lex.next_lexeme())),
          "AND" => Instruction::AND((lex.next_lexeme(), lex.next_lexeme(), lex.next_lexeme())),
          "OR" => Instruction::OR((lex.next_lexeme(), lex.next_lexeme(), lex.next_lexeme())),
          "XOR" => Instruction::XOR((lex.next_lexeme(), lex.next_lexeme(), lex.next_lexeme())),
          "EQ" => Instruction::EQ((lex.next_lexeme(), lex.next_lexeme(), lex.next_lexeme())),
          "JMP" => Instruction::JMP(lex.next_lexeme()),
          "CALL" => Instruction::CALL(lex.next_lexeme()),
          "RET" => Instruction::RET,
          "WAIT" => Instruction::WAIT(lex.next_lexeme()),
          _ => Instruction::ERR,
        }
      },
      _ => Instruction::ERR
    }
  }
}

pub struct AsmParser {
  pub program: Program,
}

impl AsmParser {
  pub fn from_lexer(lex: &mut AsmLexer) -> Self {
    let program = Program::from_lexer(lex);
    Self { program }
  } 
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_src() {
      let src = "MOV R1 R2; ADD R0 R1 R2;".to_string();
      let mut lex = AsmLexer::from_src(src);
      let parser = AsmParser::from_lexer(&mut lex);
      println!("{:#?}", parser.program);
	}
}