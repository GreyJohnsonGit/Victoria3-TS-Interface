import { z } from 'zod';

export type SubsistenceBuildingSid = z.TypeOf<typeof SubsistenceBuildingSid>;
export const SubsistenceBuildingSid = z
  .string()
  .includes('building_subsistence_');