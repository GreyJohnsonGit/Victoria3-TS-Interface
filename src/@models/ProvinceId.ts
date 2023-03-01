import { GuardReturnType } from '@lib/Validator';

export type ProvinceId = GuardReturnType<typeof isProvinceId>;

export function isProvinceId(value: unknown): value is `x${string}` {
  return typeof value === 'string' && value.match(/^x[A-F0-9]{6}$/) !== null;
}

type HexCharacter = '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'A' | 'B' | 'C' | 'D' | 'E' | 'F';
export type HexSegment = `${HexCharacter}${HexCharacter}`;
export function x(s1: HexSegment, s2: HexSegment, s3: HexSegment): ProvinceId {
  return `x${s1}${s2}${s3}` as string as ProvinceId;
}