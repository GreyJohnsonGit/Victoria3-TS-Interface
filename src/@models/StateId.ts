import { z } from 'zod';

export type StateId = z.TypeOf<typeof StateId>;
export const StateId = z.number().positive().int();