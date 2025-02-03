
import { type Definition, definitionSchema } from "@/routes/definitions/$definitionId/-feature/store/types/definition";
import { environmentSchema } from "@/routes/definitions/$definitionId/-feature/store/types/environment";
import { referenceOrProcessorOverrideSchema } from "@/routes/definitions/$definitionId/-feature/store/types/processor";
import { referenceOrObjectSchema } from "@/routes/definitions/$definitionId/-feature/store/types/utils";
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
