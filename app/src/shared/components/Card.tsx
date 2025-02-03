import type { ReactNode } from "react";

interface Props {
	header?: ReactNode;
	footer?: ReactNode;
	children: ReactNode;
    id: string;
}

export function Card({ header, footer, children , id}: Props) {
	return (
		<div id={id} className="divide-y divide-neutral-200 overflow-hidden rounded-lg bg-white shadow">
			{header && <div className="px-4 py-5 sm:px-6">{header}</div>}
			<div className="px-4 py-5 sm:px-6">{children}</div>
			{footer && <div className="px-4 py-4 sm:px-6">{footer}</div>}
		</div>
	);
}
