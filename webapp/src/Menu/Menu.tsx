import React, { PropsWithChildren, useContext, useEffect, useRef } from "react";
import {
	MenuContainer,
	MenuContext,
	MenuGlobalContext,
	MenuItemContainer,
	MenuItemContainerProps,
	MenuItemContext,
	MenuItemContextData,
} from "./MenuContainer";
import { faCheck, faChevronRight } from "@fortawesome/pro-regular-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

export type MenuProps = {
	/**
	 * Width of the Menu (see {@link StandardLonghandProperties.width})
	 */
	width?: number | string;
};

export function Menu({ children, ...props }: PropsWithChildren<MenuProps>) {
	return (
		<MenuContainer>
			<MenuContext.Consumer>
				{({ onKeyDown }) => (
					<nav
						tabIndex={0}
						className="pointer-events-auto absolute z-2000 cursor-default shadow-hard border border-transparent outline-none bg-gray-700 text-gray-200 text-xs select-none focus:border-blue-500"
						style={{ width: props.width }}
						onKeyDown={onKeyDown}
					>
						<div className="flex flex-1 p-0 m-0 overflow-visible">
							<ul className="flex flex-1 flex-col py-2 px-0 m-0 justify-end flex-nowrap">
								{children}
							</ul>
						</div>
					</nav>
				)}
			</MenuContext.Consumer>
		</MenuContainer>
	);
}

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

export function MenuItem({
	children,
	...props
}: PropsWithChildren<MenuItemProps>) {
	const { showAccessKey } = useContext(MenuGlobalContext);

	function InnerMenuItem(
		props: PropsWithChildren<MenuItemProps & MenuItemContextData>,
	) {
		const { selected, opened } = useContext(MenuContext);
		const element = useRef<HTMLLIElement>(null);

		useEffect(() => {
			if (selected === props.id && element.current) {
				element.current.focus();
			} else if (
				element.current &&
				document.activeElement == element.current
			) {
				element.current.blur();
			}
		}, [selected, opened]);

		return (
			<li
				ref={element}
				tabIndex={-1}
				accessKey={props.accessKey}
				aria-label={props.label}
				className="pointer-events-auto relative flex flex-1 pt-0.5 pb-1 px-1 mx-px cursor-pointer outline-none focus:bg-gray-900 focus:text-blue-400 focus-within:bg-gray-900 focus-within:text-blue-400"
				onPointerEnter={props.onPointerEnter}
				onFocus={props.onFocus}
				onClick={props.onClick}
				onKeyDown={props.onKeyDown}
			>
				<div className="pointer-events-none flex flex-1 flex-nowrap whitespace-nowrap ">
					<div className="w-4 text-center text-2xs">
						{props.checked && <FontAwesomeIcon icon={faCheck} />}
					</div>
					<div className="px-1 flex-1">
						{props.accessKey ? (
							<>
								{props.label.split(props.accessKey).shift()}
								<span
									className={`${
										showAccessKey
											? "underline uppercase"
											: ""
									}`}
								>
									{props.accessKey}
								</span>
								{props.label
									.split(props.accessKey)
									.slice(1)
									.join(props.accessKey)}
							</>
						) : (
							props.label
						)}
					</div>
					<div className="px-1 text-gray-500">{props.keybind}</div>
					<div className="w-4 text-center text-2xs">
						{children && <FontAwesomeIcon icon={faChevronRight} />}
					</div>
				</div>
				{children && opened === props.id && (
					<div className="absolute -top-2 right-0 transform -translate-y-px">
						{children}
					</div>
				)}
			</li>
		);
	}

	return (
		<MenuItemContainer
			id={props.id}
			accessKey={props.accessKey}
			action={props.action}
			hasChildren={!!children}
		>
			<MenuItemContext.Consumer>
				{(context) => (
					<InnerMenuItem {...props} {...context}>
						{children}
					</InnerMenuItem>
				)}
			</MenuItemContext.Consumer>
		</MenuItemContainer>
	);
}

export function Separator() {
	return (
		<li
			tabIndex={-1}
			className="flex p-0 pt-1 mt-0 mr-2 mb-1 ml-2 border-b border-gray-600"
		></li>
	);
}
