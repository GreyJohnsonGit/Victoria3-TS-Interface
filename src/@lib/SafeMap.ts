/**
 * `Map` wrapper for Maps that guarantee the existence of all keys.
 */
export class SafeMap<K, V> extends Map<K, V> {
  public get(key: K): V {
    const value = super.get(key);
    if (value === undefined) {
      throw new Error(`Key ${key} not found`);
    }

    return value;
  }

  public setFields(key: K, value: Partial<V>): this {
    const existingValue = this.get(key);
    this.set(key, { ...existingValue, ...value });
    return this;
  }
}