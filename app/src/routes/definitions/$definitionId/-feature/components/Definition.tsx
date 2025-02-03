import { HierarchyNavbar } from "@/routes/definitions/$definitionId/-feature/components/HierarchyNavbar";
import { Node } from "@/routes/definitions/$definitionId/-feature/components/Node";
import { DefinitionContextProvider } from "@/routes/definitions/$definitionId/-feature/DefinitionContext";
import { useGetDefinitionQuery } from "@/routes/definitions/$definitionId/-feature/store/api";
import { Sidebar } from "@/shared/catalyst/sidebar";

interface Props {
	definitionId: string;
}
export function Definition({ definitionId }: Props) {
	const { data: definition } = useGetDefinitionQuery(definitionId);
	if (!definition) {
		return;
	}

	return (
		<DefinitionContextProvider definitionId={definitionId}>
			<HierarchyNavbar />
			<Node id={""} value={definition} key={""} />
		</DefinitionContextProvider>
	);
}
