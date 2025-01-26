import { environmentSchema } from "@/routes/definition/-feature/store/types/environment";
import { z } from "zod";

export const installSchema = z.object({
    program: z.string(),
    arguments: z.array(z.string()),
    environments: z.array(environmentSchema),
    currentDirectory: z.string().nullable()
})