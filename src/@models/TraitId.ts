import * as TraitIdsJson from '@generated-definitions/TraitId.json';

export function isTraitId(value: unknown): value is TraitId {
  return Object.keys(TraitIdsJson).includes(value as string);
}

export type TraitId = keyof typeof TraitIdsJson;