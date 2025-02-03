
import { type ForeignDefinition, foreignDefinitionSchema } from "@/routes/definitions/$definitionId/-feature/store/types/foreign-definition";
import { referenceOrProcessorOverrideSchema } from "@/routes/definitions/$definitionId/-feature/store/types/processor";
import { z } from "zod";

const baseEdgeSchema = z.object({
	processor: referenceOrProcessorOverrideSchema.nullable(),
});

export type Edge = z.infer<typeof baseEdgeSchema> & { definition: ForeignDefinition | null };

export const edgeSchema = baseEdgeSchema.extend({
	definition: z.lazy(() => foreignDefinitionSchema.nullable()),
});
