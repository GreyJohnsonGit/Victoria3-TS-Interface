import * as basic from '@test-data/basic.json';
import { Ast, Parser } from 'interpreter/Parser';
import { isTokenSymbolic, Token } from 'interpreter/Token';

describe('Parser', () => {
  it('Should encode tokens without loss', () => {
    // Arrange
    const { tokens, ast } = basic;
    const parser = new Parser();

    // Act
    const encoded = parser.encode(tokens as Token[]);

    // Assert
    expect(encoded).toEqual(ast);

  });

  it('Should decode ast with only context loss', () => {
    // Arrange
    const { tokens, ast } = basic;
    const parser = new Parser();

    // Act
    const decoded = parser.decode(ast as Ast);

    // Assert
    expect(removeContext(decoded)).toEqual(removeContext(tokens as Token[]));
  });

  it('Should encode and decode with only context loss', () => {
    // Arrange
    const { tokens } = basic;
    const parser = new Parser();

    // Act
    const encoded = parser.encode(tokens as Token[]);
    const decoded = parser.decode(encoded);

    // Assert
    expect(removeContext(decoded)).toEqual(removeContext(tokens as Token[]));
  });

  it('Should decode and encode without loss', () => {
    // Arrange
    const { ast } = basic;
    const parser = new Parser();

    // Act
    const decoded = parser.decode(ast as Ast);
    const encoded = parser.encode(decoded);

    // Assert
    expect(encoded).toEqual(ast);
  });
});

function removeContext(tokens: Token[]) {
  return tokens.map(t => ({ kind: t.kind, value: isTokenSymbolic(t) ? t.value : undefined }));
}