import { useLabel } from "@/routes/definitions/$definitionId/-feature/components/hooks";
import { Label } from "@/shared/catalyst/fieldset";

interface Props {
	id: string;
}
export function IdLabel({ id }: Props) {
	const label = useLabel(id);
	return <Label htmlFor={id}>{label}</Label>;
}
