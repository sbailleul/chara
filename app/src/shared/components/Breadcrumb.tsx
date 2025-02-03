import { Link, type LinkProps } from "@/shared/catalyst/link";
import clsx from "clsx";

interface Props {
	links: LinkProps[];
	className?:string;
}

export function Breadcrumb({ links, className }: Props) {
	return (
		<nav aria-label="Breadcrumb" className={clsx("flex", className)}>
			<ul className="flex space-x-4 flex-wrap rounded-md bg-neutral px-6 shadow">
				{links.map((link) => (
					<li key={link.href} className="flex">
						<div className="flex items-center">
							<svg
								fill="currentColor"
								viewBox="0 0 24 44"
								preserveAspectRatio="none"
								aria-hidden="true"
								className="h-full w-6 shrink-0 text-gray-200"
							>
								<path d="M.293 0l22 22-22 22h1.414l22-22-22-22H.293z" />
							</svg>
							<Link {...link} />
						</div>
					</li>
				))}
			</ul>
		</nav>
	);
}
