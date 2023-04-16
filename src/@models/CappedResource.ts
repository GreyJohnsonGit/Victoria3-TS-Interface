import { CappedResourceSid } from '@models/CappedResourceSid';
import { Optional } from 'src/@lib/Option';
import { z } from 'zod';

export type CappedResource = z.infer<typeof CappedResource>;
export const CappedResource = z.object({
  sid: CappedResourceSid,
  amount: z.number().positive().int(),
  hidden: z.boolean(),
  depleteType: Optional(CappedResourceSid),
});