import {
	children,
	Component,
	mergeProps,
	PropsWithChildren,
	Show,
} from "solid-js";
import { UnstyledMenu, UnstyledMenuItem } from "./UnstyledMenu";
import { faCheck, faChevronRight } from "@fortawesome/pro-regular-svg-icons";
import FontAwesomeIcon from "../shims/FontAwesomeIcon";
import "./Menu.css";

export type MenuProps = {
	ref?: HTMLElement | ((e: HTMLElement) => void);
};

export const Menu: Component<PropsWithChildren<MenuProps>> = (props) => {
	return (
		<UnstyledMenu orientation="vertical">
			{({ props: innerProps }) => (
				<nav
					{...innerProps}
					ref={props.ref}
					class="pointer-events-auto inline-block relative cursor-default shadow-hard border border-transparent outline-none bg-gray-700 text-gray-200 text-xs select-none focus:border-blue-500"
				>
					<div className="flex flex-1 p-0 m-0 overflow-visible">
						<ul className="flex flex-1 flex-col py-2 px-0 m-0 justify-end flex-nowrap">
							{props.children}
						</ul>
					</div>
				</nav>
			)}
		</UnstyledMenu>
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

	ref?: HTMLLIElement | ((e: HTMLLIElement) => void);
};

export const MenuItem: Component<MenuItemProps> = (props) => {
	const submenu = children(() => props.children);
	const hasChildren = !!submenu();
	return (
		<UnstyledMenuItem
			id={props.id}
			accessKey={props.accessKey}
			action={props.action}
			hasChildren={hasChildren}
		>
			{({ selected, opened, showAccessKey, props: innerProps }) => (
				<li
					{...innerProps}
					ref={props.ref}
					class="pointer-events-auto relative w-80 flex flex-1 pt-0.5 pb-1 px-1 mx-px cursor-pointer outline-none focus:bg-gray-800 focus:text-blue-300 focus-within:bg-gray-800 focus-within:text-blue-300"
					classList={{
						"bg-gray-800 text-blue-300": selected() === props.id,
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
											"underline uppercase": showAccessKey(),
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
						<div className="px-1 text-gray-500">
							{props.keybind}
						</div>
						<div className="w-4 text-center text-2xs">
							<Show when={hasChildren}>
								<FontAwesomeIcon icon={faChevronRight} />
							</Show>
						</div>
					</div>
					<Show when={hasChildren && opened() === props.id}>
						Blep
					</Show>
				</li>
			)}
		</UnstyledMenuItem>
	);
};

export const Separator: Component = () => {
	return (
		<li
			tabIndex={-1}
			className="flex p-0 pt-1 mt-0 mr-2 mb-1 ml-2 border-b border-gray-600"
		/>
	);
};
