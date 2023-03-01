/* eslint-disable @typescript-eslint/no-explicit-any */
import { AppConfig, loadConfig } from '@lib/AppConfig';
import { StateRegion, StateRegionInterpreter } from '@models/StateRegion';
import { Parser } from 'interpreter/Parser';
import { Tokenizer } from 'interpreter/Token';
import { VTI } from 'interpreter/VTI';

describe('StateRegion', () => {
  let config: AppConfig;
  let stateRegions: StateRegion[];
  let vti: VTI;
  let interpreter: StateRegionInterpreter;

  it('Should load state regions', async () => {
    // Arrange
    interpreter = new StateRegionInterpreter();
    vti = new VTI(
      new Tokenizer(), 
      new Parser()
    );
    config = loadConfig();

    // Act
    stateRegions = vti.encodeFolder<StateRegion>(
      interpreter.path(config), interpreter
    ).flat();
    
    // Assert
    expect(stateRegions).toBeDefined();
    expect(stateRegions.length).toBe(774);
  });

  it.each([
    ['STATE_ARCTIC_OCEAN_01', 3037, ['x39CACA'], ['x39CACA']],
    ['STATE_BLACK_SEA', 3036, ['x48FAFA'], undefined],
  ])('should find sea state region %s', async (name, id, provinces, impassable) => {
    // Act
    const result = stateRegions.find(r => r.get('id') === id);
    if (result === undefined) {
      throw Error(`Could not find state region ${name} with id ${id}`);
    }

    // Assert
    expect(result.get('name')).toBe(name);
    expect(result.get('id')).toBe(id);
    expect(result.get('provinces')).toEqual(provinces);
    expect(result.getOptional('impassable').unwrapOr(undefined)).toEqual(impassable);
  });

  it.each([
    [
      'STATE_MINAS_GERAIS',
      391,
      'building_subsistence_farms',
      'xE8109E',
      'x0B09E1',
      'x80F01D',
      'x755D3F',
      240,
      [
        'bg_maize_farms', 'bg_livestock_ranches', 'bg_coffee_plantations', 
        'bg_cotton_plantations', 'bg_tobacco_plantations', 
        'bg_sugar_plantations', 'bg_banana_plantations'
      ],
      [
        { resource: 'bg_gold_fields', cap: 8, hidden: true, depleted: 'bg_gold_mining' },
        { resource: 'bg_rubber', cap: 7, hidden: false, depleted: undefined },
        { resource: 'bg_oil_extraction', cap: 5, hidden: true, depleted: undefined },
        { resource: 'bg_gold_mining', cap: 2, hidden: false, depleted: undefined },
        { resource: 'bg_iron_mining', cap: 45, hidden: false, depleted: undefined },
        { resource: 'bg_lead_mining', cap: 30, hidden: false, depleted: undefined },
        { resource: 'bg_logging', cap: 24, hidden: false, depleted: undefined },
      ],
      [ 
        'x00476A', 'x007644', 'x0341D9', 'x08DE56', 'x09A757', 'x0B09E1', 
        'x0B9B88', 'x0FD8DD', 'x145524', 'x154455', 'x1572E5', 'x1A0824', 
        'x1E0C88', 'x1E3689', 'x1ECD15', 'x1F4798', 'x205DD7', 'x2445CC', 
        'x249745', 'x2D9C6A', 'x2F1439', 'x2F1BB1', 'x2F544F', 'x32D141', 
        'x332B5D', 'x38EAE6', 'x3A1260', 'x3E5151', 'x3E98D6', 'x403955', 
        'x410EC7', 'x45CBB7', 'x471944', 'x526B6D', 'x52D36E', 'x53DAE2', 
        'x54AA39', 'x55459C', 'x565A2A', 'x57421B', 'x59E08F', 'x5F78DD', 
        'x5FAA93', 'x605AF5', 'x610B23', 'x621D0A', 'x681DC8', 'x68ED0C', 
        'x6953FC', 'x69C7EB', 'x6A33B4', 'x6AC98F', 'x6B5D9F', 'x6B6A3B', 
        'x6DE1F4', 'x6E0FE6', 'x732A6C', 'x754F27', 'x755D3F', 'x75C21D', 
        'x773C93', 'x79C205', 'x79D467', 'x7BC5C4', 'x7C5C64', 'x7D986A', 
        'x80F01D', 'x83D42C', 'x8499F8', 'x8F99E5', 'x92D205', 'x94A7F0', 
        'x96E48E', 'x996143', 'x9BFE77', 'x9C41AD', 'x9D3924', 'x9F0BC3', 
        'xA0205F', 'xA10512', 'xA150CA', 'xA1F86D', 'xA601ED', 'xA91313', 
        'xABD70F', 'xAD8BC9', 'xADF050', 'xB2F415', 'xB6B036', 'xB6ECD1', 
        'xB7263E', 'xBA7A2F', 'xBB8A73', 'xC41BD4', 'xC671F6', 'xC72E22', 
        'xCA5BB8', 'xCAC8AC', 'xCB325E', 'xCD8841', 'xCD9F06', 'xD4695B', 
        'xD4B56B', 'xDFB42B', 'xE2625E', 'xE3578B', 'xE4EA56', 'xE625EE', 
        'xE8109E', 'xEAC29B', 'xEC465F', 'xED0F30', 'xF4517F', 'xF453D3', 
        'xF5DC62', 'xF6B4D6', 'xFAE75E', 'xFCDEAB'
      ],

    ],
  ])('Should find land state region %s', (name, id, subsistence_building, city, farm, mine, wood, arable_land, 
    arable_resources, capped_resources, provinces
  ) => {
    // Act
    const result = stateRegions.find(r => r.get('id') === id);
    if (result === undefined) {
      throw Error(`Could not find state region ${name} with id ${id}`);
    }
    const raw = result.raw();
    raw.arable_resources?.sort();
    raw.provinces?.sort();
    raw.capped_resources?.sort((a, b) => a.resource.localeCompare(b.resource));

    // Assert
    expect(result.context).toBeDefined();
    expect(result.raw()).toEqual({
      name,
      id,
      subsistence_building,
      city,
      farm,
      mine,
      wood,
      arable_land,
      arable_resources: arable_resources.sort(),
      provinces: provinces.sort(),
      capped_resources: capped_resources?.sort(
        (a, b) => a.resource.localeCompare(b.resource)
      ),
    });
  });

  it('Should convert StateRegion to string', () => {
    // Arrange
    const stateRegion = stateRegions[0];
      
    // Act
    const stringRepresentation = vti.decode([stateRegion], 'test', interpreter);

    // Assert
    expect(stringRepresentation).toBe(svealand);
  });
});

