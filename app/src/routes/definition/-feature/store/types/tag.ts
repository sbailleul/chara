import { z, type ZodType } from "zod";

const baseTagSchema = z.object({
	label: z.string().nullable(),
});

type Tag = z.infer<typeof baseTagSchema> & {
	tags: Record<string, Tag>;
} & Record<string, unknown>;

export const tagSchema: ZodType<Tag> = baseTagSchema
	.extend({
		tags: z.record(
			z.string(),
			z.lazy(() => tagSchema),
		),
	})
	.and(z.record(z.string(), z.any()));
