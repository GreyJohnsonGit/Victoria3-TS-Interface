export interface IContextual {
  context: Context;
}

export interface Context {
  filepath: string;
  line: number;
  column: number;
}

export function toLink(context: Context): string {
  return `${context.filepath}:${context.line}:${context.column}`;
}

export function isContext(value: unknown): value is Context {
  if (typeof value !== 'object' || value === null) {
    return false;
  }

  const {filepath, line, column} = value as Context;
  return typeof filepath === 'string' && 
    typeof line === 'number' && 
    typeof column === 'number';
}