import { isBuildingGroupId } from '@models/BuildingGroupId';
import { BuildingGroupIdGenerator } from '@models/BuildingGroupId.gen';
import { isStateRegionName } from '@models/StateRegionName';
import { StateRegionNameGenerator } from '@models/StateRegionName.gen';
import { isTraitId } from '@models/TraitId';
import { TraitIdFactory } from '@models/TraitId.gen';
import { IModelGenerator } from 'generator/IModelGenerator';
import { SimpleId } from 'generator/SimpleIdInterpreter';
import { Ast, Parser } from 'interpreter/Parser';
import { Tokenizer } from 'interpreter/Token';

function toAst(text: string) {
  return new Parser().encode(
    new Tokenizer().encode(
      text, { filepath: 'test', line: 0, column: 0 }
    )
  );
}

const models: {
  [name: string]: {
    guard: (value: unknown) => boolean;
    ids: SimpleId[];
    factory: IModelGenerator<SimpleId>;
    ast: Ast;
    formatted: string;
  };
} = {
  BuildingGroupId: {
    guard: isBuildingGroupId,
    ids: ['bg_mining', 'bg_monuments', 'bg_coal_mining'],
    factory: new BuildingGroupIdGenerator({
      encode: () => models.BuildingGroupId.ids as string[],
      decode: () => {throw new Error('Method not implemented.');},
      path: () => {throw new Error('Method not implemented.');}
    }),
    ast: toAst(`\
bg_mining = {}
bg_monuments = {}
bg_coal_mining = {}`
    ),
    formatted: (`\
{
  "bg_mining": {},
  "bg_monuments": {},
  "bg_coal_mining": {}
}`
    ),
  },

  TraitId: {
    guard: isTraitId,
    ids: [
      'state_trait_malaria', 
      'state_trait_australian_desert', 
      'state_trait_uruguay_river'
    ],
    factory: new TraitIdFactory({
      encode: () => models.TraitId.ids as string[],
      decode: () => {throw new Error('Method not implemented.');},
      path: () => {throw new Error('Method not implemented.');}
    }),
    ast: toAst(`\
state_trait_malaria = {}
state_trait_australian_desert = {}
state_trait_uruguay_river = {}`
    ),
    formatted: (`\
{
  "state_trait_malaria": {},
  "state_trait_australian_desert": {},
  "state_trait_uruguay_river": {}
}`
    )
  },

  StateRegionName: {
    guard: isStateRegionName,
    ids: [
      'STATE_SVEALAND', 
      'STATE_BUENOS_AIRES', 
      'STATE_LAKE_NICARAGUA'
    ],
    factory: new StateRegionNameGenerator({
      encode: () => models.StateRegionName.ids as string[],
      decode: () => {throw new Error('Method not implemented.');},
      path: () => {throw new Error('Method not implemented.');}
    }),
    ast: toAst(`\
STATE_SVEALAND = {}
STATE_BUENOS_AIRES = {}
STATE_LAKE_NICARAGUA = {}`
    ),
    formatted: (`\
{
  "STATE_SVEALAND": {},
  "STATE_BUENOS_AIRES": {},
  "STATE_LAKE_NICARAGUA": {}
}`
    )
  }
};
  
Object.entries(models).forEach(([name, data]) => {
  describe(name, () => {
    it('Should have guard created', () => {
      expect(data.guard).toBeDefined();
    });
  
    it('Should encode ast to ids', () => {
      // Arrange
      const { factory, ast, ids: expectedIds } = data;
      
      // Act
      const ids = factory.interpreter.encode(ast);
  
      // Assert
      expect(ids).toEqual(expectedIds);
    });
  
    it('Should format ids to string', () => {
      // Arrange
      const { factory, ids, formatted } = data;
  
      // Act
      const stringRepresentation = factory.format(ids);
  
      // Assert
      expect(stringRepresentation).toEqual(formatted);
    });
  
    it('Should encode and format ast to string', () => {
      // Arrange
      const { factory, ast, formatted } = data;
  
      // Act
      const ids = factory.interpreter.encode(ast);
      const stringRepresentation = factory.format(ids);
  
      // Assert
      expect(stringRepresentation).toEqual(formatted);
    });
  });
});