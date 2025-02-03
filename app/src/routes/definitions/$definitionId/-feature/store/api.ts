
import { type Definition, definitionSchema } from "@/routes/definitions/$definitionId/-feature/store/types/definition";
import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";

export const definitionApi = createApi({
	reducerPath: "definitionApi",
	baseQuery: fetchBaseQuery({ baseUrl: "http://localhost:8000/api/" }),
	endpoints: (builder) => ({
		getDefinition: builder.query<Definition, string>({
			query: (id) => `definitions/${id}`,
			transformResponse: (response: Definition) => {
				definitionSchema.parse(response);
				return response;
			},
		}),
	}),
});

export const { useGetDefinitionQuery } = definitionApi;
