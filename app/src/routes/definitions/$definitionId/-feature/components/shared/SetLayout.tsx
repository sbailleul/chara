import { useDefinitionContext } from "@/routes/definitions/$definitionId/-feature/DefinitionContext";
import { IdLabel } from "@/routes/definitions/$definitionId/-feature/components/IdLabel";
import { useLabel } from "@/routes/definitions/$definitionId/-feature/components/hooks";
import { Button } from "@/shared/catalyst/button";
import { FieldGroup } from "@/shared/catalyst/fieldset";
import { Card } from "@/shared/components/Card";
import { BookmarkIcon } from "@heroicons/react/24/outline";
import type { ReactNode } from "react";

interface Props {
	id: string;
	children: ReactNode;
}

export function SetLayout({ id, children }: Props) {
	const label = useLabel(id);
	const { setCurrentHierarchyItem, currentHierarchyItem } =
		useDefinitionContext();
	const Header = (
		<div className="flex justify-between">
			{label}
			<div>
				<Button plain onClick={() => setCurrentHierarchyItem(id)}>
					{currentHierarchyItem === id ? (
						<>
							Marked <BookmarkIcon className="size-5 stroke-brand-500" />
						</>
					) : (
						<>
							Mark <BookmarkIcon className="size-5" />
						</>
					)}
				</Button>
			</div>
		</div>
	);
	return (
		<Card header={Header} id={id}>
			<FieldGroup>{children}</FieldGroup>
		</Card>
	);
}