const svealand = `
STATE_SVEALAND = {
  
  id = 1 
  subsistence_building = "building_subsistence_pastures" 
  provinces = {
    "x0974E5" "x216569" "x24A2F1" "x298E7B" "x317138" "x36F523" "x3844C0" "x3E2C22" "x404643" "x41C729" "x432E07" "x45B6EF" "x4628A5" "x4836F2" "x4B02EB" "x4C9918" "x4FA424" "x604060" "x62D0D4" "x656512" "x6CF949" "x6E95C2" "x6F40EC" "x823263" "x8AC21D" "x90845E" "x93C3BC" "x93C76C" "x9686A5" "x9BBBE3" "xA001A0" "xA08021" "xA6C31B" "xA82FF7" "xA86C50" "xA9078A" "xB3F1FB" "xB90566" "xB9391F" "xBC0288" "xC0C0E0" "xC480A5" "xC8AEAF" "xCBFD28" "xCF16B1" "xD155CE" "xD702F7" "xD96FB9" "xE6C5FC" "xE7A6D6" "xE9B084" "xEA6A0F" "xEB5D5F" "xEFA14D" "xF08323" "xF48646" "xF8E400" "xFCCAFA" 
  }
  traits = {
    "state_trait_bergslagen" "state_trait_scandinavian_forests" "state_trait_natural_harbors" 
  }
  city = "x9686A5" 
  port = "x93C3BC" 
  farm = "xF48646" 
  mine = "x6F40EC" 
  wood = "x4C9918" 
  arable_land = 30 
  arable_resources = {
    "bg_rye_farms" "bg_livestock_ranches" 
  }
  naval_exit_id = 3000 
}`;