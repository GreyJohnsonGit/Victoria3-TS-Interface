import { ArableResourceSid } from '@models/ArableResourceSid';
import { CappedResource } from '@models/CappedResource';
import { ProvinceSid } from '@models/ProvinceSid';
import { StateId } from '@models/StateId';
import { StateSid } from '@models/StateSid';
import { StateTraitSid } from '@models/StateTraitSid';
import { SubsistenceBuildingSid } from '@models/SubsistenceBuildingSid';
import { Optional } from 'src/@lib/Option';
import { z } from 'zod';

export type StateRegion = z.TypeOf<typeof StateRegion>
export const StateRegion = z.object({
  id: StateId,
  sid: StateSid,
  provinces: ProvinceSid.array(),
  
  traits: Optional(StateTraitSid.array()),
  subsistenceBuilding: Optional(SubsistenceBuildingSid),
  arableLand: Optional(z.number().nonnegative().int()),
  arableResources: Optional(ArableResourceSid.array()),
  cappedResources: Optional(CappedResource.array()),

  city: Optional(ProvinceSid),
  port: Optional(ProvinceSid),
  farm: Optional(ProvinceSid),
  mine: Optional(ProvinceSid),
  wood: Optional(ProvinceSid),
  navalExitId: Optional(StateId),
});