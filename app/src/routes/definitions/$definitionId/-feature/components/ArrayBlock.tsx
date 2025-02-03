import { useLabel } from "@/routes/definitions/$definitionId/-feature/components/hooks";
import type { NodeProps } from "@/routes/definitions/$definitionId/-feature/components/node-props";
import { FieldGroup } from "@/shared/catalyst/fieldset";
import { Fieldset, Legend } from "@headlessui/react";
import { Node } from "@/routes/definitions/$definitionId/-feature/components/Node";
import { SetLayout } from "@/routes/definitions/$definitionId/-feature/components/shared/SetLayout";

export function ArrayBlock({ id, value }: NodeProps<unknown[]>) {
	const nodes = value?.map((v, i) => {
		const fieldId = `${id}[${i}]`;
		return <Node key={fieldId} id={fieldId} value={v} />;
	});

	return <SetLayout id={id}>{nodes}</SetLayout>;
}
