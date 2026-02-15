import { z } from "zod";

export const SettingsSchema = z.object({
  user_llm_provider: z.string().min(1),
  user_llm_key: z.string().optional().nullable(),
  sandbox_enabled: z.boolean()
});

export const AgentCreateSchema = z.object({
  name: z.string().min(1),
  role: z.string().min(1),
  goal: z.string().min(1),
  tools: z.array(z.string()).default([]),
  config: z.record(z.any()).default({})
});
