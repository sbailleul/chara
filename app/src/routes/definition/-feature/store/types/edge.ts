import {
	foreignDefinitionSchema,
	type ForeignDefinition,
} from "@/routes/definition/-feature/store/types/foreignDefinition";
import { referenceOrProcessorOverrideSchema } from "@/routes/definition/-feature/store/types/processor";
import { z } from "zod";

const baseEdgeSchema = z.object({
	processor: referenceOrProcessorOverrideSchema.nullable(),
});

export type Edge = z.infer<typeof baseEdgeSchema> & { definition: ForeignDefinition | null };

export const edgeSchema = baseEdgeSchema.extend({
	definition: z.lazy(() => foreignDefinitionSchema.nullable()),
});
