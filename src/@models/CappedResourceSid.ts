import { z } from 'zod';

export type CappedResourceSid = z.TypeOf<typeof CappedResourceSid>;
export const CappedResourceSid = z.string().includes('bg_');