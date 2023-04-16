import { None, Option, Some } from '@lib/Option';
import { CappedResource } from '@models/CappedResource';
import { Writer as JominiWriter } from 'jomini';
import { WriterHelper } from './WriterHelper';

export class CappedResourceExt {
  public static fromJson(json: {
    capped_resources: unknown;
    resource: unknown;
  }): Option<CappedResource[]> {
    const { capped_resources, resource } = json as {
      capped_resources: Record<string, unknown>;
      resource: Record<string, unknown> | Record<string, unknown>[] | undefined;
    };

    const cappedResources = [] as 
      Partial<Record<keyof CappedResource, unknown>>[];

    for (const [sid, amount] of Object.entries(capped_resources)) {
      cappedResources.push({
        sid,
        amount,
        hidden: false,
        depleteType: None(),
      });
    }

    if (resource !== undefined) {
      const hiddenResources = Array.isArray(resource) ? resource : [resource];
      for (const data of hiddenResources) {
        cappedResources.push({
          sid: data.type,
          amount: data.undiscovered_amount,
          hidden: true,
          depleteType: Option(data.deplete_type),
        });
      }
    }

    const parseResult = CappedResource.array().safeParse(cappedResources);
    return parseResult.success ? Some(parseResult.data) : None();
  }

  public static write(
    cappedResources: Option<CappedResource[]>,
    writer: JominiWriter
  ): void {
    console.log(cappedResources.unwrapOr([]));
    if (cappedResources.isNone() || cappedResources.value.length === 0)
      return;

    const w = new WriterHelper(writer);

    const knownResources = cappedResources.value.filter(r => !r.hidden);
    const hiddenResources = cappedResources.value.filter(r => r.hidden);

    w.assignment('capped_resources', () => 
      w.object(() => 
        knownResources.forEach(r => 
          w.assignment(r.sid, r.amount))));

    if (hiddenResources.length === 0)
      return;

    hiddenResources.forEach(r => 
      w.assignment('resource', () => 
        w.object(() => {
          w.assignment('type', r.sid);
          w.assignment('undiscovered_amount', r.amount);
          w.optionalAssignment('deplete_type', r.depleteType);})));
  }
}