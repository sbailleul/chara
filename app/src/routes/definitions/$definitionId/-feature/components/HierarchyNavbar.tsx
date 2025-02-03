import { useDefinitionContext } from "@/routes/definitions/$definitionId/-feature/DefinitionContext";
import {
	cleanSegment,
	parsePath,
} from "@/routes/definitions/$definitionId/-feature/utils";
import type { LinkProps } from "@/shared/catalyst/link";
import { Breadcrumb } from "@/shared/components/Breadcrumb";
import { useMemo } from "react";

export function HierarchyNavbar() {
	const { currentHierarchyItem } = useDefinitionContext();

	const links = useMemo(
		() =>
			parsePath(currentHierarchyItem, true).map((itemId, idx, items) => {
				return {
					hash: `${items.slice(0, idx).join("")}${itemId}`,
					children: cleanSegment(itemId),
				} as LinkProps;
			}),
		[currentHierarchyItem],
	);
	return <Breadcrumb links={links} className="fixed z-50 bg-opacity-80 bg-neutral-100 max-w-6xl flex-wrap "/>;
}
