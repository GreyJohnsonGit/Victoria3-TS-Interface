import { AppConfig } from '@lib/AppConfig';
import { Context } from '@lib/Context';
import { Ast, Expression } from 'interpreter/Parser';

export interface IInterpreter<T> {
  encode(ast: Ast): T[];
  decode(value: T[], cx: Context): Ast;
  path(config: AppConfig): string;
}

export interface IExpressionInterpreter {
  encode(expression: Expression): unknown;
  decode(value: unknown, cx: Context): Expression;
}

export class ExpressionInterpreter implements IExpressionInterpreter {
  public constructor() {}

  public encode(expression: Expression): unknown {
    switch (expression.kind) {
    case 'number':
      return expression.value;
    case 'string':
      return expression.value;
    case 'array':
      return expression.value.map((e) => this.encode(e));
    case 'assignment':
      return [expression.variable.name, this.encode(expression.expression)];
    case 'variable':
      return expression.name;
    }
  }

  public decode(value: unknown, cx: Context): Expression {
    if (typeof value === 'number') 
      return { kind: 'number', value, context: cx };
    
    if (typeof value === 'string')
      return { kind: 'string', value, context: cx };
    
    if (Array.isArray(value)) {
      return { 
        kind: 'array', 
        value: value
          .filter(v => v !== undefined)
          .map((v) => this.decode(v, cx)),
        context: cx
      };
    }
    
    if (typeof value === 'object' && value !== null) {
      const record = value as Record<string, unknown>;
      return {
        kind: 'array',
        value: Object.entries(record)
          .filter(([,value]) => value !== undefined)  
          .map(([key, value]) => ({ 
            kind: 'assignment', 
            variable: { name: key, context: cx, kind: 'variable' }, 
            expression: this.decode(value, cx), 
            context: cx
          })),
        context: cx
      };
    }
    
    throw new Error(`Unexpected type ${typeof value}`);
  }
}