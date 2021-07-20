import {
	Accessor,
	children,
	Component,
	JSX,
	mergeProps,
	PropsWithChildren,
	Show,
	useContext,
} from "solid-js";
import { UnstyledMenu, UnstyledMenuItem } from "./UnstyledMenu";
import { faCheck, faChevronRight } from "@fortawesome/pro-regular-svg-icons";
import FontAwesomeIcon from "../shims/FontAwesomeIcon";
import "./Menu.css";
import { createMenu, createMenuItem } from "./hooks";
import {
	Anchor,
	AnchorContext,
	Constraints,
	HorizontalAlign,
	VerticalAlign,
} from "../Anchor";

export type MenuProps = {
	ref?: HTMLElement | ((e: HTMLElement) => void);
};

export const Menu: Component<PropsWithChildren<MenuProps>> = (props) => {
	const menu = createMenu({ orientation: "vertical" });

	return (
		<nav
			{...menu.attributes}
			ref={props.ref}
			class="pointer-events-auto inline-block relative cursor-default shadow-hard border border-transparent outline-none bg-gray-700 text-gray-200 text-xs select-none focus:border-blue-500"
		>
			<div className="flex flex-1 p-0 m-0 overflow-visible">
				<ul className="flex flex-1 flex-col py-2 px-0 m-0 justify-end flex-nowrap">
					<menu.NestedContextProvider>
						{props.children}
					</menu.NestedContextProvider>
				</ul>
			</div>
		</nav>
	);
};

export type MenuItemProps = {
	/**
	 * A unique identifier for this menu item
	 */
	id: string;
	/**
	 * The label of this menu item
	 */
	label: string;
	/**
	 * The access key used for accessibility navigation
	 */
	accessKey: string;
	/**
	 * The keybind to be displayed
	 */
	keybind?: string;
	/**
	 * Indicate if this menu item is checked
	 */
	checked?: boolean;
	/**
	 * The action to execute when clicking on the menu item
	 */
	action?: () => void;
};

export const MenuItem: Component<MenuItemProps> = (props) => {
	const submenu = children(() => props.children) as () => JSX.Element;
	const hasChildren = !!submenu();
	const menuitem = createMenuItem({
		id: props.id,
		accessKey: props.accessKey,
		action: props.action,
	});
	return (
		<li
			{...menuitem.attributes}
			class="pointer-events-auto relative w-80 flex flex-1 pt-0.5 pb-1 px-1 mx-px cursor-pointer outline-none"
			classList={{
				"bg-gray-800 text-blue-300": menuitem.selected() === props.id,
			}}
		>
			<div className="pointer-events-none flex flex-1 flex-nowrap whitespace-nowrap ">
				<div className="w-4 text-center text-2xs">
					<Show when={props.checked}>
						<FontAwesomeIcon icon={faCheck} />
					</Show>
				</div>
				<div className="px-1 flex-1">
					<Show when={props.accessKey} fallback={props.label}>
						<>
							{props.label.split(props.accessKey).shift()}
							<span
								classList={{
									"underline uppercase": menuitem.showAccessKey(),
								}}
							>
								{props.accessKey}
							</span>
							{props.label
								.split(props.accessKey)
								.slice(1)
								.join(props.accessKey)}
						</>
					</Show>
				</div>
				<div className="px-1 text-gray-500">{props.keybind}</div>
				<div className="w-4 text-center text-2xs">
					<Show when={hasChildren}>
						<FontAwesomeIcon icon={faChevronRight} />
					</Show>
				</div>
			</div>
			<Show when={hasChildren && menuitem.opened() === props.id}>
				<Anchor constraints={anchorConstraints} class="z-50">
					{() => {
						const ctx = useContext(AnchorContext)();
						const transform = ctx?.transform ?? [
							HorizontalAlign.LEFT,
							VerticalAlign.TOP,
						];
						return (
							<div
								class="transform"
								classList={{
									"translate-y-[calc(-0.5rem-1px)]":
										transform[1] === VerticalAlign.TOP,
									"translate-y-[calc(0.5rem+1px)]":
										transform[1] === VerticalAlign.BOTTOM,
								}}
							>
								{submenu()}
							</div>
						);
					}}
				</Anchor>
			</Show>
		</li>
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
	return (
		<li
			tabIndex={-1}
			className="flex p-0 pt-1 mt-0 mr-2 mb-1 ml-2 border-b border-gray-600"
		/>
	);
};
