/**
 * TODO: Update this component to use your client-side framework's link
 * component. We've provided examples of how to do this for Next.js, Remix, and
 * Inertia.js in the Catalyst documentation:
 *
 * https://catalyst.tailwindui.com/docs#client-side-router-integration
 */

import * as Headless from "@headlessui/react";
import type React from "react";
import { forwardRef } from "react";
import {
	Link as TanstackLink,
	type LinkComponentProps,
} from "@tanstack/react-router";
export const Link = forwardRef(function Link(
	props: { href: string } & React.ComponentPropsWithoutRef<"a"> &
		LinkComponentProps<"a">,
	ref: React.ForwardedRef<HTMLAnchorElement>,
) {
	console.log("T");
	return (
		<Headless.DataInteractive>
			<TanstackLink {...props} ref={ref} />
		</Headless.DataInteractive>
	);
});
