import { Context, IContextual } from '@lib/Context';
import * as basic from '@test-data/basic.json';
import { ExpressionInterpreter, IInterpreter } from 'interpreter/Interpreter';
import { Assignment, Ast, Expression } from 'interpreter/Parser';

describe('GenericInterpreter', () => {
  it('Should encode Ast with only context loss', () => {
    // Arrange
    const { ast, obj } = basic;
    const interpreter = new ExpressionInterpreter();
    const objInterpreter = new ObjInterpreter(interpreter);

    // Act
    const encoded = objInterpreter.encode(ast.expression as Expression);

    // Assert
    expect(removeContext(encoded)).toEqual(removeContext(obj));
  });

  it('Should decode obj with only context loss', () => {
    // Arrange
    const context = { filepath: 'test', line: 0, column: 0 };
    const { ast, obj } = basic;
    const interpreter = new ExpressionInterpreter();
    const objInterpreter = new ObjInterpreter(interpreter);

    // Act
    const decoded = objInterpreter.decode(obj, context);

    // Assert
    expect(removeContext(decoded)).toEqual(removeContext(ast));
  });

  it('Should encode and decode with only context loss', () => {
    // Arrange
    const context = { filepath: 'test', line: 0, column: 0 };
    const { ast } = basic;
    const interpreter = new ExpressionInterpreter();
    const objInterpreter = new ObjInterpreter(interpreter);

    // Act
    const encoded = objInterpreter.encode(ast.expression as Expression);
    const decoded = objInterpreter.decode(encoded, context);
    
    // Assert
    if (decoded.kind !== 'assignment') throw new Error('Expected assignment');
    if (decoded.expression.kind !== 'array') throw new Error('Expected array');
    const decodedObj = decoded.expression.value[0];
    if (decodedObj.kind !== 'assignment') throw new Error('Expected assignment');
    if (decodedObj.expression.kind !== 'array') throw new Error('Expected array');
    const decodedValues = decodedObj.expression.value as Assignment[];
    const astValues = ast.expression.value[0].expression.value as Assignment[];
    for (const decodedValue of decodedValues) {
      const astValue = astValues.find(v => v.variable.name === decodedValue.variable.name);
      expect(removeContext(decodedValue.expression)).toEqual(removeContext(astValue?.expression));
    }
  });

  it('Should decode and encode with only context loss', () => {
    // Arrange
    const context = { filepath: 'test', line: 0, column: 0 };
    const { obj } = basic;
    const interpreter = new ExpressionInterpreter();
    const objInterpreter = new ObjInterpreter(interpreter);

    // Act
    const decoded = objInterpreter.decode(obj, context);
    if (decoded.kind !== 'assignment') throw new Error('Expected assignment');
    const encoded = objInterpreter.encode(decoded.expression);

    // Assert
    expect(removeContext(encoded)).toEqual(removeContext(obj));
  });
});

class Obj implements IContextual {
  objId: string;
  id: number;
  ids: number[];
  name: string;
  names: string[];
  context: Context;

  constructor(
    obj: unknown, 
  ) {
    const fullObj = obj as Obj;
    this.objId = fullObj.objId;
    this.id = fullObj.id;
    this.ids = fullObj.ids;
    this.name = fullObj.name;
    this.names = fullObj.names;
    this.context = fullObj.context;
  }
}

class ObjInterpreter implements IInterpreter<Obj> {
  constructor(
    private expressionInterpreter: ExpressionInterpreter
  ) {}

  encode(expression: Expression): Obj[] {
    const obj = {} as Partial<Obj> & Record<string, unknown>;
    if (expression.kind !== 'array')
      throw new Error('Expected array');

    for (const assignment of expression.value) {
      if (assignment.kind !== 'assignment')
        throw new Error('Expected assignment');

      obj.context = assignment.context;
      obj.objId = assignment.variable.name;
    
      if (assignment.expression.kind !== 'array')
        throw new Error('Expected array');

      for (const field of assignment.expression.value) {
        if (field.kind !== 'assignment')
          throw new Error('Expected assignment');

        obj[field.variable.name] = this.expressionInterpreter.encode(field.expression);
      }
    }
    return [new Obj(obj)];
  }
  
  decode(objs: Obj[], cx: Context): Ast {
    return {
      kind: 'assignment',
      variable: {
        kind: 'variable',
        name: '#GLOBAL',
        context: cx,
      },
      context: cx,
      expression: {
        kind: 'array',
        context: cx,
        value: objs.map(obj => ({
          kind: 'assignment',
          variable: {
            kind: 'variable',
            name: obj.objId,
            context: cx,
          },
          context: cx,
          expression: {
            kind: 'array',
            context: cx,
            value: Object
              .entries(obj)
              .filter(([key]) => key !== 'objId')
              .filter(([key]) => key !== 'context')
              .map(([key, value]) => {
                return {
                  kind: 'assignment' as const,
                  variable: {
                    kind: 'variable' as const,
                    name: key,
                    context: cx,
                  },
                  expression: this.expressionInterpreter.decode(value, cx),
                  context: cx,
                };
              })
          }
        }))
      }
    };
  }

  path(): string {
    throw new Error('Method not implemented.');
  }
}

function removeContext(obj: unknown): unknown {
  if (typeof obj === 'object' && obj !== null) {
    const record = obj as Record<string, unknown>;
    if (Object.keys(record).includes('context'))
      delete record.context;

    for (const key in record) {
      record[key] = removeContext(record[key]);
    }
  }
  return obj;
}