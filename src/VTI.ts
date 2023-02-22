import { Builder, interpretAst } from './Interpreter';
import { parse } from './Parser';
import { tokenize } from './Token';

export function interpret<T>(text: string, filepath: string, builder: Builder<T>) {
  const tokens = tokenize(text, { filepath, line: 1, column: 0 });
  const ast = parse(tokens);
  return interpretAst(ast, builder);
}