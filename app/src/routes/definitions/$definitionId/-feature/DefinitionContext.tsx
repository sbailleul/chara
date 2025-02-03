import { definitionApi } from "@/routes/definitions/$definitionId/-feature/store/api";
import { parsePath } from "@/routes/definitions/$definitionId/-feature/utils";
import { useAppDispatch } from "@/store/store";
import {
	createContext,
	useContext,
	useMemo,
	useState,
	type ReactNode,
} from "react";

interface DefinitionContext {
	update: <T>(id: string, value: T) => void;
	setCurrentHierarchyItem: (id: string) => void;
	currentHierarchyItem: string | undefined;
}
const context = createContext<DefinitionContext>({} as DefinitionContext);

export const useDefinitionContext = () => useContext(context);
interface Props {
	definitionId: string;
	children: ReactNode;
}

export function DefinitionContextProvider({ definitionId, children }: Props) {
	const dispatch = useAppDispatch();
	const update = (id: string, value: unknown) => {
		const segments = parsePath(id, false);
		dispatch(
			definitionApi.util.updateQueryData(
				"getDefinition",
				definitionId,
				(definition) => {
					// biome-ignore lint/suspicious/noExplicitAny: <explanation>
					let tmpDefinition = definition as any;
					for (let i = 0; i < segments.length; i++) {
						if (i === segments.length - 1) {
							tmpDefinition[segments[i]] = value;
						} else {
							tmpDefinition = tmpDefinition[segments[i]];
						}
					}
				},
			),
		);
	};
	const [currentHierarchyItem, setCurrentHierarchyItem] = useState<string>();
	const toggleCurrentHierarchyItem = (id: string) => {
		id === currentHierarchyItem
			? setCurrentHierarchyItem(undefined)
			: setCurrentHierarchyItem(id);
	};
	return (
		<context.Provider
			value={{
				update,
				setCurrentHierarchyItem: toggleCurrentHierarchyItem,
				currentHierarchyItem,
			}}
		>
			{children}
		</context.Provider>
	);
}
