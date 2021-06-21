import React, {
	useState,
	createContext,
	PropsWithChildren,
	useContext,
	useRef,
	useEffect,
} from "react";
import { faCheck, faChevronRight } from "@fortawesome/pro-regular-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import type { StandardLonghandProperties } from "csstype";

export const MenuContext = createContext<{
	showAccessKey: boolean;
	autoSelectFirst: boolean;
	setShowAccessKey: (state: boolean) => void;
}>({
	showAccessKey: false,
	autoSelectFirst: false,
	setShowAccessKey() {},
});

export type MenuProps = {
	/**
	 * Width of the Menu (see {@link StandardLonghandProperties.width})
	 */
	width?: number | string;
};

/**
 * Uncontrolled Menu component
 */
export function Menu({ width, children }: PropsWithChildren<MenuProps>) {
	const { showAccessKey, autoSelectFirst, setShowAccessKey } = useContext(
		MenuContext,
	);
	const [selected, setSelected] = useState<string | null>(null);
	const [opened, setOpened] = useState<string | null>(null);
	const [subAutoSelectFirst, setSubAutoSelectFirst] = useState<boolean>(
		false,
	);
	const element = useRef<HTMLDivElement>(null);

	const items: Pick<MenuItemProps, "id" | "accesskey" | "action">[] = [];
	React.Children.forEach(children, (item, idx) => {
		if (React.isValidElement(item) && "id" in item.props) {
			const id = item.props.id as string;
			const accesskey = item.props.accesskey as string;
			const action = item.props.action;
			items.push({
				id,
				accesskey,
				action,
			});
		}
	});

	useEffect(() => {
		if (autoSelectFirst && selected === null) {
			setSelected(items[0].id);
		}
	}, [autoSelectFirst]);

	useEffect(() => {
		function onLeave() {
			console.log("Menu Leaving?");
			setSelected(null);
			setOpened(null);
		}

		document.addEventListener("click", onLeave);
		document.addEventListener("keydown", onLeave);

		return () => {
			document.removeEventListener("click", onLeave);
			document.removeEventListener("keydown", onLeave);
		};
	}, []);

	return (
		<MenuContext.Provider
			value={{
				showAccessKey,
				autoSelectFirst: subAutoSelectFirst,
				setShowAccessKey,
			}}
		>
			<div
				tabIndex={0}
				className="pointer-events-auto absolute z-2000 cursor-default shadow-hard border border-transparent outline-none bg-gray-700 text-gray-200 text-xs select-none focus:border-blue-500"
				ref={element}
				style={{ width: width }}
				onKeyDown={(e) => {
					if (opened === null) {
						if (e.code === "ArrowDown") {
							e.stopPropagation();
							let selectedIdx = items.findIndex(
								(item) => item.id === selected,
							);
							selectedIdx = (selectedIdx + 1) % items.length;
							setSelected(items[selectedIdx].id);
						} else if (e.code === "ArrowUp") {
							e.stopPropagation();
							let selectedIdx = items.findIndex(
								(item) => item.id === selected,
							);
							if (selectedIdx === -1) {
								selectedIdx = items.length - 1;
							} else {
								selectedIdx =
									(items.length + (selectedIdx - 1)) %
									items.length;
							}
							setSelected(items[selectedIdx].id);
						} else if (e.code === "ArrowLeft") {
							setSelected(null);
							setOpened(null);
						} else if (showAccessKey) {
							const accessedItem = items.find(
								(item) =>
									`Key${item.accesskey.toUpperCase()}` ===
									e.code,
							);
							if (accessedItem) {
								if (accessedItem.action) {
									setSubAutoSelectFirst(false);
									setSelected(null);
									setOpened(null);
									setShowAccessKey(false);
									accessedItem.action();
								} else {
									e.stopPropagation();
									setSubAutoSelectFirst(true);
									setSelected(accessedItem.id);
									setOpened(accessedItem.id);
								}
							}
						}
					} else {
						if (e.code === "ArrowLeft") {
							e.stopPropagation();
							setSubAutoSelectFirst(false);
							setOpened(null);
						}
					}
				}}
			>
				<div className="flex flex-1 p-0 m-0 overflow-visible">
					<ul className="flex flex-1 flex-col py-2 px-0 m-0 justify-end flex-nowrap">
						{React.Children.map(children, (child, idx) => (
							<MenuItemContext.Provider
								key={`menuitem-${idx}`}
								value={{
									selectedId: selected,
									openedId: opened,
									setSelected(id) {
										setSelected(id);
									},
									setOpened(id) {
										if (id) {
											setSelected(id);
											setOpened(id);
										} else {
											setSelected(null);
											setOpened(null);
										}
									},
									setAutoSelectFirst() {
										setSubAutoSelectFirst(true);
									},
								}}
							>
								{child}
							</MenuItemContext.Provider>
						))}
					</ul>
				</div>
			</div>
		</MenuContext.Provider>
	);
}

