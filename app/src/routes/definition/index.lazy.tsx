import { Definitions } from "@/routes/definition/-feature/Definitions";
import { createLazyFileRoute } from "@tanstack/react-router";

export const Route = createLazyFileRoute("/definition/")({
	component: RouteComponent,
});

function RouteComponent() {
	return (
		<div>
			<Definitions />
		</div>
	);
}
