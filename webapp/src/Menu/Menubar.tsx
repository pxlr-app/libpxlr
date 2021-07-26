import { Component, PropsWithChildren, Show, useContext } from "solid-js";
import { faCheck, faChevronRight } from "@fortawesome/pro-regular-svg-icons";
import FontAwesomeIcon from "../shims/FontAwesomeIcon";
import { UnstyledMenu, UnstyledMenuItem, UnstyledMenuItemProps } from "./UnstyledMenu";
import { Anchor, AnchorContext, Constraints, HorizontalAlign, VerticalAlign } from "../Anchor";
import "./Menubar.css";

export type MenubarProps = {
	ref?: HTMLElement | ((e: HTMLElement) => void);
};

export const Menubar: Component<PropsWithChildren<MenubarProps>> = (props) => {
	return (
		<UnstyledMenu orientation="horizontal">
			{({ props: innerProps }) => (
				<nav
					{...innerProps}
					ref={props.ref}
					class="pointer-events-auto cursor-default inline-flex p-0 m-0 border border-transparent outline-none overflow-visible bg-gray-900 text-gray-200 text-xs select-none focus:border-blue-500"
				>
					<ul className="flex flex-row p-0 m-0 justify-end flex-nowrap">{props.children}</ul>
				</nav>
			)}
		</UnstyledMenu>
	);
};

export type MenubarItemProps = Omit<UnstyledMenuItemProps, "children"> & {
	/**
	 * The label of this menu item
	 */
	label: string;
};

export const MenubarItem: Component<MenubarItemProps> = (props) => {
	return (
		<UnstyledMenuItem id={props.id} accessKey={props.accessKey} action={props.action}>
			{({ selected, opened, showAccessKey, props: innerProps }) => {
				const hasChildren = "children" in props;
				return (
					<li
						{...innerProps}
						class="pointer-events-auto relative outline-none focus:bg-gray-700 focus-within:bg-gray-700"
						classList={{
							"bg-gray-700": selected(),
						}}
					>
						<div className="pointer-events-none flex flex-nowrap px-3 py-1 whitespace-nowrap">
							<Show when={props.accessKey} fallback={props.label}>
								<>
									{props.label.split(props.accessKey).shift()}
									<span
										classList={{
											"underline uppercase": showAccessKey(),
										}}
									>
										{props.accessKey}
									</span>
									{props.label.split(props.accessKey).slice(1).join(props.accessKey)}
								</>
							</Show>
						</div>
						<Show when={hasChildren && opened()}>
							<Anchor constraints={anchorConstraints} class="z-50">
								{props.children}
							</Anchor>
						</Show>
					</li>
				);
			}}
		</UnstyledMenuItem>
	);
};

const anchorConstraints: Constraints = {
	origins: [
		{
			anchor: [HorizontalAlign.LEFT, VerticalAlign.BOTTOM],
			transform: [HorizontalAlign.LEFT, VerticalAlign.TOP],
		},
		{
			anchor: [HorizontalAlign.LEFT, VerticalAlign.TOP],
			transform: [HorizontalAlign.LEFT, VerticalAlign.BOTTOM],
		},
		{
			anchor: [HorizontalAlign.RIGHT, VerticalAlign.BOTTOM],
			transform: [HorizontalAlign.RIGHT, VerticalAlign.TOP],
		},
		{
			anchor: [HorizontalAlign.RIGHT, VerticalAlign.TOP],
			transform: [HorizontalAlign.RIGHT, VerticalAlign.BOTTOM],
		},
	],
};
