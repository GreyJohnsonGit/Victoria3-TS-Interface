import { StateRegionExt } from '@lib/StateRegionExt';
import * as FileSystem from 'fs';
import { Jomini } from 'jomini';
import { Some } from './@lib/Option';

async function main() {
  const jomini = await Jomini.initialize();

  const data = FileSystem.readFileSync('../../Steam/steamapps/common/Victoria 3/game/map_data/state_regions/00_west_europe.txt', 'utf8');
  const result = jomini.parseText(data);
  
  const rawStateRegions = Object.entries(result);
  const stateRegion = StateRegionExt.fromJSON(rawStateRegions[0][0], rawStateRegions[0][1]);
  
  const backToJSON = jomini.write(writer => 
    stateRegion.match({
      Some: (stateRegion) => StateRegionExt.write(Some(stateRegion), writer),
      None: () => undefined,
    })
  );

  const asString = String.fromCharCode(...backToJSON);

  stateRegion.match({
    Some: (stateRegion) => stateRegion.cappedResources.kind === 'Some' && console.log(stateRegion.cappedResources.value),
    None: () => undefined,
  });

  FileSystem.writeFileSync('result.json', asString, 'utf8');
}

main();