import { Context } from '@lib/Context';
import { InvalidNumberToken, InvalidSymbolToken, InvalidToken, UnexpectedToken, UnfinishedHexStringToken, UnfinishedStringToken } from '@lib/Error';

export interface ITokenizer {
  encode(text: string, cx: Context): Token[];
  decode(tokens: Token[]): string;
}

export class Tokenizer implements ITokenizer {
  constructor() {}

  public encode(text: string, cx: Context): Token[] {
    const tokens: Token[] = [
      Token(cx, 'symbol', '#GLOBAL'),
      Token(cx, '='),
      Token(cx, '{')
    ];
  
    for (const line of text.split('\n')) {
      while (cx.column < line.length) {
        const character = line.charAt(cx.column); 
  
        if (character.match(/\s/)) {
          cx.column++;
          continue;
        }
  
        if (character === '{') {
          tokens.push(Token(cx, '{'));
          cx.column++;
          continue;
        }
  
        if (character === '}') {
          tokens.push(Token(cx, '}'));
          cx.column++;
          continue;
        }
  
        if (character === '=') {
          tokens.push(Token(cx, '='));
          cx.column++;
          continue;
        }
  
        if (character === '#') {
          break;
        }
  
        if (character === '"') {
          const [token, length] = this.encodeString(cx, line);
          cx.column += length;
          tokens.push(token);
          continue;
        }
  
        if (character === 'x') {
          const [token, length] = this.encodeHexString(cx, line);
          cx.column += length;
          tokens.push(token);
          continue;
        }
  
        if (character.match(/[\d+-.]/)) {
          const [token, length] = this.encodeNumber(cx, line);
          cx.column += length;
          tokens.push(token);
          continue;
        }
  
        if (character.match(/[a-zA-Z_]/)) {
          const [token, length] = this.encodeSymbol(cx, line);
          cx.column += length;
          tokens.push(token);
          continue;
        }
  
        throw UnexpectedToken(character, cx);
      }
      cx.line++;
      cx.column = 0;
    }
  
    tokens.push(Token(cx, '}'));
  
    return tokens;
  }

  public decode(tokens: Token[]): string {
    let indent = 0;
    return tokens.slice(3, -1).map(token => {
      const spaces = (indent: number) => '  '.repeat(indent);
      if (token.kind === 'string')
        return `"${token.value}" `;
  
      if (token.kind === 'number')
        return `${token.value} `;
  
      if (token.kind === 'symbol')
        return `\n${spaces(indent)}${token.value}`;
  
      if (token.kind === '{') { 
        indent++;
        return `{\n${spaces(indent)}`;
      }
      
      if (token.kind === '}') {
        indent--;
        return `\n${spaces(indent)}}`;
      }
  
      if (token.kind === '=')
        return ' = ';
    }).join('');
  }

  private encodeString(
    cx: Context, 
    text: string
  ): [Token, number] {
    const symbol = text.slice(cx.column).match(/^".*?"/)?.[0];
  
    if (symbol === undefined) {
      throw UnfinishedStringToken(cx);
    }
  
    const value = symbol.replaceAll('"', '');
    const token = Token(cx, 'string', value);
    return [token, symbol.length];
  }
  
  private encodeHexString(
    cx: Context,
    text: string
  ): [Token, number] {
    const symbol = text.slice(cx.column).match(/^x[\dA-F]{6}/)?.[0];
  
    if (symbol === undefined) {
      throw UnfinishedHexStringToken(cx);
    }
  
    const token = Token(cx, 'string', symbol);
    return [token, symbol.length];
  }
  
  private encodeNumber(
    cx: Context, 
    text: string, 
  ): [Token, number] {
    const symbol = text.slice(cx.column).match(/^[+-]?\s*\d*\.?\d+/)?.[0];
    const value = parseFloat(symbol ?? '');
    
    if (symbol === undefined || isNaN(value)) {
      throw InvalidNumberToken(cx);
    }
  
    const token = Token(cx, 'number', value);
    return [token, symbol.length];
  }
  
  private encodeSymbol(
    cx: Context, 
    text: string, 
  ): [Token, number] {
    const symbol = text.slice(cx.column).match(/^[a-zA-Z_][a-zA-Z0-9_]*/)?.[0];
  
    if (symbol === undefined) {
      throw InvalidSymbolToken(cx);
    }
  
    const token = Token(cx, 'symbol', symbol);
    return [token, symbol.length];
  }
}

export type Token = SymbolicToken<Symbolic> | LiteralToken<Literal>;

export interface SymbolicToken<Kind extends Symbolic> {
  context: Context;
  kind: Kind;
  value: SymbolicValueType[Kind];
}

export interface LiteralToken<Kind extends Literal> {
  context: Context;
  kind: Kind;
}

export type TokenKind = Symbolic | Literal;

export type Symbolic = 'number' | 'string' | 'symbol';
export type Literal = '{' | '}' | '=' | '#';

interface SymbolicValueType {
  number: number;
  string: string;
  symbol: string;
}

export function Token<K extends keyof SymbolicValueType>(
  cx: Context,
  kind: K, 
  value: SymbolicValueType[K]): Token;
export function Token<K extends Literal>(
  cx: Context,
  kind: K): Token;
export function Token<K extends TokenKind>(
  cx: Context,
  kind: K, 
  value?: SymbolicValueType[keyof SymbolicValueType] | undefined): Token
{
  const symbolic = { 
    kind, 
    value, 
    context: {...cx}
  } as SymbolicToken<Symbolic>;
  if (value !== undefined && isTokenSymbolic(symbolic)) {
    return symbolic;    
  }

  const literal = {
    kind,
    context: {...cx}
  } as LiteralToken<Literal>;
  if (isTokenLiteral(literal)) {
    return literal;
  }

  throw InvalidToken(kind, value, cx);
}

export function isTokenLiteral(token: Token, literal?: Literal): token is LiteralToken<Literal> {
  const { kind } = token;
 
  if (literal !== undefined) {
    return kind === literal;
  }  

  return kind === '{' || kind === '}' || kind === '=' || kind === '#';
}

export function isTokenSymbolic(token: Token): token is SymbolicToken<Symbolic> {
  const { kind } = token;
  return kind === 'number' || kind === 'string' || kind === 'symbol';
}


export function isNumber(token: Token): token is SymbolicToken<'number'> {
  return token.kind === 'number';
}

export function isString(token: Token): token is SymbolicToken<'string'> {
  return token.kind === 'string';
}

export function isSymbol(token: Token): token is SymbolicToken<'symbol'> {
  return token.kind === 'symbol';
}