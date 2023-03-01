import { AppConfig } from '@lib/AppConfig';
import { Context } from '@lib/Context';
import { ErrorFactory } from '@lib/Error';
import { isNone, isSome, Option } from '@lib/Option';
import { OptionalKeys, RequiredKeys } from '@lib/Utility';
import { GuardReturnType, isArray, isBoolean, isNumber, isObject, isOptional, isUndefined } from '@lib/Validator';
import { BuildingGroupId, isBuildingGroupId } from '@models/BuildingGroupId';
import { isBuildingId } from '@models/BuildingId';
import { isProvinceId } from '@models/ProvinceId';
import { isStateRegionName, StateRegionName } from '@models/StateRegionName';
import { isTraitId } from '@models/TraitId';
import { ExpressionInterpreter, IInterpreter } from 'interpreter/Interpreter';
import { Ast, Expression } from 'interpreter/Parser';

export interface IStateRegion {
  context: Context;

  get<K extends RequiredKeys<RawStateRegion>>(key: K): RawStateRegion[K];
  getOptional<K extends OptionalKeys<RawStateRegion>>(key: K): Option<RawStateRegion[K]>;

  set<K extends RequiredKeys<RawStateRegion>>(key: K, value: RawStateRegion[K]): void;
  setOptional<K extends OptionalKeys<RawStateRegion>>(key: K, value: Option<RawStateRegion[K]>): void;
  setOptional<K extends OptionalKeys<RawStateRegion>>(key: K, value: RawStateRegion[K] | undefined): void;

  raw(): RawStateRegion;
  isModified(): boolean;
}

type RawStateRegion = GuardReturnType<typeof isRawStateRegion>;

const isCappedResource = isObject({
  resource: isBuildingGroupId,
  cap: isNumber,
  hidden: isBoolean,
  depleted: isOptional(isBuildingGroupId)
});

const isRawStateRegion = isObject({
  name: isStateRegionName,
  id: isNumber,
  provinces: isArray(isProvinceId),
  subsistence_building: isOptional(isBuildingId),
  traits: isOptional(isArray(isTraitId)),
  city: isOptional(isProvinceId),
  port: isOptional(isProvinceId),
  farm: isOptional(isProvinceId),
  mine: isOptional(isProvinceId),
  wood: isOptional(isProvinceId),
  arable_land: isOptional(isNumber),
  arable_resources: isOptional(isArray(isBuildingGroupId)),
  prime_land: isOptional(isArray(isProvinceId)),
  naval_exit_id: isOptional(isNumber),
  impassable: isOptional(isArray(isProvinceId)),
  capped_resources: isOptional(isArray(isCappedResource))
});


export class StateRegion implements IStateRegion {
  public context: Context;
  private _stateRegion: RawStateRegion;
  private _modified = false;

  constructor(
    unverifiedStateRegion: Partial<RawStateRegion>,
    context: Context,
  ) {
    const errorMessages: string[] = [];
    const onInvalid = (key: string, value: unknown) => {
      return `Invalid state_region.${key}: ${JSON.stringify(value)}`;
    };

    if (!isRawStateRegion(unverifiedStateRegion, { onInvalid })) {
      throw new Error(errorMessages.join('\n'));
    }
    
    this._stateRegion = unverifiedStateRegion;
    this.context = context;
  }

  public isModified() {
    return this._modified;
  }

  public get<K extends RequiredKeys<RawStateRegion>>(key: K) {
    return this._stateRegion[key];
  }
  
  public set<K extends RequiredKeys<RawStateRegion>>(key: K, value: RawStateRegion[K]) {
    this._stateRegion[key] = value;
    this._modified = true;
  }

  public getOptional<K extends OptionalKeys<RawStateRegion>>(key: K) {
    return Option(this._stateRegion[key]);
  }

  public setOptional<K extends OptionalKeys<RawStateRegion>>(key: K, value: RawStateRegion[K] | undefined): void;
  public setOptional<K extends OptionalKeys<RawStateRegion>>(key: K, value: Option<RawStateRegion[K]>): void;
  public setOptional<K extends OptionalKeys<RawStateRegion>>(key: K, value: Option<RawStateRegion[K]> | RawStateRegion[K] | undefined) {
    if (isNone(value) || isUndefined(value)) {
      delete this._stateRegion[key];
      return;
    }
    
    this._stateRegion[key] = isSome(value) ? value.value : value;
    this._modified = true;
  }

  public raw(): RawStateRegion {
    return this._stateRegion;
  }
}

export class StateRegionInterpreter implements IInterpreter<IStateRegion> {
  private interpreter = new ExpressionInterpreter();

  public constructor() {}

  public path(config: AppConfig) {
    return `${config.victoria3Path}/game/map_data/state_regions`;
  } 

