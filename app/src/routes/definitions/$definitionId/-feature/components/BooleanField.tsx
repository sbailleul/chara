import { IdLabel } from "@/routes/definitions/$definitionId/-feature/components/IdLabel";
import type { NodeProps } from "@/routes/definitions/$definitionId/-feature/components/node-props";
import { useDefinitionContext } from "@/routes/definitions/$definitionId/-feature/DefinitionContext";
import { Checkbox } from "@/shared/catalyst/checkbox";
import { Field } from "@headlessui/react";

export function BooleanField({ id, value }: NodeProps<boolean>) {
	const { update } = useDefinitionContext();

	return (
		<Field>
			<IdLabel id={id} />
			<Checkbox checked={value} onChange={(val) => update(id, val)} />
		</Field>
	);
}
