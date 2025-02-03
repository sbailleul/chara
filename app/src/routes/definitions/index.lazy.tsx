import { AppSidebar } from "@/core/Sidebar";
import { Definitions } from "@/routes/definitions/-feature/components/Definitions";
import { SidebarLayout } from "@/shared/catalyst/sidebar-layout";
import { createLazyFileRoute } from "@tanstack/react-router";

export const Route = createLazyFileRoute("/definitions/")({
	component: RouteComponent,
});
function RouteComponent() {
	return <Definitions />;
}
