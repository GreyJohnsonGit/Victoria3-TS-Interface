import { parse } from './Parser';
import * as basic from './test_cases/basic.json';
import { Token } from './Token';

describe('parse', () => {
  it('should parse a token stream', () => {
    // Arrange
    const tokens = basic.tokenized as Token[];

    // Act
    const result = parse(tokens);

    // Assert
    expect(result).toEqual(basic.parsed);
  });
});