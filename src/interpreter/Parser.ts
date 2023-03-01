import { IContextual } from '@lib/Context';
import { ExpectedAssignmentGotEOF, ExpectedAssignmentGotToken, ExpectedAst, ExpectedCloseGotEOF, ExpectedExpressionGotEOF, ExpectedExpressionGotToken, ExpectedExpressionOrCloseGotEOF, ExpectedOpenGotEOF, ExpectedOpenGotLiteral, ExpectedVariableGotEOF, ExpectedVariableGotToken } from '@lib/Error';
import { isNumber, isString, isSymbol, isTokenLiteral, Token } from 'interpreter/Token';

export interface IParser {
  encode(tokens: Token[]): Ast;
  decode(ast: Ast): Token[];
}

export class Parser implements IParser {
  public constructor() {}

  public encode(tokens: Token[]): Ast {
    const [ast] = this.encodeExpression(tokens, 0);
  
    if (!isAst(ast)) {
      throw ExpectedAst(ast);
    }

    return ast;
  }
  
  public decode(ast: Ast): Token[] {
    return this.decodeExpression(ast);
  }

  private encodeExpression(
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
      return this.encodeExpressionArray(tokenStream, index);
    }
  
    if (isSymbol(token) && nextToken !== undefined) {
      if (isTokenLiteral(nextToken, '=')) {
        return this.encodeAssignment(tokenStream, index);
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
  
  private encodeExpressionArray(
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
      const [expression, consumed] = this.encodeExpression(
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
  
  private encodeAssignment(
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
  
    const [expression, consumed] = this.encodeExpression(
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
        context: cx
      }, 
      tokensConsumed
    ];
  }

  private decodeExpression(expression: Expression): Token[] {
    const tokens: Token[] = [];

    if (expression.kind === 'string') tokens.push({
      kind: 'string',
      value: expression.value,
      context: expression.context
    });

    if (expression.kind === 'number') tokens.push({
      kind: 'number',
      value: expression.value,
      context: expression.context
    });

    if (expression.kind === 'variable') tokens.push({ 
      kind: 'symbol', 
      value: expression.name, 
      context: expression.context
    });

    if (expression.kind === 'array') tokens.push(
      { kind: '{', context: expression.context },
      ...expression.value.flatMap(e => this.decodeExpression(e)),
      { kind: '}', context: expression.context }
    );

    if (expression.kind === 'assignment') tokens.push(
      { kind: 'symbol', value: expression.variable.name, context: expression.context },
      { kind: '=', context: expression.context },
      ...this.decodeExpression(expression.expression)
    );

    return tokens;
  }
}

export type Ast = Assignment & {
  variable: { name: '#GLOBAL' };
  expression: ExpressionArray;
};

export type Expression = Literal | Variable | ExpressionArray | Assignment;

export type Literal = NumberLiteral | StringLiteral;

export interface Assignment extends IContextual {
  kind: 'assignment';
  variable: Variable;
  expression: Expression;
}

export interface NumberLiteral extends IContextual {
  kind: 'number';
  value: number;
}

export interface StringLiteral extends IContextual {
  kind: 'string';
  value: string;
}

export interface Variable extends IContextual {
  kind: 'variable';
  name: string;
}

export interface ExpressionArray extends IContextual {
  kind: 'array';
  value: Expression[];
}

function isAst(expression: Expression): expression is Ast {
  return expression.kind === 'assignment' && 
    expression.variable.name === '#GLOBAL' &&
    expression.expression.kind === 'array';
}