export function parsePath(
	path: string | undefined,
	keepSeparators: true,
): string[];
export function parsePath(
	path: string | undefined,
	keepSeparators: false,
): (string | number)[];
export function parsePath(
	path: string | undefined,
	keepSeparators: boolean,
): typeof keepSeparators extends true ? string[] : (string | number)[] {
	const segments = path?.match(/(\[(\d*)\])|(\.\w*)/g) ?? [];
	if (keepSeparators === true) {
		return segments;
	}
	return segments.map(cleanSegment);
}

export function cleanSegment(itemId: string): string | number {
	if (itemId.startsWith(".")) {
		return itemId.substring(1);
	}
	return Number.parseInt(itemId.substring(1, itemId.length - 1));
}
