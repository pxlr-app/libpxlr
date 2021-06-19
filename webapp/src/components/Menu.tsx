import React, {
	createContext,
	PropsWithChildren,
	useEffect,
	useContext,
	useReducer,
	useRef,
} from "react";
import { faCheck, faChevronRight } from "@fortawesome/pro-regular-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import type { StandardLonghandProperties } from "csstype";

type MenuNav = {
	id: string;
	accesskey: string;
	isLeaf: boolean;
	action: MenuItemProps["action"];
};

type ControlledMenuAction =
	| {
			type: "RESET";
	  }
	| {
			type: "SELECT";
			path: string[];
	  }
	| {
			type: "OPEN";
			path: string[];
			autoSelectFirst?: boolean;
	  }
	| {
			type: "ACCESSKEY";
			path: false | string[];
	  }
	| {
			type: "NAV_PUSH";
			nav: MenuNav[];
	  }
	| {
			type: "NAV_POP";
	  };

type ControlledMenuState = {
	accesskey: false | string[];
	selected: string[];
	opened: string[];

	navigation: MenuNav[][];
	autoSelectFirst?: boolean;
};

function controlledMenuReducer(
	state: ControlledMenuState,
	action: ControlledMenuAction,
) {
	switch (action.type) {
		case "RESET":
			state = {
				accesskey: false,
				selected: [],
				opened: [],
				navigation: state.navigation,
			};
			break;
		case "ACCESSKEY":
			state = { ...state, accesskey: action.path };
			break;
		case "SELECT":
			state = { ...state, selected: action.path };
			break;
		case "OPEN":
			state = {
				...state,
				opened: action.path,
				autoSelectFirst: action.autoSelectFirst,
			};
			break;
		case "NAV_PUSH": {
			let queue = state.navigation.concat([]);
			queue.unshift(action.nav);
			if (state.autoSelectFirst) {
				state = {
					...state,
					navigation: queue,
					autoSelectFirst: undefined,
					selected: state.opened.concat(action.nav[0].id),
				};
			} else {
				state = { ...state, navigation: queue };
			}
			break;
		}
		case "NAV_POP": {
			let queue = state.navigation.concat([]);
			queue.shift();
			state = { ...state, navigation: queue };
			break;
		}
		default:
			throw new Error();
	}
	return state;
}

const ControlledMenuContext = createContext<
	[ControlledMenuState, React.Dispatch<ControlledMenuAction>]
>([
	{
		accesskey: false,
		selected: [],
		opened: [],
		navigation: [],
	},
	() => {},
]);
const MenuContext = createContext<{ addNavItem: (item: MenuNav) => void }>({
	addNavItem: () => {},
});
const MenuItemContext = createContext<string[]>([]);

export type MenuProps = {
	/**
	 * Width of the Menu (see {@link StandardLonghandProperties.width})
	 */
	width?: number | string;
};

/**
 * Uncontrolled Menu component
 */
