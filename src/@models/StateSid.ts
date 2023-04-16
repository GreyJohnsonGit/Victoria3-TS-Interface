import { z } from 'zod';

export type StateSid = z.TypeOf<typeof StateSid>;
export const StateSid = z.string().includes('STATE_');