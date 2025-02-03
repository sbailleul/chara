import {
	type Edge,
	edgeSchema,
} from "@/routes/definitions/$definitionId/-feature/store/types/edge";
import { metadataSchema } from "@/routes/definitions/$definitionId/-feature/store/types/metadata";
import { processorSchema } from "@/routes/definitions/$definitionId/-feature/store/types/processor";
import { tagSchema } from "@/routes/definitions/$definitionId/-feature/store/types/tag";
import { type ZodType, z } from "zod";

export const baseDefinitionSchema = z.object({
	id: z.string().uuid(),
	name: z.string(),
	location: z.string().nullable(),
	metadata: z.record(z.string(), metadataSchema),
	tags: z.record(z.string(), tagSchema),
	processors: z.record(z.string(), processorSchema),
	arguments: z.record(z.string(), z.array(z.string())),
	environments: z.record(z.string(), z.record(z.string(), z.string())),
});

export type Definition = z.infer<typeof baseDefinitionSchema> & {
	edges: Record<string, Edge>;
};

export const definitionSchema: ZodType<Definition> =
	baseDefinitionSchema.extend({
		edges: z.lazy(() => z.record(z.string(), edgeSchema)),
	});