export function Menu(props: PropsWithChildren<MenuProps>) {
	const [_, dispatch] = useContext(ControlledMenuContext);
	const navItems = useRef<MenuNav[]>([]);

	useEffect(() => {
		dispatch({ type: "NAV_PUSH", nav: navItems.current });
		return () => {
			dispatch({ type: "NAV_POP" });
		};
	}, []);
	return (
		<div
			className="absolute z-2000 shadow-hard border-0 outline-none bg-gray-700 text-gray-200 text-xs select-none"
			style={{ width: props.width }}
			onPointerLeave={(e) => {
				dispatch({ type: "RESET" });
			}}
		>
			<div className="flex flex-1 p-0 m-0 overflow-visible">
				<ul className="flex flex-1 flex-col py-2 px-0 m-0 justify-end flex-nowrap">
					{React.Children.toArray(props.children).map(
						(child, idx) => (
							<MenuContext.Provider
								key={idx}
								value={{
									addNavItem(nav) {
										navItems.current.push(nav);
									},
								}}
							>
								{child}
							</MenuContext.Provider>
						),
					)}
				</ul>
			</div>
		</div>
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
export function MenuItem(props: PropsWithChildren<MenuItemProps>) {
	const menu = useContext(MenuContext);
	const parentPath = useContext(MenuItemContext);
	const parentAbsolutePath = parentPath.join("/") + "/";
	const currentPath = parentPath.concat([props.id]);
	const itemAbsolutePath = currentPath.join("/") + "/";
	const [state, dispatch] = useContext(ControlledMenuContext);
	const navItem = {
		id: props.id,
		accesskey: props.accesskey,
		isLeaf: !props.children,
		action: props.action,
	};
	useEffect(() => {
		menu.addNavItem(navItem);
	}, []);
	return (
		<li
			className={`relative mx-px ${
				state &&
				(state.selected.join("/") + "/").substr(
					0,
					itemAbsolutePath.length,
				) == itemAbsolutePath
					? "bg-gray-900 text-blue-400"
					: ""
			}`}
			onPointerEnter={(e) =>
				dispatch({ type: "SELECT", path: currentPath })
			}
			onPointerLeave={(e) => dispatch({ type: "OPEN", path: parentPath })}
		>
			<a
				href="#"
				className="flex flex-1 flex-nowrap whitespace-nowrap pt-0.5 pb-1 px-1"
				onClick={(e) => {
					e.preventDefault();
				}}
				onPointerUp={(e) => {
					if (navItem.isLeaf) {
						props.action && props.action();
						dispatch({ type: "RESET" });
					} else {
						e.stopPropagation();
						dispatch({ type: "OPEN", path: currentPath });
					}
				}}
			>
				<div className="w-4 text-center text-2xs">
					{props.checked && <FontAwesomeIcon icon={faCheck} />}
				</div>
				<div className="px-1 flex-1">
					{props.accesskey ? (
						<>
							{props.label.split(props.accesskey).shift()}
							<span
								className={`${
									state &&
									state.accesskey &&
									(state.opened.join("/") + "/").substr(
										0,
										parentAbsolutePath.length,
									) == parentAbsolutePath
										? "underline uppercase"
										: ""
								}`}
							>
								{props.accesskey}
							</span>
							{props.label
								.split(props.accesskey)
								.slice(1)
								.join(props.accesskey)}
						</>
					) : (
						props.label
					)}
				</div>
				<div className="px-1 text-gray-500">{props.keybind}</div>
				<div className="w-4 text-center text-2xs">
					{props.children && (
						<FontAwesomeIcon icon={faChevronRight} />
					)}
				</div>
			</a>
			{props.children &&
				(!state ||
					(state.opened.join("/") + "/").substr(
						0,
						itemAbsolutePath.length,
					) == itemAbsolutePath) && (
					<MenuItemContext.Provider value={currentPath}>
						<div className="absolute -top-2 right-0">
							{props.children}
						</div>
					</MenuItemContext.Provider>
				)}
		</li>
	);
}

/**
 * Uncontrolled Separator component
 */
export function Separator() {
	return (
		<li className="flex p-0 pt-1 mt-0 mr-2 mb-1 ml-2 border-b border-gray-600"></li>
	);
}

export type ControlledMenuProps = {
	/**
	 * The HTML Element to attach keyboard events to
	 */
	containerRef: HTMLElement;
};

/**
 * Controlled Menu component
 */
export function ControlledMenu(props: PropsWithChildren<ControlledMenuProps>) {
	const [state, dispatch] = useReducer(controlledMenuReducer, {
		accesskey: false,
		selected: [],
		opened: [],
		navigation: [],
	});
	useEffect(() => {
		const onKeyDown = (e: KeyboardEvent) => {
			switch (e.code) {
				case "AltLeft": {
					e.preventDefault();
					dispatch({
						type: "ACCESSKEY",
						path: state.accesskey !== false ? false : state.opened,
					});
					break;
				}
				case "ArrowUp": {
					e.preventDefault();
					let idx: number;
					if (state.selected.length === 0) {
						idx = state.navigation[0].length - 1;
					} else {
						let currentSelected =
							state.selected[state.selected.length - 1];
						idx = state.navigation[0].findIndex(
							(props) => props.id === currentSelected,
						);
						idx = Math.max(0, idx - 1);
					}
					let selected = state.opened.concat([
						state.navigation[0][idx].id,
					]);
					dispatch({ type: "SELECT", path: selected });
					break;
				}
				case "ArrowDown": {
					e.preventDefault();
					let idx: number;
					if (state.selected.length === 0) {
						idx = 0;
					} else {
						let currentSelected =
							state.selected[state.selected.length - 1];
						idx = state.navigation[0].findIndex(
							(props) => props.id === currentSelected,
						);
						idx = Math.min(state.navigation[0].length - 1, idx + 1);
					}
					let selected = state.opened.concat([
						state.navigation[0][idx].id,
					]);
					dispatch({ type: "SELECT", path: selected });
					break;
				}
				case "ArrowLeft":
				case "Escape": {
					e.preventDefault();
					if (state.navigation.length > 1) {
						let parentSelected = state.selected.slice(0, -1);
						dispatch({
							type: "SELECT",
							path: parentSelected,
						});
						dispatch({
							type: "OPEN",
							path: parentSelected.slice(0, -1),
						});
					} else {
						// TODO onBlur?
						dispatch({ type: "RESET" });
					}
					break;
				}
				case "ArrowRight":
				case "Enter": {
					e.preventDefault();
					let currentSelected =
						state.selected[state.selected.length - 1];
					let item = state.navigation[0].find(
						(props) => props.id === currentSelected,
					);
					if (item) {
						if (item.isLeaf) {
							item.action && item.action();
							dispatch({ type: "RESET" });
						} else {
							dispatch({
								type: "OPEN",
								path: state.selected,
								autoSelectFirst: true,
							});
						}
					}
					break;
				}
				default:
					if (state.accesskey) {
						let item = state.navigation[0].find(
							(item) =>
								`Key${item.accesskey.toUpperCase()}` === e.code,
						);
						if (item) {
							e.preventDefault();
							if (item.isLeaf) {
								item.action && item.action();
								dispatch({ type: "RESET" });
							} else {
								dispatch({
									type: "OPEN",
									path: state.opened.concat([item.id]),
									autoSelectFirst: true,
								});
							}
						}
					}
			}
		};

		props.containerRef.addEventListener("keydown", onKeyDown);

		return () => {
			props.containerRef.removeEventListener("keydown", onKeyDown);
		};
	}, [props.containerRef, state, dispatch]);
	return (
		<ControlledMenuContext.Provider value={[state, dispatch]}>
			{props.children}
		</ControlledMenuContext.Provider>
	);
}
