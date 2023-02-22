import { RawCappedResource, SeaStateRegion, StateRegion } from './StateRegion';

export function validateString(value: unknown): value is string {
  return typeof value === 'string';
}

export function validateNumber(value: unknown): value is number {
  return typeof value === 'number';
}

export function validateStringArray(value: unknown): value is string[] {
  return Array.isArray(value) && value.every(validateString);
}

export function validateRawCappedResource(value: unknown): value is RawCappedResource {
  const raw = value as RawCappedResource;
  return Array.isArray(raw) && raw.every(([resource, cap]) => validateString(resource) && validateNumber(cap));
}

export function validateSeaStateRegion(value: StateRegion): value is SeaStateRegion {
  if (typeof value !== 'object' || value === null) {
    return false;
  }

  type Key = keyof SeaStateRegion;
  const expectedKeys: Key[] = ['name', 'id', 'provinces'];
  const optionalKeys: Key[] = ['impassable', 'context'];
  const keys = Object.keys(value);

  const hasExtraKeys = keys.find(key => 
    !expectedKeys.includes(key as Key) &&
    !optionalKeys.includes(key as Key)
  ) !== undefined;

  const hasMissingKeys = expectedKeys.find(key => 
    !keys.includes(key)
  ) !== undefined;

  return !hasExtraKeys && !hasMissingKeys;
}