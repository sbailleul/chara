import {
	type DefinitionSummary,
	definitionSummarySchema,
} from "@/routes/definitions/-feature/store/types/definitionSummary";
import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";

// Define a service using a base URL and expected endpoints
export const definitionsApi = createApi({
	reducerPath: "definitionsApi",
	baseQuery: fetchBaseQuery({ baseUrl: "http://localhost:8000/api/" }),
	endpoints: (builder) => ({
		getDefinitions: builder.query<DefinitionSummary[], void>({
			query: () => "definitions-summaries",
			transformResponse: (responses: DefinitionSummary[]) => {
				return responses.map((def) => definitionSummarySchema.parse(def));
			},
		}),
	}),
});

export const { useGetDefinitionsQuery } = definitionsApi;
