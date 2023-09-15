package token

type TokenType string

type Token struct {
	Type    TokenType
	Literal string
}

const (
	ILLEGAL = "ILLEGAL"
	EOF     = "EOF"

	LPAREN = "LPAREN"
	RPAREN = "RPAREN"
	COMMA  = "COMMA"

	ID  = "ID"
	INT = "INT"

	LET      = "LET"
	FUNCTION = "FUNC"

	EQUAL  = "EQUAL"  // ==
	ASSIGN = "ASSIGN" // =
)

var keywords = map[string]TokenType{
	"let":  LET,
	"func": FUNCTION,
}

// In order to determine if a string is a keyword or identity, we make use of
// the keywords LUT. If the string isn't in the LUT we return that it's an ID
func LookupIdentity(identity string) TokenType {
	if tok, ok := keywords[identity]; ok {
		return tok
	}
	return ID
}
