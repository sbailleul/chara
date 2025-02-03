import { useLabel } from "@/routes/definitions/$definitionId/-feature/components/hooks";
import { Node } from "@/routes/definitions/$definitionId/-feature/components/Node";
import type { NodeProps } from "@/routes/definitions/$definitionId/-feature/components/node-props";
import { SetLayout } from "@/routes/definitions/$definitionId/-feature/components/shared/SetLayout";
import { FieldGroup, Fieldset, Legend } from "@/shared/catalyst/fieldset";
import { Card } from "@/shared/components/Card";

export function ObjectBlock({ id, value }: NodeProps<object>) {
	const nodes = Object.entries(value as object).map(([k, v]) => {
		const fieldId = `${id}.${k}`;
		return <Node key={fieldId} id={fieldId} value={v} />;
	});

	return <SetLayout id={id}>{nodes}</SetLayout>;
}
