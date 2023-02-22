import { Context } from 'Context';
import { ExpectedAssignmentGotEOF, ExpectedAssignmentGotToken, ExpectedAst, ExpectedCloseGotEOF, ExpectedExpressionGotEOF, ExpectedExpressionGotToken, ExpectedExpressionOrCloseGotEOF, ExpectedOpenGotEOF, ExpectedOpenGotLiteral, ExpectedVariableGotEOF, ExpectedVariableGotToken } from './Error';
import { isNumber, isString, isSymbol, isTokenLiteral, Token } from './Token';

export function parse(tokenStream: Token[]): Ast {
  const [ast] = parseExpression(tokenStream, 0);
  
  if (!isAstRoot(ast)) {
    throw ExpectedAst(ast);
  }

  return ast;
}

function parseExpressionArray(
  tokenStream: Token[], 
  index: number
): [ExpressionArray, number] {
  const expressions: Expression[] = [];
  let tokensConsumed = 0;
  
  const openBrace = tokenStream.at(index);
  if (openBrace === undefined) {
    throw ExpectedOpenGotEOF();
  }
  const cx = openBrace.context;

  if (!isTokenLiteral(openBrace, '{')) {
    throw ExpectedOpenGotLiteral(openBrace, cx);
  }
  tokensConsumed++;

  let token = tokenStream.at(index + tokensConsumed);
  if (token === undefined) {
    throw ExpectedExpressionOrCloseGotEOF();
  }

  while (!isTokenLiteral(token, '}')) {
    const [expression, consumed] = parseExpression(
      tokenStream, 
      index + tokensConsumed
    );

    expressions.push(expression);
    tokensConsumed += consumed;
    token = tokenStream.at(index + tokensConsumed);

    if (token === undefined) {
      throw ExpectedCloseGotEOF();
    }
  }
  tokensConsumed++;

  return [{
    kind: 'array',
    value: expressions,
    context: openBrace.context
  }, tokensConsumed];
}

function parseExpression(
  tokenStream: Token[], 
  index: number
): [Expression, number] {
  const token = tokenStream.at(index);
  const nextToken = tokenStream.at(index + 1);

  if (token === undefined) {
    throw ExpectedExpressionGotEOF();
  }
  const cx = token.context;

  if (isTokenLiteral(token, '{')) {
    return parseExpressionArray(tokenStream, index);
  }

  if (isSymbol(token) && nextToken !== undefined) {
    if (isTokenLiteral(nextToken, '=')) {
      return parseAssignment(tokenStream, index);
    }
  }

  if (isString(token)) {
    return [{
      kind: 'string',
      value: token.value,
      context: cx
    }, 1];
  }

  if (isNumber(token)) {
    return [{
      kind: 'number',
      value: token.value,
      context: cx
    }, 1];
  }

  if (isSymbol(token)) {
    return [{
      kind: 'variable',
      name: token.value,
      context: cx
    }, 1];
  }

  throw ExpectedExpressionGotToken(token, cx);
}

function parseAssignment(
  tokenStream: Token[], 
  index: number
): [Assignment, number] {
  let tokensConsumed = 0;
  const variable = tokenStream.at(index);
  if (variable === undefined) {
    throw ExpectedVariableGotEOF();
  }
  const cx = variable.context;

  if (!isSymbol(variable)) {
    throw ExpectedVariableGotToken(variable, cx);
  }
  tokensConsumed++;

  const equals = tokenStream.at(index + tokensConsumed);
  if (equals === undefined) {
    throw ExpectedAssignmentGotEOF();
  }

  if (!isTokenLiteral(equals, '=')) {
    throw ExpectedAssignmentGotToken(equals, cx);
  }
  tokensConsumed++;

  const [expression, consumed] = parseExpression(
    tokenStream, 
    index + tokensConsumed
  );
  tokensConsumed += consumed;
  
  return [
    {
      kind: 'assignment',
      variable: {
        kind: 'variable',
        name: variable.value,
        context: cx
      },
      expression,
    }, 
    tokensConsumed
  ];
}

export type Ast = {
  kind: 'assignment';
  variable: {
    kind: 'variable';
    name: '#GLOBAL';
    context: Context;
  };
  expression: ExpressionArray;
};

function isAstRoot(expression: Expression): expression is Ast {
  return expression.kind === 'assignment' && 
    expression.variable.name === '#GLOBAL' &&
    expression.expression.kind === 'array';
}

export type Expression = ContextualExpression | Assignment;

export type ContextualExpression = Literal | Variable | ExpressionArray;

export type Assignment = {
  kind: 'assignment';
  variable: Variable;
  expression: Expression;
}

type Literal = NumberLiteral | StringLiteral;

type NumberLiteral = {
  kind: 'number';
  value: number;
  context: Context;
}

type StringLiteral = {
  kind: 'string';
  value: string;
  context: Context;
}

type Variable = {
  kind: 'variable';
  name: string;
  context: Context;
}

type ExpressionArray = {
  kind: 'array';
  value: Expression[];
  context: Context;
}