export const MenuItemContext = createContext<{
	selectedId: string | null;
	openedId: string | null;
	setSelected: (id: string) => void;
	setAutoSelectFirst: () => void;
	setOpened: (id: string | false) => void;
}>({
	selectedId: null,
	openedId: null,
	setSelected: () => {},
	setAutoSelectFirst: () => {},
	setOpened: () => {},
});

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
	accesskey: string;
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

/**
 * Uncontrolled MenuItem component
 */
export function MenuItem({
	id,
	accesskey,
	action,
	checked,
	label,
	keybind,
	children,
}: PropsWithChildren<MenuItemProps>) {
	const { showAccessKey } = useContext(MenuContext);
	const {
		selectedId,
		openedId,
		setSelected,
		setAutoSelectFirst,
		setOpened,
	} = useContext(MenuItemContext);
	const element = useRef<HTMLLIElement>(null);

	useEffect(() => {
		if (selectedId === id && element.current) {
			element.current.focus();
		} else if (
			element.current &&
			document.activeElement == element.current
		) {
			element.current.blur();
		}
	}, [selectedId, openedId]);

	function onClick(e: React.MouseEvent | React.KeyboardEvent) {
		if (children) {
			e.stopPropagation();
			setOpened(openedId === id ? false : id);
		} else if (action) {
			setOpened(false);
			action();
		}
	}

	return (
		<li
			tabIndex={-1}
			accessKey={accesskey}
			className="pointer-events-auto relative flex flex-1 pt-0.5 pb-1 px-1 mx-px cursor-pointer outline-none focus:bg-gray-900 focus:text-blue-400 focus-within:bg-gray-900 focus-within:text-blue-400"
			ref={element}
			onPointerEnter={(e) => {
				setSelected(id);
			}}
			onFocus={(e) => {
				setSelected(id);
			}}
			onClick={onClick}
			onKeyDown={(e) => {
				if (
					e.code === "Space" ||
					e.code === "Enter" ||
					e.code === "ArrowRight"
				) {
					setAutoSelectFirst();
					onClick(e);
				}
			}}
		>
			<div className="pointer-events-none flex flex-1 flex-nowrap whitespace-nowrap ">
				<div className="w-4 text-center text-2xs">
					{checked && <FontAwesomeIcon icon={faCheck} />}
				</div>
				<div className="px-1 flex-1">
					{accesskey ? (
						<>
							{label.split(accesskey).shift()}
							<span
								className={`${
									showAccessKey ? "underline uppercase" : ""
								}`}
							>
								{accesskey}
							</span>
							{label.split(accesskey).slice(1).join(accesskey)}
						</>
					) : (
						label
					)}
				</div>
				<div className="px-1 text-gray-500">{keybind}</div>
				<div className="w-4 text-center text-2xs">
					{children && <FontAwesomeIcon icon={faChevronRight} />}
				</div>
			</div>
			{children && openedId === id && (
				<div className="absolute -top-2 right-0">{children}</div>
			)}
		</li>
	);
}

/**
 * Uncontrolled Separator component
 */
export function Separator() {
	return (
		<li
			tabIndex={-1}
			className="flex p-0 pt-1 mt-0 mr-2 mb-1 ml-2 border-b border-gray-600"
		></li>
	);
}

export type ControlledMenuProps = {
	/**
	 * The HTML Element to attach keyboard events to
	 */
	container: HTMLElement;
};

/**
 * Controlled Menu component
 */
export function ControlledMenu({
	children,
	container,
}: PropsWithChildren<ControlledMenuProps>) {
	const [showAccessKey, setShowAccessKey] = useState<boolean>(false);

	useEffect(() => {
		function onKeyDown(e: KeyboardEvent) {
			if (e.code === "AltLeft") {
				e.preventDefault();
				e.stopPropagation();
				setShowAccessKey(!showAccessKey);
			}
		}

		container.addEventListener("keydown", onKeyDown);

		return () => {
			container.removeEventListener("keydown", onKeyDown);
		};
	}, [container, showAccessKey, setShowAccessKey]);

	return (
		<MenuContext.Provider
			value={{
				showAccessKey,
				autoSelectFirst: showAccessKey,
				setShowAccessKey,
			}}
		>
			{children}
		</MenuContext.Provider>
	);
}
