package lexer

import "boof/src/token"

type Lexer struct {
	source string // The source code
	head   int
	ch     byte
}

func New(source string) *Lexer {
	lex := &Lexer{source: source}
	lex.nextChar()
	return lex
}

func (lex *Lexer) Next() token.Token {
	var tok token.Token

	// We want to explicitly skip whitespace in order to catch illegal characters.
	lex.skipWhitespace()

	switch lex.ch {
	case '(':
		tok = newToken(token.LPAREN, lex.ch)
	case ')':
		tok = newToken(token.RPAREN, lex.ch)
	case ',':
		tok = newToken(token.COMMA, lex.ch)
	case '=':
		if lex.peek(1) == '=' {
			tail := lex.ch
			lex.nextChar()
			tok.Type = token.EQUAL
			tok.Literal = string(tail) + string(lex.ch)
		} else {
			tok.Type = token.ASSIGN
			tok.Literal = string(lex.ch)
		}
	case 0:
		tok.Type = token.EOF
		tok.Literal = ""
	default:
		if isLetter(lex.ch) {
			tok.Type = token.LookupIdentity(tok.Literal)
			tok.Literal = lex.readIdentifier()
			// Skip nextChar at the bottom of the func.
			return tok
		} else if isDigit(lex.ch) {
			tok.Type = token.INT
			tok.Literal = lex.readNumber()
			// Skip nextChar at the bottom of the func.
			return tok
		} else {
			tok = newToken(token.ILLEGAL, lex.ch)
		}
	}

	lex.nextChar()
	return tok
}

func newToken(tokenType token.TokenType, ch byte) token.Token {
	return token.Token{Type: tokenType, Literal: string(ch)}
}

func (lex *Lexer) nextChar() {
	// If the lexer has reached the end of the source then assign the head
	// character the value of 0, which is the ASCII code for the NUL terminator.
	if lex.head >= len(lex.source) {
		lex.ch = 0
	} else {
		// Else assign the read character to the next character in the source.
		lex.ch = lex.source[lex.head]
	}
	lex.head++
}

func (lex *Lexer) peek(offset int) byte {
	if lex.head+offset > len(lex.source) {
		return 0
	} else {
		return lex.source[lex.head+offset]
	}
}

func (lex *Lexer) skipWhitespace() {
	for lex.ch == ' ' || // Space
		lex.ch == '\n' || // New line
		lex.ch == '\t' || // Tab
		lex.ch == '\r' { // Carriage
		lex.nextChar()
	}
}

func (lex *Lexer) readIdentifier() string {
	tail := lex.head
	for isLetter(lex.ch) {
		lex.nextChar()
	}
	return lex.source[tail:lex.head]
}

func (lex *Lexer) readNumber() string {
	tail := lex.head
	for isDigit(lex.ch) {
		lex.nextChar()
	}
	return lex.source[tail:lex.head]
}

func isLetter(ch byte) bool {
	// Because characters are numeric, they are ordered. This means we can use the
	// comparison operators to check if the character is within a range.
	return ch >= 'a' && ch <= 'z' ||
		ch >= 'A' && ch <= 'Z' ||
		ch == '_'
}

func isDigit(ch byte) bool {
	return ch >= '0' && ch <= '9'
}
