import { isBuildingId } from '@models/BuildingId';

describe('BuildingId', () => {
  test.each([
    ['building_coal_mine', 'valid'],
    ['building_eiffel_tower', 'valid'],
    ['building_cotton_plantation', 'valid'],
    ['coal_mine', 'invalid'],
    ['_building_coal_mine', 'invalid'],
  ])('Should mark %s as %s', (id, validity) => {
    // Act + Assert
    if (isBuildingId(id)) {
      expect(validity).toBe('valid');
    } else {
      expect(validity).toBe('invalid');
    }
  });
});