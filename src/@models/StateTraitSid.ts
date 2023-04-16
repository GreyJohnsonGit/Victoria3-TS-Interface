import { z } from 'zod';

export type StateTraitSid = z.TypeOf<typeof StateTraitSid>;
export const StateTraitSid = z.string().includes('state_trait_');