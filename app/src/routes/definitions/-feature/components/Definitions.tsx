import { useGetDefinitionsQuery } from "@/routes/definitions/-feature/store/api";
import { Field, Label } from "@/shared/catalyst/fieldset";
import { Select } from "@/shared/catalyst/select";
import { Outlet, useNavigate, useParams } from "@tanstack/react-router";
export function Definitions() {
	const { data: definitions } = useGetDefinitionsQuery();
	const navigate = useNavigate();
	if (definitions === undefined) {
		return;
	}
	return (
		<>
			<Field>
				<Label htmlFor="definition">Select definition</Label>
				<Select
					id="definition"
					onChange={({ target: { value: id } }) => {
						navigate({
							to: "/definitions/$definitionId",
							params: { definitionId: id },
						});
					}}
				>
					{definitions.map(({ id, name }) => (
						<option key={id} value={id}>
							{name} ({id})
						</option>
					))}
				</Select>
			</Field>
		</>
	);
}
