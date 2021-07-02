import React, { PropsWithChildren, useContext, useEffect, useRef } from "react";
import {
	MenuAlignmentContext,
	MenuContainer,
	MenuItemContainer,
} from "./MenuContainer";
import { faCheck, faChevronRight } from "@fortawesome/pro-regular-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { Anchor, Constraints, HorizontalAlign, VerticalAlign } from "../Anchor";

export type MenuProps = {
	/**
	 * Width of the Menu (see {@link StandardLonghandProperties.width})
	 */
	width?: number | string;
};

export function Menu({ children, ...props }: PropsWithChildren<MenuProps>) {
	return (
		<MenuContainer orientation="vertical">
			{({ elementRef, props }) => (
				<nav
					{...props}
					ref={elementRef}
					className="pointer-events-auto inline-block relative cursor-default shadow-hard border border-transparent outline-none bg-gray-700 text-gray-200 text-xs select-none focus:border-blue-500"
				>
					<div className="flex flex-1 p-0 m-0 overflow-visible">
						<ul className="flex flex-1 flex-col py-2 px-0 m-0 justify-end flex-nowrap">
							{children}
						</ul>
					</div>
				</nav>
			)}
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
	id,
	accessKey,
	action,
	checked,
	label,
	keybind,
}: PropsWithChildren<MenuItemProps>) {
	return (
		<MenuItemContainer
			id={id}
			accessKey={accessKey}
			action={action}
			hasChildren={!!children}
		>
			{({ showAccessKey, selected, opened, elementRef, props }) => (
				<li
					{...props}
					ref={elementRef as any}
					className={[
						"pointer-events-auto relative w-80 flex flex-1 pt-0.5 pb-1 px-1 mx-px cursor-pointer outline-none focus:bg-gray-800 focus:text-blue-300 focus-within:bg-gray-800 focus-within:text-blue-300",
						selected && "bg-gray-800 text-blue-300",
					].join(" ")}
				>
					<div className="pointer-events-none flex flex-1 flex-nowrap whitespace-nowrap ">
						<div className="w-4 text-center text-2xs">
							{checked && <FontAwesomeIcon icon={faCheck} />}
						</div>
						<div className="px-1 flex-1">
							{accessKey ? (
								<>
									{label.split(accessKey).shift()}
									<span
										className={`${
											showAccessKey
												? "underline uppercase"
												: ""
										}`}
									>
										{accessKey}
									</span>
									{label
										.split(accessKey)
										.slice(1)
										.join(accessKey)}
								</>
							) : (
								label
							)}
						</div>
						<div className="px-1 text-gray-500">{keybind}</div>
						<div className="w-4 text-center text-2xs">
							{children && (
								<FontAwesomeIcon icon={faChevronRight} />
							)}
						</div>
					</div>
					{children && opened && (
						<Anchor className="z-50" constraints={MENU_CONSTRAINTS}>
							{({ transformRef, transformOrigin }) => (
								<div ref={transformRef as any}>
									<div
										className={`transform ${
											transformOrigin[1] ===
											VerticalAlign.TOP
												? "translate-y-[calc(-0.5rem-1px)]"
												: "translate-y-[calc(0.5rem+1px)]"
										}`}
									>
										<MenuAlignmentContext.Provider
											value={{
												alignment: transformOrigin,
											}}
										>
											{children}
										</MenuAlignmentContext.Provider>
									</div>
								</div>
							)}
						</Anchor>
					)}
				</li>
			)}
		</MenuItemContainer>
	);
}

const MENU_CONSTRAINTS: Constraints = {
	preventOverlap: true,
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

export function Separator() {
	return (
		<li
			tabIndex={-1}
			className="flex p-0 pt-1 mt-0 mr-2 mb-1 ml-2 border-b border-gray-600"
		></li>
	);
}
