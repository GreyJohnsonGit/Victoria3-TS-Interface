import { CappedResourceExt } from '@lib/CappedResourceExt';
import { None, Option, Some } from '@lib/Option';
import { StateRegion } from '@models/StateRegion';
import { Writer as JominiWriter } from 'jomini';
import { WriterHelper } from './WriterHelper';

export class StateRegionExt {
  public static fromJSON(sid: string, json: unknown): Option<StateRegion> {
    if (typeof json !== 'object' || json === null)
      return None();

    const stateRegion = json as Record<string, unknown>;

    const cappedResources = CappedResourceExt.fromJson({
      capped_resources: stateRegion.capped_resources,
      resource: stateRegion.resource,
    }).match({
      Some: (cappedResources) => cappedResources,
      None: () => [],
    });

    const parsedStateRegion = StateRegion.safeParse({
      sid,
      id: stateRegion.id,
      provinces: stateRegion.provinces,

      traits: Option(stateRegion.traits),
      subsistenceBuilding: Option(stateRegion.subsistence_building),
      arableLand: Option(stateRegion.arable_land),
      arableResources: Option(stateRegion.arable_resources),
      cappedResources: Option(cappedResources),
      city: Option(stateRegion.city),
      port: Option(stateRegion.port),
      farm: Option(stateRegion.farm),
      mine: Option(stateRegion.mine),
      wood: Option(stateRegion.wood),
      navalExitId: Option(stateRegion.naval_exit_id),
    });

    return parsedStateRegion.success ? Some(parsedStateRegion.data) : None();
  }

  public static write(
    stateRegion: Option<StateRegion>,
    writer: JominiWriter
  ): void {
    if (stateRegion.isNone())
      return;

    const w = new WriterHelper(writer);
    const s = stateRegion.value;

    w.assignment(s.sid, () => 
      w.object(() => {
        w.assignment('id', s.id);
        w.optionalAssignment('subsistence_building', s.subsistenceBuilding);
        w.assignment('provinces', () =>
          w.array(() => 
            s.provinces
              .forEach(a => writer.write_quoted(a))));
        s.traits.map(traits =>
          w.assignment('traits', () =>
            w.array(() =>
              traits
                .forEach(a => writer.write_quoted(a)))));
        w.optionalAssignment('city', s.city);
        w.optionalAssignment('port', s.port);
        w.optionalAssignment('farm', s.farm);
        w.optionalAssignment('mine', s.mine);
        w.optionalAssignment('wood', s.wood);
        w.optionalAssignment('arable_land', s.arableLand);
        s.arableResources.map(arableResources =>
          w.assignment('arable_resources', () =>
            w.array(() =>
              arableResources.forEach(a => writer.write_quoted(a)))));
        CappedResourceExt.write(s.cappedResources, writer);
        w.optionalAssignment('naval_exit_id', s.navalExitId);}));
  }
}