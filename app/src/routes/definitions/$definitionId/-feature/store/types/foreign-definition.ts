
import { baseDefinitionSchema } from "@/routes/definitions/$definitionId/-feature/store/types/definition";
import { processorOverrideSchema } from "@/routes/definitions/$definitionId/-feature/store/types/processor";
import { z } from "zod";

export const foreignDefinitionSchema = z
	.string()
	.or(processorOverrideSchema)
	.or(z.lazy(() => baseDefinitionSchema));

export type ForeignDefinition = z.infer<typeof foreignDefinitionSchema>;
