import { ObjectBlock } from "@/routes/definitions/$definitionId/-feature/components/ObjectBlock";
import type { NodeProps } from "@/routes/definitions/$definitionId/-feature/components/node-props";
import { TextField } from "@/routes/definitions/$definitionId/-feature/components/TextField";
import { ArrayBlock } from "@/routes/definitions/$definitionId/-feature/components/ArrayBlock";

export function Node({ id, value }: NodeProps<unknown>) {
	switch (typeof value) {
		case "boolean":
			return <span>boolean {value}</span>;
		case "number":
			return <span>number {value}</span>;
		case "bigint":
			return <span>bigint {value}</span>;
		case "string":
			return <TextField id={id} value={value} />;
		case "object":
			if (value === null) {
				return <span>null</span>;
			}
			if (Array.isArray(value)) {
				return <ArrayBlock id={id} value={value}/>
			}
			return <ObjectBlock id={id} value={value} />;
	}
}
