import { promises as FileSystem } from 'fs';
import { AppConfig } from './AppConfig';
import { Context, toLink } from './Context';
import { ExpectedNumber, ExpectedRawCappedResource, ExpectedString, ExpectedStringArray } from './Error';
import { quote } from './Utility';
import { validateNumber, validateRawCappedResource, validateString, validateStringArray } from './Validator';
import { interpret } from './VTI';

export type RawCappedResource = ([string, number]|[string, number, boolean, string | undefined])[];

export interface CappedResource {
  resource: string;
  cap: number;
  hidden: boolean;
  depleted?: string;
}

function CappedResource(raw: RawCappedResource): CappedResource[] {
  return raw.map(r => {
    const [resource, cap, hidden, depleted] = r;
    return {
      resource,
      cap,
      hidden: hidden ?? false,
      depleted
    };
  });
}

export interface StateRegion {
  name: string;
  id: number;
  provinces: string[];

  subsistence_building?: string;
  traits?: string[];
  city?: string;
  port?: string;
  farm?: string;
  mine?: string;
  wood?: string;
  arable_land?: number;
  arable_resources?: string[];
  prime_land?: string[];
  capped_resources?: CappedResource[];
  naval_exit_id?: number;
  impassable?: string[];
  context?: Context;
}

export interface SeaStateRegion {
  name: string;
  id: number;
  provinces: string[];
  
  impassable?: string[];
  context?: Context;
}

function validate(cx: Context, rawStateRegion: Record<string, unknown>): StateRegion {
  const stateRegion: Partial<StateRegion> =  {};
  Object.keys(rawStateRegion).forEach((key) => { 
    const value = rawStateRegion[key];
    switch (key) {
    case 'name':
    case 'subsistence_building':
    case 'city':
    case 'port':
    case 'farm':
    case 'mine':
    case 'wood':
      if (validateString(value)) {
        stateRegion[key] = value;
      } else {
        throw ExpectedString(key, value, cx);
      }
      break;
    case 'id':
    case 'arable_land':
    case 'naval_exit_id':
      if (validateNumber(value)) {
        stateRegion[key] = value;
      } else {
        throw ExpectedNumber(key, value, cx);
      }
      break;
    case 'provinces':
    case 'traits':
    case 'arable_resources':
    case 'impassable':
    case 'prime_land':
      if (validateStringArray(value)) {
        stateRegion[key] = value;
      } else {
        throw ExpectedStringArray(key, value, cx);
      }
      break;
    case 'capped_resources':
      
      if (validateRawCappedResource(value)) {
        stateRegion[key] = CappedResource(value);
      } else {
        throw ExpectedRawCappedResource(key, value, cx);
      }
      break;
    default:
      throw new Error(`Unexpected key ${key} at ${toLink(cx)}`);
    }
  });
  stateRegion.context = cx;
  return stateRegion as StateRegion;
}

function set(key: string, value: unknown, stateRegion: Record<string, unknown>) {
  if (key === 'capped_resources' || key === 'resource') {
    stateRegion['capped_resources'] = stateRegion['capped_resources'] ?? [];
    
    if (key === 'capped_resources') {
      stateRegion[key] = [stateRegion[key], value].flat(1);
    }

    if (key === 'resource' && Array.isArray(value)) {
      const entry = [
        value.find(([key]) => key === 'type')?.[1],
        value.find(([key]) => key === 'undiscovered_amount')?.[1] ?? 
        value.find(([key]) => key === 'discovered_amount')?.[1],
        value.find(([key]) => key === 'discovered_amount') === undefined,
        value.find(([key]) => key === 'depleted_type')?.[1]
      ];

      stateRegion['capped_resources'] = [
        stateRegion['capped_resources'], 
        [entry]
      ].flat(1);
    }
    return;
  }

  stateRegion[key] = value;
}

export function stateRegionBuilder() {
  let stateRegion: Record<string, unknown> = {};
  
  const builder = () => ({
    validate: (cx: Context) => {
      const result = validate(cx, stateRegion);
      stateRegion = {};
      return result;
    },
    set: (key: string, value: unknown) => {
      set(key, value, stateRegion);
      return builder();
    }
  });

  return builder();
}

export function toString(stateRegion: StateRegion) {
  const sr = stateRegion;
  const hiddenCapped = sr.capped_resources?.filter(r => r.hidden) ?? [];
  const knownCapped = sr.capped_resources?.filter(r => !r.hidden) ?? [];
  sr.capped_resources?.forEach(r => {
    if (r.hidden) {
      hiddenCapped.push(r);
    } else {
      knownCapped.push(r);
    }
  });
  return `${sr.name} = {
  id = ${sr.id}
  provinces = { ${sr.provinces.map(quote).join(' ')} }
${sr.prime_land ? 
    `  prime_land = { ${sr.prime_land.map(quote).join(' ')} }` : ''
}
${sr.subsistence_building ? 
    `  subsistence_building = "${sr.subsistence_building}"` : ''
}
${sr.traits ? 
    `  traits = { ${sr.traits.map(quote).join(' ')} }` : ''
}
${sr.city ? 
    `  city = "${sr.city}"` : ''
}
${sr.port ? 
    `  port = "${sr.port}"` : ''
}
${sr.farm ? 
    `  farm = "${sr.farm}"` : ''
}
${sr.mine ? 
    `  mine = "${sr.mine}"` : ''
}
${sr.wood ? 
    `  wood = "${sr.wood}"` : ''
}
${sr.arable_land ? 
    `  arable_land = ${sr.arable_land}` : ''
}
${sr.arable_resources ? 
    `  arable_resources = { ${sr.arable_resources.map(quote).join(' ')} }` : ''
}
${knownCapped.length > 0 ? 
    `  capped_resources = {
    ${knownCapped.map(r => `${r.resource} = ${r.cap}`).join('\n    ')}
  }` : ''
}
${hiddenCapped.length > 0 ? hiddenCapped.map(r => `
      resource = {
        type = "${r.resource}"
        undiscovered_amount = ${r.cap}
${r.depleted !== undefined ? 
    `   depleted_type = "${r.depleted}"` : ''
}
      }\n`): ''
}
}`;
}

const inPath = (config: AppConfig) => `${config.victoria3Path}/game/map_data/state_regions`;
const outPath = (config: AppConfig) => `${config.outputDir}/game/map_data/state_regions`;

export async function loadStateRegions(config: AppConfig): Promise<StateRegion[]> {
  const fileNames = await FileSystem.readdir(inPath(config));
  const stateRegions = fileNames.map(async (fileNames) => {
    const filePath = `${inPath(config)}/${fileNames}`;
    const fileContents = await FileSystem.readFile(filePath, 'utf8');
    const stateRegion = stateRegionBuilder();
    return interpret(fileContents, filePath, stateRegion);
  });
  return (await Promise.all(stateRegions)).flat();
}

export async function saveStateRegions(config: AppConfig, stateRegions: StateRegion[]) {
  await FileSystem.mkdir(outPath(config), { recursive: true });
  await Promise.all(stateRegions.map(async (stateRegion) => {
    const filePath = `${outPath(config)}/${stateRegion.name}.txt`;
    const fileContents = toString(stateRegion);
    await FileSystem.appendFile(filePath, fileContents, 'utf8');
  }));
}
    