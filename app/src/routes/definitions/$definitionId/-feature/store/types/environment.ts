import { referenceOrObjectSchema } from "@/routes/definitions/$definitionId/-feature/store/types/utils";
import { z } from "zod";

export const environmentSchema = referenceOrObjectSchema(z.record(z.string(), z.string()))
