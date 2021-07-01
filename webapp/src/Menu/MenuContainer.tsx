import React, {
	createContext,
	createRef,
	PropsWithChildren,
	useContext,
	useEffect,
	useMemo,
	useRef,
	useState,
} from "react";

export type AccessibleMenuData = {
	showAccessKey: boolean;
	setShowAccessKey: React.Dispatch<React.SetStateAction<boolean>>;
	navigationMethod: "pointer" | "keyboard";
	setNavigationMethod: React.Dispatch<
		React.SetStateAction<"pointer" | "keyboard">
	>;
};

export const AccessibleMenuContext = createContext<AccessibleMenuData>({
	showAccessKey: false,
	setShowAccessKey() {},
	navigationMethod: "pointer",
	setNavigationMethod() {},
});

export type AccessibleMenuContainerProps = {
	/**
	 * The HTML Element to attach keyboard events to
	 */
	container: HTMLElement;
};

export type MenuData = AccessibleMenuData & {
	orientation: "horizontal" | "vertical";

	selected: string | null;
	opened: string | null;
	setSelected: React.Dispatch<React.SetStateAction<string | null>>;
	setOpened: React.Dispatch<React.SetStateAction<string | null>>;
	elementRef: React.Ref<HTMLElement>;

	props: {
		tabIndex: number;
		onKeyDown: (event: React.KeyboardEvent) => void;
	};
};

export const MenuContext = createContext<
	Omit<MenuData, "elementRef" | "props"> | undefined
>(undefined);

export type MenuContainerProps = {
	orientation: "horizontal" | "vertical";
	children: (context: MenuData) => React.ReactNode;
};

export type MenuItemData = AccessibleMenuData & {
	orientation: "horizontal" | "vertical";

	selected: boolean;
	opened: boolean;
	setSelected: React.Dispatch<React.SetStateAction<string | null>>;
	setOpened: React.Dispatch<React.SetStateAction<string | null>>;
	elementRef: React.Ref<HTMLElement>;

	props: {
		tabIndex: number;
		onPointerEnter: (event: React.PointerEvent) => void;
		onFocus: (event: React.FocusEvent) => void;
		onClick: (event: React.MouseEvent) => void;
		onKeyDown: (event: React.KeyboardEvent) => void;
	};
};

export type MenuItemContainerProps = {
	id: string;
	accessKey: string;
	action?: () => void;
	hasChildren: boolean;
	children: (context: MenuItemData) => React.ReactNode;
};

export function MenuContainer({ children, orientation }: MenuContainerProps) {
	const {
		showAccessKey,
		setShowAccessKey,
		setNavigationMethod,
		navigationMethod,
	} = useContext(AccessibleMenuContext);
	const [selected, setSelected] = useState<string | null>(null);
	const [opened, setOpened] = useState<string | null>(null);
	const elementRef = useRef<HTMLElement>();

	useEffect(() => {
		function onLeave() {
			setNavigationMethod("pointer");
			setSelected(null);
			setOpened(null);
		}

		document.addEventListener("click", onLeave);
		document.addEventListener("keydown", onLeave);

		return () => {
			document.removeEventListener("click", onLeave);
			document.removeEventListener("keydown", onLeave);
		};
	}, [setShowAccessKey]);

	const data = useMemo<MenuData>(
		() => ({
			orientation,
			showAccessKey,
			navigationMethod,
			selected,
			opened,
			elementRef(element) {
				if (element) {
					elementRef.current = element;
					element.setAttribute("role", "menu");

					if (!selected && navigationMethod === "keyboard") {
						const menuitems = Array.from(
							element.querySelectorAll(
								":not([role='menu']) [role='menuitem']",
							),
						);
						if (menuitems.length > 0) {
							setSelected((menuitems[0] as any)._menuitem_id);
							setOpened(null);
						}
					}
				}
			},
			setShowAccessKey,
			setNavigationMethod,
			setSelected,
			setOpened,

			props: {
				tabIndex: 0,
				onKeyDown(e) {
					if (e.code === "AltLeft") {
						e.preventDefault();
						e.stopPropagation();
						setShowAccessKey((state) => !state);
						setNavigationMethod("keyboard");
					} else if (!opened) {
						const menuitems = Array.from(
							elementRef.current!.querySelectorAll(
								":not([role='menu']) [role='menuitem']",
							),
						).map((menuitem) => ({
							id: (menuitem as any)._menuitem_id,
							accessKey: (menuitem as any)._menuitem_accessKey,
							action: (menuitem as any)._menuitem_action,
							element: menuitem,
						}));
						let selectedIdx = menuitems.findIndex(
							(menuitem) => menuitem.id === selected,
						);

						if (
							(orientation === "vertical" &&
								e.code === "ArrowDown") ||
							(orientation === "horizontal" &&
								e.code === "ArrowRight")
						) {
							e.preventDefault();
							e.stopPropagation();
							selectedIdx = (selectedIdx + 1) % menuitems.length;
							setNavigationMethod("keyboard");
							setSelected(menuitems[selectedIdx].id);
						} else if (
							(orientation === "vertical" &&
								e.code === "ArrowUp") ||
							(orientation === "horizontal" &&
								e.code === "ArrowLeft")
						) {
							e.preventDefault();
							e.stopPropagation();
							if (selectedIdx === -1) {
								selectedIdx = menuitems.length - 1;
							} else {
								selectedIdx =
									(menuitems.length + (selectedIdx - 1)) %
									menuitems.length;
							}
							setNavigationMethod("keyboard");
							setSelected(menuitems[selectedIdx].id);
						} else if (
							(orientation === "vertical" &&
								e.code === "ArrowLeft") ||
							(orientation === "horizontal" &&
								e.code === "ArrowUp")
						) {
							setNavigationMethod("keyboard");
							setSelected(null);
							setOpened(null);
						} else if (showAccessKey) {
							const accessedItem = menuitems.find(
								({ id, accessKey }) =>
									`Key${accessKey.toUpperCase()}` === e.code,
							);
							if (accessedItem) {
								if (accessedItem.action) {
									setShowAccessKey(false);
									setNavigationMethod("keyboard");
									setSelected(null);
									setOpened(null);
									accessedItem.action();
								} else {
									e.preventDefault();
									e.stopPropagation();
									setNavigationMethod("keyboard");
									setSelected(accessedItem.id);
									setOpened(accessedItem.id);
								}
							}
						}
					} else if (
						(orientation === "vertical" &&
							e.code === "ArrowLeft") ||
						(orientation === "horizontal" && e.code === "ArrowUp")
					) {
						e.preventDefault();
						e.stopPropagation();
						setNavigationMethod("keyboard");
						setOpened(null);
					}
				},
			},
		}),
		[orientation, showAccessKey, navigationMethod, selected, opened],
	);

	return useMemo(
		() => (
			<MenuContext.Provider value={data}>
				{children(data)}
			</MenuContext.Provider>
		),
		[data],
	);
}

