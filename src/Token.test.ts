import * as basic from './test_cases/basic.json';
import { tokenize } from './Token';

describe('tokenize', () => {
  it('should tokenize a string', () => {
    // Arrange
    const context = { filepath: 'test', line: 1, column: 0 };
    const input = basic.raw;

    // Act
    const tokens = tokenize(input, context);

    // Assert
    const expected = basic.tokenized;
    expect(tokens.length).toBe(expected.length);
    expect(tokens).toEqual(expected);
  });
});