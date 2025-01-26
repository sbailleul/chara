import { environmentSchema } from "@/routes/definition/-feature/store/types/environment";
import { installSchema } from "@/routes/definition/-feature/store/types/install";
import { referenceOrObjectSchema } from "@/routes/definition/-feature/store/types/utils";
import { z } from "zod";

export const processorOverrideSchema = z.object({
	ref: z.string().nullable(),
	arguments: z.array(z.string()),
	environments: z.array(environmentSchema),
});

export type ProcessorOverride = z.infer<typeof processorOverrideSchema>;
export const referenceOrProcessorOverrideSchema = referenceOrObjectSchema(
	processorOverrideSchema,
);

export const processorSchema = z.object({
	arguments: z.array(z.string()),
	environments: z.array(environmentSchema),
	program: z.string(),
	install: installSchema.nullable(),
	currentDirectory: z.string().nullable(),
});