export function MenuItemContainer({
	children,
	id,
	accessKey,
	hasChildren,
	action,
}: MenuItemContainerProps) {
	const menuContext = useContext(MenuContext);
	if (!menuContext) {
		throw new Error(
			"MenuItemContainer needs to be a descendant of MenuContainer.",
		);
	}

	const {
		showAccessKey,
		setShowAccessKey,
		navigationMethod,
		setNavigationMethod,
	} = useContext(AccessibleMenuContext);

	const {
		orientation,
		selected,
		opened,
		setSelected,
		setOpened,
	} = menuContext;
	const elementRef = useRef<HTMLElement>();

	const data = useMemo<MenuItemData>(() => {
		function onClick(e: React.MouseEvent | React.KeyboardEvent) {
			// Not bubbling up
			if (e.target === e.currentTarget) {
				// Has children
				if (hasChildren) {
					e.preventDefault();
					e.stopPropagation();
					if (opened === id) {
						setSelected(null);
						setOpened(null);
					} else {
						setSelected(id);
						setOpened(id);
					}
				}
				// Has action
				else if (action) {
					setSelected(null);
					setOpened(null);
					action();
				}
			}
		}
		return {
			orientation,
			showAccessKey,
			navigationMethod,
			selected: selected === id,
			opened: opened === id,
			elementRef(element) {
				if (element) {
					elementRef.current = element;
					element.setAttribute("role", "menuitem");
					(element as any)._menuitem_id = id;
					(element as any)._menuitem_accessKey = accessKey;
					(element as any)._menuitem_action = action;

					if (selected === id) {
						element.focus();
					} else if (element === document.activeElement) {
						element.blur();
					}
				}
			},
			setShowAccessKey,
			setNavigationMethod,
			setSelected,
			setOpened,
			props: {
				tabIndex: -1,
				onPointerEnter(e) {
					setNavigationMethod("pointer");
					setSelected(id);
					if (opened !== id) {
						setOpened(null);
					}
				},
				onFocus(e) {
					setSelected(id);
					if (opened !== id) {
						setOpened(null);
					}
				},
				onClick(e) {
					setNavigationMethod("pointer");
					onClick(e);
				},
				onKeyDown(e) {
					if (e.code === "AltLeft") {
						e.preventDefault();
						e.stopPropagation();
						setShowAccessKey((state) => !state);
						setNavigationMethod("keyboard");
					} else if (
						e.code === "Space" ||
						e.code === "Enter" ||
						(orientation === "vertical" &&
							e.code === "ArrowRight") ||
						(orientation === "horizontal" && e.code === "ArrowDown")
					) {
						setNavigationMethod("keyboard");
						onClick(e);
					}
				},
			},
		};
	}, [
		id,
		orientation,
		showAccessKey,
		navigationMethod,
		selected === id,
		opened === id,
	]);

	return useMemo(() => <>{children(data)}</>, [data]);
}

export function AccessibleMenuContainer({
	children,
	container,
}: PropsWithChildren<AccessibleMenuContainerProps>) {
	const [showAccessKey, setShowAccessKey] = useState<boolean>(false);
	const [navigationMethod, setNavigationMethod] = useState<
		"pointer" | "keyboard"
	>("pointer");

	const data = useMemo(
		() => ({
			showAccessKey,
			setShowAccessKey,
			navigationMethod,
			setNavigationMethod,
		}),
		[showAccessKey, navigationMethod],
	);

	useEffect(() => {
		function onKeyDown(e: KeyboardEvent) {
			if (e.code === "AltLeft") {
				e.preventDefault();
				e.stopPropagation();
				setShowAccessKey((state) => !state);
				setNavigationMethod("keyboard");
			}
		}

		container.addEventListener("keydown", onKeyDown);

		return () => {
			container.removeEventListener("keydown", onKeyDown);
		};
	}, [container]);

	return useMemo(
		() => (
			<AccessibleMenuContext.Provider value={data}>
				{children}
			</AccessibleMenuContext.Provider>
		),
		[data],
	);
}
