import * as basic from '@test-data/basic.json';
import { Token, Tokenizer } from 'interpreter/Token';

describe('Tokenizer', () => {
  it('Should accurately encode string', () => {
    // Arrange
    const context = { filepath: 'test', line: 0, column: 0 };
    const { text, tokens } = basic;
    const tokenizer = new Tokenizer();

    // Act
    const encoded = tokenizer.encode(text, context);

    // Assert
    expect(encoded.length).toBe(tokens.length);
    expect(encoded).toEqual(tokens);
  });

  it('Should accurately decode tokens', () => {
    // Arrange
    const { text, tokens } = basic;
    const tokenizer = new Tokenizer();

    // Act
    const decoded = tokenizer.decode(tokens as Token[]);

    // Assert
    expect(decoded).toEqual(text);
  });

  it('Should decode and encode without loss', () => {
    // Arrange
    const context = { filepath: 'test', line: 0, column: 0 };
    const { tokens } = basic;
    const tokenizer = new Tokenizer();

    // Act
    const decoded = tokenizer.decode(tokens as Token[]);
    const encoded = tokenizer.encode(decoded, context);

    // Assert
    expect(encoded).toEqual(tokens);
  });

  it('Should encode and decode without loss', () => {
    // Arrange
    const context = { filepath: 'test', line: 0, column: 0 };
    const { text } = basic;
    const tokenizer = new Tokenizer();

    // Act
    const encoded = tokenizer.encode(text, context);
    const decoded = tokenizer.decode(encoded as Token[]);

    // Assert
    expect(decoded).toEqual(text);
  });
});