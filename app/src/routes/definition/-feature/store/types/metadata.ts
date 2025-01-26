import {
	type Edge,
	edgeSchema,
} from "@/routes/definition/-feature/store/types/edge";
import { environmentSchema } from "@/routes/definition/-feature/store/types/environment";
import {
	processorSchema,
	referenceOrProcessorOverrideSchema,
} from "@/routes/definition/-feature/store/types/processor";
import { tagSchema } from "@/routes/definition/-feature/store/types/tag";
import {
	type Definition,
	definitionSchema,
} from "@/routes/definition/-feature/store/types/types";
import { referenceOrObjectSchema } from "@/routes/definition/-feature/store/types/utils";
import { type ZodType, z } from "zod";

const baseMetadataEdgeSchema = z.object({
	ref: z.string(),
	arguments: z.array(z.string()),
	environments: z.array(environmentSchema),
});

type MetadataEdge = z.infer<typeof baseMetadataEdgeSchema> & {
	definition: Definition | null;
} & Record<string, unknown>;

const metadataEdgeSchema: ZodType<MetadataEdge> = baseMetadataEdgeSchema
	.extend({
		definition: z.lazy(() => definitionSchema).nullable(),
	})
	.and(z.record(z.string(), z.any()));

const referenceOrMetadataEdgeSchema =
	referenceOrObjectSchema(metadataEdgeSchema);

export const metadataSchema = z
	.object({
		edges: z.array(referenceOrMetadataEdgeSchema),
		tags: z.array(z.string()),
		processor: referenceOrProcessorOverrideSchema.nullable(),
	})
	.and(z.record(z.string(), z.any()));
