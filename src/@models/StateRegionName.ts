import * as StateRegionNamesJson from '@generated-definitions/StateRegionName.json';

export function isStateRegionName(value: unknown): value is StateRegionName {
  return Object.keys(StateRegionNamesJson).includes(value as string);
}

export type StateRegionName = keyof typeof StateRegionNamesJson;