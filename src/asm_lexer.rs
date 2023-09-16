
#[derive(Debug, Clone)]
pub struct Token {
  lexeme: String,
}

#[derive(Debug, Clone)]
pub enum TokenType {
  Text(Token),
  Number(Token),
}

#[derive(Debug, Clone)]
pub struct AsmLexer {
  pub tokens: Vec<TokenType>,
  pub head: u32,
}

#[derive(Debug, Clone)]
enum State {
  Base,
  Text(Token),
  Number(Token),
}

impl AsmLexer {
  pub fn from_src(src: String) -> Self {
    let mut lex = Self { tokens: vec![], head: 0 };
    let mut state: State = State::Base;
    for c in src.chars() {
      match c {
        'A'..='Z' => {
          match &mut state {
            State::Base => state = State::Text(Token { lexeme: c.to_string() }),
            State::Text(tok) => tok.lexeme.push(c),
            State::Number(_) => {}
          }
        }
        '0'..='9' => {
          match &mut state {
            State::Base => state = State::Number(Token { lexeme: c.to_string() }),
            State::Number(tok) => tok.lexeme.push(c),
            State::Text(_) => {}
          }
        }
        ' ' => {
          match &state {
            State::Text(tok) => {
              lex.tokens.push(TokenType::Text(tok.clone()));
              state = State::Base;
            },
            State::Number(tok) => {
              lex.tokens.push(TokenType::Number(tok.clone()));
              state = State::Base;
            },
            State::Base => {}
          }
        },
        _ => {}
      }
    }
    lex
  }

  pub fn next(&mut self) { self.head += 1 }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_lex_src() {
    let src = "INT 1 AB ".to_string();
    let lex = AsmLexer::from_src(src);
    println!("{:?}", lex.tokens);
	}
}