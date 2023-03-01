
// Praise https://code.lol/post/programming/type-guard-composition/
export type Guard<T = unknown> = (x: unknown) => x is T;
export type GuardReturnType<T extends Guard> = T extends Guard<infer U> ? U : never;
export type Key = string | number | symbol;
type GuardRecord = Record<Key, Guard>;

export function isUnknown(value: unknown): value is unknown {
  return true;
}

export function isDefined<T>(value: T): value is NonNullable<T> {
  return value !== undefined && value !== null;
}

export function isOptional<T>(isT: Guard<T>): Guard<T | undefined> {
  return (value: unknown): value is T | undefined => 
    value === undefined || isT(value);
}

export function isUndefined(value: unknown): value is undefined {
  return value === undefined;
}

export function isEmpty(value: unknown): value is Record<string, never> {
  return typeof value === 'object' && value !== null && Object.keys(value).length === 0;
}

export function isString(value: unknown): value is string {
  return typeof value === 'string';
}
  
export function isNumber(value: unknown): value is number {
  return typeof value === 'number';
}
  
export function isBoolean(value: unknown): value is boolean {
  return typeof value === 'boolean';
}
  
export function isArray<T>(isT: Guard<T>): Guard<T[]> {
  return (value: unknown): value is T[] => 
    Array.isArray(value) && value.every(isT);
}

export function isObject<T extends GuardRecord>(guards: T) {
  return (
    value: unknown, 
    options?: { 
      onInvalid?: (key: keyof T, value: unknown) => void;
    }
  ): value is { [key in keyof T]: GuardReturnType<T[key]> } => {
    if (typeof value !== 'object' || value === null) {
      return false;
    }

    let hasAllValidFields = true;
    Object.entries(value).forEach(([key, value]) => {
      if (!guards[key](value)) {
        options?.onInvalid?.(key, value);
        hasAllValidFields &&= false;
      }
    });
    return hasAllValidFields;
  };
}

export function onInvalid<T>(isT: Guard<T>, onInvalid: () => void
): Guard<T> {
  return (value: unknown): value is T => {
    if (isT(value)) {
      return true;
    } else {
      onInvalid();
      return false;
    }
  };
}

export function isIdFactory<Id extends string> (
  ids: Record<Id, unknown>
): Guard<Id> {
  return (id: unknown): id is Id => {
    return typeof id === 'string' && 
      Object.keys(ids).includes(id as string);
  };
}