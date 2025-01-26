import {
	type Definition,
	definitionSchema,
} from "@/routes/definition/-feature/store/types/types";
import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";

// Define a service using a base URL and expected endpoints
export const definitionApi = createApi({
	reducerPath: "pokemonApi",
	baseQuery: fetchBaseQuery({ baseUrl: "http://localhost:8000/api/" }),
	endpoints: (builder) => ({
		getDefinition: builder.query<Definition, string>({
			query: (id) => `definitions/${id}`,
			transformResponse: (response: Definition) => {
				definitionSchema.parse(response);
				return response;
			},
		}),
		getDefinitions: builder.query<Definition[], void>({
			query: () => "definitions",
			transformResponse: (responses: Definition[]) => {
				return responses.map((def) => definitionSchema.parse(def));
			},
		}),
	}),
});

export const { useGetDefinitionQuery, useGetDefinitionsQuery } = definitionApi;
