import { Component, PropsWithChildren, Show, useContext } from "solid-js";
import { faCheck, faChevronRight } from "@fortawesome/pro-regular-svg-icons";
import FontAwesomeIcon from "../shims/FontAwesomeIcon";
import { UnstyledMenu, UnstyledMenuItem, UnstyledMenuItemProps } from "./UnstyledMenu";
import { Anchor, AnchorContext, Constraints, HorizontalAlign, VerticalAlign } from "../Anchor";
import "./Menu.css";

export type MenuProps = {
	ref?: HTMLElement | ((e: HTMLElement) => void);
};

export const Menu: Component<PropsWithChildren<MenuProps>> = (props) => {
	return (
		<UnstyledMenu orientation="vertical">
			{({ props: innerProps }) => (
				<nav {...innerProps} ref={props.ref} class="menu">
					<div className="menu__wrapper">
						<ul className="menu__list">{props.children}</ul>
					</div>
				</nav>
			)}
		</UnstyledMenu>
	);
};

export type MenuItemProps = Omit<UnstyledMenuItemProps, "children"> & {
	/**
	 * The label of this menu item
	 */
	label: string;
	/**
	 * The keybind to be displayed
	 */
	keybind?: string;
	/**
	 * Indicate if this menu item is checked
	 */
	checked?: boolean;
};

export const MenuItem: Component<MenuItemProps> = (props) => {
	return (
		<UnstyledMenuItem id={props.id} accessKey={props.accessKey} action={props.action}>
			{({ selected, opened, showAccessKey, props: innerProps }) => {
				const hasChildren = "children" in props;
				return (
					<li
						{...innerProps}
						class="menuitem"
						classList={{
							"menuitem--selected": selected(),
						}}
					>
						<div className="menuitem__wrapper">
							<div className="menuitem__icon">
								<Show when={props.checked}>
									<FontAwesomeIcon icon={faCheck} />
								</Show>
							</div>
							<div className="menuitem__label">
								<Show when={props.accessKey} fallback={props.label}>
									<>
										{props.label.split(props.accessKey).shift()}
										<span
											classList={{
												"menuitem__label--accesskey": showAccessKey(),
											}}
										>
											{props.accessKey}
										</span>
										{props.label.split(props.accessKey).slice(1).join(props.accessKey)}
									</>
								</Show>
							</div>
							<div className="menuitem__keybind">{props.keybind}</div>
							<div className="menuitem__icon">
								<Show when={hasChildren}>
									<FontAwesomeIcon icon={faChevronRight} />
								</Show>
							</div>
						</div>
						<Show when={hasChildren && opened()}>
							<Anchor constraints={anchorConstraints} class="menuitem__anchor">
								<NestedMenu>{props.children}</NestedMenu>
							</Anchor>
						</Show>
					</li>
				);
			}}
		</UnstyledMenuItem>
	);
};

const NestedMenu: Component = (props) => {
	const ctx = useContext(AnchorContext);
	const transform = () => ctx()?.transform ?? [HorizontalAlign.LEFT, VerticalAlign.TOP];
	return (
		<div
			class="menuitem__nested"
			classList={{
				"menuitem__nested--top": transform()[1] === VerticalAlign.TOP,
				"menuitem__nested--bottom": transform()[1] === VerticalAlign.BOTTOM,
			}}
		>
			{props.children}
		</div>
	);
};

const anchorConstraints: Constraints = {
	origins: [
		{
			anchor: [HorizontalAlign.RIGHT, VerticalAlign.TOP],
			transform: [HorizontalAlign.LEFT, VerticalAlign.TOP],
		},
		{
			anchor: [HorizontalAlign.LEFT, VerticalAlign.TOP],
			transform: [HorizontalAlign.RIGHT, VerticalAlign.TOP],
		},
		{
			anchor: [HorizontalAlign.RIGHT, VerticalAlign.BOTTOM],
			transform: [HorizontalAlign.LEFT, VerticalAlign.BOTTOM],
		},
		{
			anchor: [HorizontalAlign.LEFT, VerticalAlign.BOTTOM],
			transform: [HorizontalAlign.RIGHT, VerticalAlign.BOTTOM],
		},
	],
};

export const Separator = () => {
	return <li tabIndex={-1} className="flex p-0 pt-1 mt-0 mr-2 mb-1 ml-2 border-b border-gray-600" />;
};
