import { IInterpreter } from 'interpreter/Interpreter';

export interface IModelGenerator<Model> {
  interpreter: IInterpreter<Model>;
  format(models: Model[]): string;
  options: {
    gameFilepath: string;
    generatorFilePath: string;
    isFolder?: boolean;
  };
}