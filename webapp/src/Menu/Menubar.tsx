import React, { PropsWithChildren, useContext, useEffect, useRef } from "react";
import { Anchor, Constraints, HorizontalAlign, VerticalAlign } from "../Anchor";
import {
	MenuAlignmentContext,
	MenuContainer,
	MenuItemContainer,
} from "./MenuContainer";

export type MenubarProps = {
	/**
	 * Width of the Menu (see {@link StandardLonghandProperties.width})
	 */
	width?: number | string;
};

export function Menubar({
	children,
	...props
}: PropsWithChildren<MenubarProps>) {
	return (
		<MenuContainer orientation="horizontal">
			{({ elementRef, props }) => (
				<nav
					{...props}
					ref={elementRef}
					className="pointer-events-auto cursor-default inline-flex p-0 m-0 border border-transparent outline-none overflow-visible bg-gray-900 text-gray-200 text-xs select-none focus:border-blue-500"
				>
					<ul className="flex flex-row p-0 m-0 justify-end flex-nowrap">
						{children}
					</ul>
				</nav>
			)}
		</MenuContainer>
	);
}

export type MenubarItemProps = {
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

export function MenubarItem({
	children,
	id,
	accessKey,
	action,
	checked,
	label,
	keybind,
}: PropsWithChildren<MenubarItemProps>) {
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
						"pointer-events-auto relative outline-none focus:bg-gray-700 focus-within:bg-gray-700",
						selected && "bg-gray-700",
					].join(" ")}
				>
					<div className="pointer-events-none flex flex-nowrap px-3 py-1 whitespace-nowrap">
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
					{children && opened && (
						<Anchor
							className="z-50"
							constraints={MENUBAR_CONSTRAINTS}
						>
							{({ transformRef, transformOrigin }) => (
								<div ref={transformRef as any}>
									<MenuAlignmentContext.Provider
										value={{
											alignment: transformOrigin,
										}}
									>
										{children}
									</MenuAlignmentContext.Provider>
								</div>
							)}
						</Anchor>
					)}
				</li>
			)}
		</MenuItemContainer>
	);
}

const MENUBAR_CONSTRAINTS: Constraints = {
	preventOverlap: true,
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
