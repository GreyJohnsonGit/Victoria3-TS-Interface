import { ContextualExpression, Expression } from 'Parser';
import { Token } from 'Token';
import { Context, toLink } from './Context';

export function UnexpectedToken(character: string, cx: Context) {
  return new Error(`Unexpected Token '${character}' at ${toLink(cx)}`);
}

export function UnfinishedStringToken(cx: Context) {
  return new Error(`Unfinished String at ${toLink(cx)}`);
}

export function UnfinishedHexStringToken(cx: Context) {
  return new Error(`Unfinished Hex String at ${toLink(cx)}`);
}

export function InvalidNumberToken(cx: Context) {
  return new Error(`Attempted to tokenize non-number as number at ${toLink(cx)}`);
}

export function InvalidSymbolToken(cx: Context) {
  return new Error(`Attempted to tokenize non-symbol as symbol at ${toLink(cx)}`);
}

export function InvalidToken(cx: Context) {
  return new Error(`Invalid Token at ${toLink(cx)}`);
}

export function ExpectedAst(expression: Expression) {
  return new Error(`Expected an AST, got ${expression.kind}`);
}

export function ExpectedOpenGotEOF() {
  return new Error('Expected \'{\', got end of file');
}

export function ExpectedOpenGotLiteral(token: Token, cx: Context) {
  return new Error(`Expected '{', got ${token.kind} at ${toLink(cx)}`);
}

export function ExpectedExpressionOrCloseGotEOF() {
  return new Error('Expected Expression or \'}\', got end of file');
}

export function ExpectedCloseGotEOF() {
  return new Error('Expected \'}\', got end of file');
}

export function ExpectedExpressionGotEOF() {
  return new Error('Expected Expression, got end of file');
}

export function ExpectedExpressionGotToken(token: Token, cx: Context) {
  return new Error(`Expected Expression, got ${token.kind} at ${toLink(cx)}`);
}

export function ExpectedVariableGotEOF() {
  return new Error('Expected Variable, got end of file');
}

export function ExpectedVariableGotToken(token: Token, cx: Context) {
  return new Error(`Expected Variable, got ${token.kind} at ${toLink(cx)}`);
}

export function ExpectedAssignmentGotEOF() {
  return new Error('Expected \'=\', got end of file.');
}

export function ExpectedAssignmentGotToken(token: Token, cx: Context) {
  return new Error(`Expected '=', got ${token.kind} at ${toLink(cx)}`);
}

export function ExpectedDefinitionGotExpression(expression: ContextualExpression) {
  return new Error(`Expected Definition, got ${expression.kind} at ${toLink(expression.context)}`);
}

export function ExpectedArrayGotExpression(expression: Expression, cx: Context) {
  return new Error(`Expected Array, got ${expression.kind} at ${toLink(cx)}`);
}

export function ExpectedFundamentalTypeGotComplexType(expression: Expression, cx: Context) {
  return new Error(`Expected Fundamental Type (string, number, Array, Assignment), got ${expression.kind} at ${toLink(cx)}`);
}

export function ExpectedString(key: string, value: unknown, cx: Context) { 
  return new Error(`Expected ${key} to be a string, got ${typeof value} in definition at ${toLink(cx)}`);
}

export function ExpectedNumber(key: string, value: unknown, cx: Context) {
  return new Error(`Expected ${key} to be a number, got ${typeof value} in definition at ${toLink(cx)}`);
}

export function ExpectedStringArray(key: string, value: unknown, cx: Context) {
  return new Error(`Expected ${key} to be an array of strings, got ${typeof value} in definition at ${toLink(cx)}`);
}

export function ExpectedRawCappedResource(key: string, value: unknown, cx: Context) {
  return new Error(`Expected ${key} to be of type [string, number, boolean][], got ${value} in definition at ${toLink(cx)}`);
}