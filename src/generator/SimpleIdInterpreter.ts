import { AppConfig } from '@lib/AppConfig';
import { Context } from '@lib/Context';
import { ErrorFactory } from '@lib/Error';
import { IInterpreter } from 'interpreter/Interpreter';
import { Ast } from 'interpreter/Parser';

export type SimpleId = string | number;

export class SimpleIdInterpreter<Id extends SimpleId> implements IInterpreter<SimpleId> {
  encode(ast: Ast): Id[] {
    if (ast.kind !== 'assignment')
      throw ErrorFactory.ExpectedAssignment(ast);
    
    const assignments = ast.expression;
    if (assignments.kind !== 'array')
      throw ErrorFactory.ExpectedArray(ast);

    const ids: Id[] = [];
    assignments.value.forEach((e) => {
      if (e.kind !== 'assignment')
        throw ErrorFactory.ExpectedAssignment(e);

      ids.push(e.variable.name as Id);
    });

    return ids;
  }

  decode(_value: Id[], _cx: Context): Ast {
    throw new Error('Method not implemented.');
  }

  path(_config: AppConfig): string {
    throw new Error('Method not implemented.');
  }
}