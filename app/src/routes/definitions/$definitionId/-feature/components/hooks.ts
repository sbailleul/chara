import { useMemo } from "react";

export function useLabel(id: string): string {
	return useMemo(() => {
		if (id.endsWith("]")) {
			const arrayIndex = id.match(/\[(\d*)\]/)?.pop();
			return `Item ${arrayIndex}`;
		}
		return id.split(".").pop() as string;

	}, [id]);
}
