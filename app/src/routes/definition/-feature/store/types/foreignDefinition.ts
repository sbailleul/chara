import { processorOverrideSchema } from "@/routes/definition/-feature/store/types/processor";
import {
	baseDefinitionSchema,
	type Definition,
} from "@/routes/definition/-feature/store/types/types";
import { z } from "zod";

export const foreignDefinitionSchema = z
	.string()
	.or(processorOverrideSchema)
	.or(z.lazy(() => baseDefinitionSchema));

export type ForeignDefinition = z.infer<typeof foreignDefinitionSchema>;
