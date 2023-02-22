import { Context } from './Context';
import { Builder, interpretAst } from './Interpreter';
import { Ast } from './Parser';
import * as basic from './test_cases/basic.json';

describe('interpretAst', () => {
  it('should interpret an AST', () => {
    // Arrange
    const ast = basic.parsed as Ast;

    type TestObj = {
      id: number;
      name: string;
      ids: number[];
      names: string[];
      context: Context;
    };

    const builder: Builder<TestObj> = (() => {
      const obj: Record<string, unknown> = {};
      
      const set = () => ({
        validate: (cx: Context) => ({...obj, context: cx} as TestObj),
        set: (key: string, value: unknown) => {
          obj[key] = value;
          return set();
        }
      });
    
      return set();
    })();

    // Act
    const result = interpretAst(ast, builder);

    // Assert
    expect(result).toEqual(basic.interpreted);
  });
});