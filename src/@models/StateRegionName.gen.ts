import { IModelGenerator } from 'generator/IModelGenerator';
import { SimpleIdInterpreter } from 'src/generator/SimpleIdInterpreter';

const filepath = 'game/map_data/state_regions/';

export class StateRegionNameGenerator implements IModelGenerator<string> {
  constructor(
    public interpreter = new SimpleIdInterpreter<string>(),
    public options = { 
      gameFilepath: filepath,
      generatorFilePath: __filename, 
      isFolder: true
    }
  ) {}

  public format(stateRegionNames: string[]) {
    const names: Record<string, object> = {};
    stateRegionNames.forEach(id => names[id] = {});
    return JSON.stringify(names, null, 2);
  }
}