import { z } from 'zod';

export type ProvinceSid = z.TypeOf<typeof ProvinceSid>;
export const ProvinceSid = z.string().regex(/x[0-9A-F]{6}/);