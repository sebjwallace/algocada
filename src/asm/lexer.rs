
#[derive(Debug, Clone)]
pub struct Token {
  pub lexeme: String,
}

#[derive(Debug, Clone)]
pub enum TokenType {
  Text(Token),
  Number(Token),
  Label(Token),
  Semicolon,
  EOF,
}

impl TokenType {
    pub fn get_lexeme (&self) -> String {
      match self {
        TokenType::Text(tok) | TokenType::Number(tok) => tok.lexeme.clone(),
        _ => "".to_string()
      }
    }
}

#[derive(Debug, Clone)]
pub struct AsmLexer {
  pub tokens: Vec<TokenType>,
  pub cursor: usize,
}

#[derive(Debug, Clone)]
enum State {
  Base,
  Text(Token),
  Number(Token),
  Label(Token),
}

impl AsmLexer {
  pub fn from_src(src: String) -> Self {
    let mut lex = Self { tokens: vec![], cursor: 0 };
    let mut state: State = State::Base;
    for c in src.chars() {
      match c {
        'A'..='Z' => {
          match &mut state {
            State::Base => state = State::Text(Token { lexeme: c.to_string() }),
            State::Text(tok) => tok.lexeme.push(c),
            State::Number(_) | State::Label(_) => {}
          }
        }
        '0'..='9' => {
          match &mut state {
            State::Base => state = State::Number(Token { lexeme: c.to_string() }),
            State::Number(tok) => tok.lexeme.push(c),
            State::Text(tok) => tok.lexeme.push(c),
            State::Label(_) => {}
          }
        }
        ':' => {
          match &state {
            State::Text(tok) => state = State::Label(tok.clone()),
            _ => {}
          }
        }
        ' ' | ';' => {
          match &state {
            State::Text(tok) => {
              lex.tokens.push(TokenType::Text(tok.clone()));
              state = State::Base;
            },
            State::Number(tok) => {
              lex.tokens.push(TokenType::Number(tok.clone()));
              state = State::Base;
            },
            State::Label(tok) => {
              lex.tokens.push(TokenType::Label(tok.clone()));
              state = State::Base;
            }
            State::Base => {}
          }
          match c {
            ';' => lex.tokens.push(TokenType::Semicolon),
            _ => {}
          }
        },
        _ => {}
      }
    }
    lex
  }

  pub fn current(&self) -> &TokenType {
    match self.tokens.get(self.cursor) {
      Some(tok) => tok,
      None => &TokenType::EOF
    }
  }

  pub fn next(&mut self) -> &TokenType {
    self.cursor += 1;
    self.current()
  }

  pub fn next_lexeme(&mut self) -> String {
    self.next().get_lexeme()
  }

  pub fn eof(&self) -> bool {
    self.cursor >= self.tokens.len()
  }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_lex_src() {
    let src = "INT 1 AB;".to_string();
    let lex = AsmLexer::from_src(src);
    println!("{:?}", lex.tokens);
	}
}