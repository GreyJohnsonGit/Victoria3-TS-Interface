export function quote(printable: unknown) {
  return `"${printable}"`;
}

export class Utility {
  public static Unique<T>(array: T[]): T[] {
    return [...new Set(array)];
  }
}

export type OptionalKeys<T> = keyof {[K in keyof T as (undefined extends T[K] ? K : never)]: T[K]}
export type RequiredKeys<T> = keyof {[K in keyof T as (undefined extends T[K] ? never : K)]: T[K]}