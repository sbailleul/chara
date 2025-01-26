import { referenceOrObjectSchema } from "@/routes/definition/-feature/store/types/utils";
import { z } from "zod";

export const environmentSchema = referenceOrObjectSchema(z.record(z.string(), z.string()))
