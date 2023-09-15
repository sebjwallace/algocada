package lexer

import (
	"boof/src/token"
	"testing"
)

func Add(a int, b int) int {
	return a + b
}

func TestAdd(t *testing.T) {
	src := "(=,)"
	lex := New(src)

	tests := []struct {
		expectedType    token.TokenType
		expectedLiteral string
	}{
		{token.LPAREN, "("},
		{token.ASSIGN, "="},
		{token.COMMA, ","},
		{token.RPAREN, ")"},
		{token.EOF, ""},
	}

	for i, tt := range tests {
		tok := lex.Next()
		if tok.Type != tt.expectedType {
			t.Fatalf("tests[%d] - tokentype wrong. expected=%q, got=%q",
				i, tt.expectedType, tok.Type)
		}
		if tok.Literal != tt.expectedLiteral {
			t.Fatalf("tests[%d] - literal wrong. expected=%q, got=%q",
				i, tt.expectedLiteral, tok.Literal)
		}
	}
}
