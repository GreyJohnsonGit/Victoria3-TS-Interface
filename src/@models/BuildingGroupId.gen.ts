import { IModelGenerator } from 'generator/IModelGenerator';
import { SimpleIdInterpreter } from 'generator/SimpleIdInterpreter';

const filepath = 'game/common/building_groups';

export class BuildingGroupIdGenerator implements IModelGenerator<string> {
  constructor(
    public interpreter = new SimpleIdInterpreter<string>(),
    public options = { 
      gameFilepath: filepath,
      generatorFilePath: __filename, 
      isFolder: true
    }
  ) {}

  public format(buildingGroupIds: string[]) {
    const ids: Record<string, object> = {};
    buildingGroupIds.forEach(id => ids[id] = {});
    return JSON.stringify(ids, null, 2);
  }
}