import { Graph } from "@/features/graph/components/Graph";
import { createLazyFileRoute } from "@tanstack/react-router";

export const Route = createLazyFileRoute("/graph/")({
	component: RouteComponent,
});

function RouteComponent() {
	return <Graph />;
}
