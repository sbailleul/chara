import {
	Sidebar,
	SidebarBody,
	SidebarSection,
	SidebarItem,
} from "@/shared/catalyst/sidebar";
import { Logo } from "@/shared/components/Logo";
import { Link } from "@tanstack/react-router";

export function AppSidebar() {
	return (
		<Sidebar>
			<SidebarBody>
				<div className="mb-2 flex">
					<Link aria-label="Home" to={"."}>
						<Logo />
					</Link>
				</div>
				<SidebarSection>
					<SidebarItem to=".">Home</SidebarItem>
					<SidebarItem to="/graph">Graph</SidebarItem>
					<SidebarItem to="/definitions">Definitions</SidebarItem>
				</SidebarSection>
			</SidebarBody>
		</Sidebar>
	);
}
