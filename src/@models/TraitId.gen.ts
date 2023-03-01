import { IModelGenerator } from 'generator/IModelGenerator';
import { SimpleIdInterpreter } from 'generator/SimpleIdInterpreter';

const filepath = 'game/common/state_traits';

export class TraitIdFactory implements IModelGenerator<string> {
  constructor(
    public interpreter = new SimpleIdInterpreter<string>(),
    public options = { 
      gameFilepath: filepath,
      generatorFilePath: __filename, 
      isFolder: true
    }
  ) {}

  public format(traitIds: string[]) {
    const ids: Record<string, object> = {};
    traitIds.forEach(id => ids[id] = {});
    return JSON.stringify(ids, null, 2);
  }
}