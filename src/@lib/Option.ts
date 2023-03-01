import { isDefined } from '@lib/Validator';

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
  isSome(): this is Some<T>;
  isNone(): this is None<T>;
  equals(other: NonNullable<T>): boolean;
  match<R>(arms: MatchArms<T, R>): R;
}

type MatchArms<T, R> = { 
  ['Some']: (value: NonNullable<T>) => R; 
  ['None']: () => R; 
};

export function isOption<T>(value: unknown): value is Option<T> {
  return isSome(value) || isNone(value);
}

export function isSome<T>(value: unknown): value is Some<T> {
  const option = value as Option<T>;
  return option.kind === 'Some';
}

export function isNone<T>(value: unknown): value is None<T> {
  const option = value as Option<T>;
  return option.kind === 'None';
}

export function Option<T>(value: T): Option<T> {
  return isDefined(value) ? Some(value) : None();
}

export function None<T>(): None<T> {
  return { 
    kind: 'None',
    unwrapOr(defaultValue: T) {
      return defaultValue;
    },
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
    isSome: () => true,
    isNone: () => false,
    equals: (other: T) => value === other,
    match: <R>(arms: MatchArms<T, R>) => arms.Some(value),
  };
}