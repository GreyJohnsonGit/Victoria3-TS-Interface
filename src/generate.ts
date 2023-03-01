import { loadConfig } from '@lib/AppConfig';
import { BuildingGroupIdGenerator } from '@models/BuildingGroupId.gen';
import { StateRegionNameGenerator } from '@models/StateRegionName.gen';
import { TraitIdFactory } from '@models/TraitId.gen';
import * as FileSystem from 'fs';
import { IModelGenerator } from 'generator/IModelGenerator';
import { VTI } from 'interpreter/VTI';

async function generateModel<Model>(modelGenerator: IModelGenerator<Model>) {
  const {
    interpreter,
    format,
    options: { gameFilepath, generatorFilePath, isFolder },
  } = modelGenerator;
  const config = loadConfig();
  const path = `${config.victoria3Path}/${gameFilepath}`;
  const writeTo = generatorFilePath
    .replace('build\\@models', 'src\\@generated-definitions')
    .replace('.gen.js', '.json');

  const vti = new VTI();

  
  const fileName = writeTo.split('\\').pop();
  console.log(`Generating ${fileName}...`);
  
  let models = [] as Model[];
  if (isFolder) {
    models = vti.encodeFolder(path, interpreter);
  } else {
    models = vti.encodeFile(path,interpreter);
  }

  FileSystem.writeFileSync(writeTo, format(models));
  console.log(`${fileName} Complete!`);

}

if (require.main === module) {
  if (!FileSystem.existsSync('src/@generated-definitions'))
    FileSystem.mkdirSync('src/@generated-definitions');

  generateModel(new BuildingGroupIdGenerator());
  generateModel(new TraitIdFactory());
  generateModel(new StateRegionNameGenerator());
}