  public encode(ast: Ast): StateRegion[] {
    const builders: Partial<RawStateRegion>[] = [];

    const definitions = ast.expression.value;
    for (const definition of definitions) {
      let builder: Partial<RawStateRegion> = {};

      if (definition.kind !== 'assignment')
        throw ErrorFactory.ExpectedAssignment(definition);

      const { variable: { name }, expression: fields } = definition;
      builder.name = name as StateRegionName;
      builder.capped_resources = [];

      if (fields.kind !== 'array')
        throw ErrorFactory.ExpectedArray(fields);

      for (const field of fields.value) {
        if (field.kind !== 'assignment')
          throw ErrorFactory.ExpectedAssignment(definition);

        const { variable: { name: key }, expression: value } = field;
        if (key === 'resource') {
          this.encodeResource(builder, value);
        } else if (key === 'capped_resources') {
          this.encodeCappedResource(builder, value);
        } else {
          const record = builder as Record<string, unknown>;
          record[key] = this.interpreter.encode(value);
          builder = record as Partial<RawStateRegion>;
        }
      }

      if (builder.capped_resources?.length === 0)
        builder.capped_resources = undefined;

      builders.push(builder);
    }
  
    return builders.map(builder => 
      new StateRegion(builder, ast.context)
    ); 
  }
  
  public decode(stateRegions: StateRegion[], cx: Context): Ast {
    const rawStateRegions = stateRegions.map(sr => sr.raw());
    return {
      kind: 'assignment',
      variable: {
        kind: 'variable',
        name: '#GLOBAL',
        context: cx,
      },
      context: cx,
      expression: {
        kind: 'array',
        context: cx,
        value: rawStateRegions.map(rawStateRegion => ({
          kind: 'assignment',
          variable: {
            kind: 'variable',
            name: rawStateRegion.name,
            context: cx,
          },
          context: cx,
          expression: {
            kind: 'array',
            context: cx,
            value: Object.entries(rawStateRegion)
              .filter(([,value]) => value !== undefined)
              .filter(([key]) => key !== 'context')  
              .filter(([key]) => key !== 'name')  
              .map(([key, value]) => {
                if (key === 'capped_resources') {
                  return this.decodeCappedResources(rawStateRegion, cx);
                }

                return {
                  kind: 'assignment' as const,
                  variable: {
                    kind: 'variable' as const,
                    name: key,
                    context: cx,
                  },
                  expression: this.interpreter.decode(value, cx),
                  context: cx,
                };
              }).flat(1) // Because encodeCappedResources returns an array
          }
        }))
      }
    };
  }

  private encodeResource(
    builder: Partial<RawStateRegion>, 
    assignmentArray: Expression
  ) {
    if (assignmentArray.kind !== 'array')
      throw ErrorFactory.ExpectedAssignment(assignmentArray);
    
    const { value: assignments } = assignmentArray;
    const possibleFields: {
      type?: string; 
      undiscovered_amount?: number; 
      discovered_amount?: number;
      depleted_type?: string;
      [invalidKey: string]: unknown;
    } = {};
    for (const assignment of assignments) {
      if (assignment.kind !== 'assignment')
        throw ErrorFactory.ExpectedAssignment(assignment);
      
      const [key, value] = this.interpreter.encode(assignment) as [string, unknown];
      possibleFields[key] = value;
    }

    if (possibleFields.type === undefined)
      throw ErrorFactory.ExpectedField('\'type\'', assignmentArray);

    const cap = possibleFields.undiscovered_amount ?? possibleFields.discovered_amount;
    if (cap === undefined) 
      throw ErrorFactory.ExpectedField(
        '\'undiscovered_amount\' or \'discovered_amount\'', 
        assignmentArray
      );

    builder.capped_resources?.push({
      resource: possibleFields.type as BuildingGroupId,
      cap,
      hidden: possibleFields.discovered_amount === undefined,
      depleted: possibleFields.depleted_type as BuildingGroupId
    });
  }
  
  private encodeCappedResource(
    builder: Partial<RawStateRegion>, 
    assignmentArray: Expression
  ) {
    if (assignmentArray.kind !== 'array')
      throw ErrorFactory.ExpectedArray(assignmentArray);
    

    for (const assignment of assignmentArray.value) {
      if (assignment.kind !== 'assignment')
        throw ErrorFactory.ExpectedAssignment(assignment);
      
      const { variable: { name }, expression: value } = assignment;

      builder.capped_resources?.push({
        resource: name as BuildingGroupId,
        cap: this.interpreter.encode(value) as number,
        hidden: false,
        depleted: undefined
      });
    }
  }

  private decodeCappedResources(stateRegion: RawStateRegion, cx: Context): Expression[] {
    const special = stateRegion.capped_resources?.filter(c => c.hidden || c.depleted) ?? [];
    const normal = stateRegion.capped_resources?.filter(c => !c.hidden && !c.depleted) ?? [];
    
    const capped_resources = {} as Record<string, number>;
    for (const resource of normal) {
      capped_resources[resource.resource] = resource.cap;
    }
    
    const special_resources = [];
    for (const resource of special) {
      special_resources.push({
        [resource.hidden ? 'discovered_amount' : 'undiscovered_amount']: resource.cap,
        'depleted_type': resource.depleted,
        'type': resource.resource
      });

      return [
        {
          kind: 'assignment',
          variable: {
            kind: 'variable',
            name: 'capped_resources',
            context: cx,
          },
          context: cx,
          expression: this.interpreter.decode(capped_resources, cx)
        },
        ...special_resources.map(r => this.interpreter.decode(r, cx))
      ];
    }

    return [];
  }
}