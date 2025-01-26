import { useGetDefinitionQuery } from "@/routes/definition/-feature/store/api";
export function Test() {
	const { data } = useGetDefinitionQuery(
		"c7447dd8-2da4-4719-b4c3-4250200c9563",
	);
	
	return <>{JSON.stringify(data)}</>;
}
