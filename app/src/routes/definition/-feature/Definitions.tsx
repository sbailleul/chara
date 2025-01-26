import {
	useGetDefinitionQuery,
	useGetDefinitionsQuery,
} from "@/routes/definition/-feature/store/api";
export function Definitions() {
	const { data } = useGetDefinitionsQuery();

	
	return <>{JSON.stringify(data)}</>;
}
