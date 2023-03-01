export type BuildingId = Lowercase<`building_${string}`>;

export function isBuildingId(value: unknown): value is BuildingId {
  return typeof value === 'string' && value.split('building_')[0] === '';
}
