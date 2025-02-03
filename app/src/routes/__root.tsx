import { AppSidebar } from "@/core/Sidebar";
import { SidebarLayout } from "@/shared/catalyst/sidebar-layout";
import { createRootRoute, Link, Outlet } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";
export const Route = createRootRoute({
	component: () => (
		<SidebarLayout sidebar={<AppSidebar/>}navbar={undefined} >
			<Outlet />
			<TanStackRouterDevtools />
		</SidebarLayout>
	),
});
