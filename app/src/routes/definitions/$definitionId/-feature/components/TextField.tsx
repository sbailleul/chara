import { IdLabel } from "@/routes/definitions/$definitionId/-feature/components/IdLabel";
import type { NodeProps } from "@/routes/definitions/$definitionId/-feature/components/node-props";
import { useDefinitionContext } from "@/routes/definitions/$definitionId/-feature/DefinitionContext";
import { definitionApi } from "@/routes/definitions/$definitionId/-feature/store/api";
import { Field, Label } from "@/shared/catalyst/fieldset";
import { Input } from "@/shared/catalyst/input";
import { useContext } from "react";
import { useDispatch } from "react-redux";

export function TextField({ value, id }: NodeProps<string>) {
	const { update } = useDefinitionContext();
	return (
		<Field>
			<IdLabel id={id} />
			<Input
				id={id}
				value={value}
				type="text"
				onChange={(e) => update(id, e.target.value)}
			/>
		</Field>
	);
}
