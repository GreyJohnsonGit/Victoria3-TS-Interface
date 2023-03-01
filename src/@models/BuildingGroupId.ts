import * as BuildingGroupIdsJson from '@generated-definitions/BuildingGroupId.json';

export function isBuildingGroupId(value: unknown): value is BuildingGroupId {
  return Object.keys(BuildingGroupIdsJson).includes(value as string);
}

export type BuildingGroupId = keyof typeof BuildingGroupIdsJson;