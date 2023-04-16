import { z } from 'zod';

export type ArableResourceSid = z.TypeOf<typeof ArableResourceSid>;
export const ArableResourceSid = z.string().includes('bg_');