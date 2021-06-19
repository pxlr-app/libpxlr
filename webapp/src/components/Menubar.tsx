import React, {
	createContext,
	PropsWithChildren,
	useContext,
	useEffect,
	useReducer,
	useRef,
} from "react";

type ControlledMenubarAction =
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
	  }
	| {
			type: "ACCESSKEY";
			path: false | string[];
	  }
	| {
			type: "NAVIGATION";
			nav: MenuNav[];
	  };

type MenuNav = {
	id: string;
	accesskey: string;
};

type ControlledMenubarState = {
	accesskey: false | string[];
	selected: string[];
	opened: string[];
	navigation: MenuNav[];
};

function controlledMenubarReducer(
	state: ControlledMenubarState,
	action: ControlledMenubarAction,
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
			};
			break;
		case "NAVIGATION":
			state = { ...state, navigation: action.nav };
			break;
		default:
			throw new Error();
	}
	console.log(action.type, state);
	return state;
}

const ControlledMenubarContext = createContext<
	[ControlledMenubarState, React.Dispatch<ControlledMenubarAction>]
>([
	{
		accesskey: false,
		selected: [],
		opened: [],
		navigation: [],
	},
	() => {},
]);
const MenubarContext = createContext<{ addNavItem: (item: MenuNav) => void }>({
	addNavItem: () => {},
});
const MenubarItemContext = createContext<string[]>([]);

export function Menubar(props: PropsWithChildren<{}>) {
	const [_, dispatch] = useContext(ControlledMenubarContext);
	const navItems = useRef<MenuNav[]>([]);

	useEffect(() => {
		dispatch({ type: "NAVIGATION", nav: navItems.current });
		let onLeave = (e) => {
			dispatch({ type: "RESET" });
		};

		document.addEventListener("pointerup", onLeave);

		return () => {
			document.removeEventListener("pointerup", onLeave);
		};
	}, []);
	return (
		<div
			className="inline-flex p-0 m-0 border-0 outline-none overflow-visible bg-gray-900 text-gray-200 text-xs select-none"
			// onPointerLeave={(e) => {
			// 	dispatch({ type: "RESET" });
			// }}
		>
			<ul className="flex flex-row p-0 m-0 justify-end flex-nowrap">
				{React.Children.toArray(props.children).map((child, idx) => (
					<MenubarContext.Provider
						key={idx}
						value={{
							addNavItem(nav) {
								navItems.current.push(nav);
							},
						}}
					>
						{child}
					</MenubarContext.Provider>
				))}
			</ul>
		</div>
	);
}

export type ItemProps = {
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
};

export function MenubarItem(props: PropsWithChildren<ItemProps>) {
	const menubar = useContext(MenubarContext);
	const parentPath = useContext(MenubarItemContext);
	const currentPath = parentPath.concat([props.id]);
	const itemAbsolutePath = currentPath.join("/") + "/";
	const [state, dispatch] = useContext(ControlledMenubarContext);
	const navItem = {
		id: props.id,
		accesskey: props.accesskey,
	};
	useEffect(() => {
		menubar.addNavItem(navItem);
	}, []);
	return (
		<li
			className="relative"
			onPointerEnter={(e) =>
				dispatch({ type: "SELECT", path: currentPath })
			}
			// onPointerLeave={(e) => dispatch({ type: "OPEN", path: parentPath })}
		>
			<a
				href="#"
				// focus:border focus:border-blue-500 focus:bg-gray-700
				className={`flex flex-nowrap px-3 py-1 whitespace-nowrap border border-transparent ${
					state &&
					(state.selected.join("/") + "/").substr(
						0,
						itemAbsolutePath.length,
					) == itemAbsolutePath
						? "bg-gray-700"
						: ""
				}`}
				onClick={(e) => {
					e.preventDefault();
					dispatch({ type: "OPEN", path: currentPath });
				}}
				onPointerUp={(e) => {
					e.stopPropagation();
				}}
			>
				{props.accesskey ? (
					<>
						{props.label.split(props.accesskey).shift()}
						<span
							className={`${
								state && state.accesskey
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
			</a>
			{props.children &&
				(!state ||
					((state.opened.join("/") + "/").substr(
						0,
						itemAbsolutePath.length,
					) == itemAbsolutePath &&
						(state.selected.join("/") + "/").substr(
							0,
							itemAbsolutePath.length,
						) == itemAbsolutePath)) && (
					<MenubarItemContext.Provider value={currentPath}>
						<div className="absolute bottom-0 left-0">
							{props.children}
						</div>
					</MenubarItemContext.Provider>
				)}
		</li>
	);
}

export type ControlledMenubarProps = {
	/**
	 * The HTML Element to attach keyboard events to
	 */
	containerRef: HTMLElement;
};

/**
 * Controlled Menunbar component
 */
export function ControlledMenubar(
	props: PropsWithChildren<ControlledMenubarProps>,
) {
	const [state, dispatch] = useReducer(controlledMenubarReducer, {
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
					// TODO Select first navigation
					break;
				}
				default:
					if (state.accesskey) {
						let item = state.navigation.find(
							(item) =>
								`Key${item.accesskey.toUpperCase()}` === e.code,
						);
						if (item) {
							e.preventDefault();
							dispatch({
								type: "SELECT",
								path: state.opened.concat([item.id]),
							});
							dispatch({
								type: "OPEN",
								path: state.opened.concat([item.id]),
							});
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
		<ControlledMenubarContext.Provider value={[state, dispatch]}>
			{props.children}
		</ControlledMenubarContext.Provider>
	);
}
