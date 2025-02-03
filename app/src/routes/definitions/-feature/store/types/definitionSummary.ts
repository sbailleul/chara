import { z } from "zod";

export const definitionSummarySchema = z.object({
    id: z.string().uuid(),
    name: z.string(),
});
export type DefinitionSummary = z.infer<typeof definitionSummarySchema>;
