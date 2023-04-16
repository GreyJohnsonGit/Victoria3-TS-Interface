import { z } from 'zod';

export type Option<T> = Some<T> | None<T>;

export interface Some<T> extends OptionMethods<T> {
  kind: 'Some';
  value: NonNullable<T>;
}

export interface None<T> extends OptionMethods<T> {
  kind: 'None';
}

export interface OptionMethods<T> {
  unwrapOr(defaultValue: T): T;
  map<R>(onSome: (value: T) => Option<R> | void): Option<R> | void;
  isSome(): this is Some<T>;
  isNone(): this is None<T>;
  equals(other: NonNullable<T>): boolean;
  match<R>(arms: MatchArms<T, R>): R;
}

type MatchArms<T, R> = { 
  ['Some']: (value: T) => R; 
  ['None']: () => R; 
};

export const Optional = <T>(typeValue: z.ZodType<T>) => 
  z.custom<Option<T>>(value => {
    if (isOption<T>(value) && value.isSome()) {
      return typeValue.safeParse(value.value).success;
    }

    if (isOption<T>(value) && value.isNone())
      return true;
  });
export function isOption<T>(value: unknown): value is Option<T> {
  return isSome(value) || isNone(value);
}

export function isSome<T>(value: unknown): value is Some<T> {
  const option = value as Option<T>;
  return typeof option === 'object' && option.kind === 'Some';
}

export function isNone<T>(value: unknown): value is None<T> {
  const option = value as Option<T>;
  return typeof option === 'object' && option.kind === 'None';
}

export function Option<T>(value: T): Option<T> {
  return value !== null && value !== undefined ? Some(value) : None();
}

export function None<T>(): None<T> {
  return { 
    kind: 'None',
    unwrapOr(defaultValue: T) {
      return defaultValue;
    },
    map: () => None(),
    isSome: () => false,
    isNone: () => true,
    equals: (value: T) => value === undefined,
    match: <R>(arms: MatchArms<T, R>) => arms.None(),
  };
}

export function Some<T>(value: NonNullable<T>): Some<T> {
  return {
    kind: 'Some',
    value,
    unwrapOr: () => value,
    map: <R>(onSome: (value: T) => Option<R>) => onSome(value),
    isSome: () => true,
    isNone: () => false,
    equals: (other: T) => value === other,
    match: <R>(arms: MatchArms<T, R>) => arms.Some(value),
  };
}