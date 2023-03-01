import { HexSegment, isProvinceId, x } from '@models/ProvinceId';

describe('ProvinceId', () => {
  it('Should always produce valid ids using \'x\'', () => {
    // Arrange
    const hexDigits = '0123456789ABCDEF'.split('');
    const hexSegments = hexDigits.flatMap(d1 =>
      hexDigits.flatMap(d2 => `${d1}${d2}`)
    ) as HexSegment[];

    // Act
    const isValid = hexSegments.flatMap(s1 =>
      hexSegments.flatMap(s2 => 
        isProvinceId(x(s1, s2, s1[0] + s2[0] as HexSegment))
      ),
    );

    // Assert
    expect(isValid.every(v => v)).toBe(true);
  });
});