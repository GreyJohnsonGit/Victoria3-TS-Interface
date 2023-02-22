import { Context } from './Context';
import { ExpectedArrayGotExpression, ExpectedDefinitionGotExpression, ExpectedFundamentalTypeGotComplexType as FoundUnexpectedExpression } from './Error';
import { Ast, Expression } from './Parser';

export interface Builder<T> {
  set(key: string, value: unknown): Builder<T>;
  validate(cx: Context): T;
}

export function interpretAst<T>(ast: Ast, builder: Builder<T>): T[] {
  const results: T[] = [];

  const definitions = ast.expression.value;

  for (const definition of definitions) {
    if (definition.kind !== 'assignment') {
      throw ExpectedDefinitionGotExpression(definition);
    }

    const { 
      variable: { name, context: cx }, 
      expression: properties 
    } = definition;

    builder.set('name', name);

    if (properties.kind !== 'array') {
      throw ExpectedArrayGotExpression(properties, cx);
    }

    for (const property of properties.value) {
      if (property.kind !== 'assignment') {
        continue;
      }

      const { variable, expression } = property;
      builder.set(variable.name, interpretFundamental(expression));
    }

    results.push(builder.validate(cx));
  }

  return results;
}

function interpretFundamental(expression: Expression): unknown {
  switch (expression.kind) {
  case 'number':
    return expression.value;
  case 'string':
    return expression.value;
  case 'array':
    return expression.value.map((e) => interpretFundamental(e));
  case 'assignment':
    return [expression.variable.name, interpretFundamental(expression.expression)];
  default:
    throw FoundUnexpectedExpression(expression, expression.context);
  }
}