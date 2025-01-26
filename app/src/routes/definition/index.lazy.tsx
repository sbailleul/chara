import { Test } from "@/routes/definition/-feature/Definition*";
import { createLazyFileRoute } from "@tanstack/react-router";

export const Route = createLazyFileRoute("/definition/")({
	component: RouteComponent,
});

function RouteComponent() {
	return (
		<div>
			<Test />
		</div>
	);
}
