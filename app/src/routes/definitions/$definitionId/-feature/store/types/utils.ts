import { z, type ZodTypeAny } from "zod";

export const referenceOrObjectSchema = <T extends ZodTypeAny>(
	objectSchema: T,
) => z.string().or(objectSchema);
