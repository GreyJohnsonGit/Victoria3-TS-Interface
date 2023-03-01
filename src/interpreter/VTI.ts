import * as FileSystem from 'fs';
import { AppConfig } from 'src/@lib/AppConfig';
import { IPersistent } from 'src/@lib/IPersistent';
import { IInterpreter } from 'src/interpreter/Interpreter';
import { Ast, IParser, Parser } from 'src/interpreter/Parser';
import { ITokenizer, Tokenizer } from 'src/interpreter/Token';
import { IContextual } from '../@lib/Context';
import { Utility } from '../@lib/Utility';

export class VTI {
  public constructor(
    private tokenizer: ITokenizer = new Tokenizer(),
    private parser: IParser = new Parser(),
  ) {}
  
  public encode<
    Model, 
    _Interpreter extends IInterpreter<Model>
  > (text: string, filepath: string, interpreter: _Interpreter): Model[] {
    return interpreter.encode(
      this.parser.encode(
        this.tokenizer.encode(text, { filepath, line: 0, column: 0 })
      )
    );
  }

  public encodeFile<Model> (filepath: string, interpreter: IInterpreter<Model>): Model[] {
    const text = FileSystem.readFileSync(filepath);
    return this.encode(text.toString(), filepath, interpreter);
  }

  public encodeFolder<Model> (folderpath: string, interpreter: IInterpreter<Model>): Model[] {
    const files = FileSystem.readdirSync(folderpath);
    const results: Model[] = [];
    for (const file of files) {
      const filepath = `${folderpath}/${file}`;
      const result = this.encodeFile(filepath, interpreter);
      results.push(...result);
    }
    return results;
  }

  public decode<Model> (models: Model[], filepath: string, interpreter: IInterpreter<Model>): string {
    return this.tokenizer.decode(
      this.parser.decode(
        interpreter.decode(
          models, { filepath, line: 0, column: 0 }
        ) as Ast
      )
    );
  }

  public decodeToFile<Model> (models: unknown[], filepath: string, interpreter: IInterpreter<Model>) {
    const text = this.decode(models, filepath, interpreter);
    const folder = filepath.split('/').slice(0, -1).join('/');
    if (!FileSystem.existsSync(folder))
      FileSystem.mkdirSync(folder, { recursive: true });

    FileSystem.writeFileSync(filepath, text);
  }

  public decodeToFolder<Model extends IContextual & IPersistent> (
    models: Model[], 
    interpreter: IInterpreter<Model>,
    config: AppConfig
  ) {
    const filepaths = Utility.Unique(models.map(model => model.context.filepath));
    for (const filepath of filepaths) {
      const file = models.filter(model => model.context.filepath === filepath);
      if (file.some(model => model.isModified() === true)) {
        const outPath = filepath.replace(config.victoria3Path, config.outputDir);
        this.decodeToFile(file, outPath, interpreter);
      }
    }
  }
}