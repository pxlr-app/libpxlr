import React, {
	createContext,
	createRef,
	PropsWithChildren,
	useContext,
	useEffect,
	useRef,
	useState,
} from "react";

export const MenuGlobalContext = createContext<{
	showAccessKey: boolean;
	setShowAccessKey: (state: boolean) => void;
	autoSelectFirst: React.MutableRefObject<boolean>;
}>({
	showAccessKey: false,
	setShowAccessKey() {},
	autoSelectFirst: createRef<boolean>() as React.MutableRefObject<boolean>,
});

export type MenuContextData = {
	addOrUpdateMenuItem: (ref: object, item: MenuItemState) => void;
	removeMenuItem: (ref: object) => void;
	onKeyDown: (event: React.KeyboardEvent) => void;

	selected: string | null;
	opened: string | null;
	setSelected: (id: string | null) => void;
	setOpened: (id: string | null) => void;
};

export const MenuContext = createContext<MenuContextData>({
	addOrUpdateMenuItem() {},
	removeMenuItem() {},
	onKeyDown() {},

	selected: null,
	opened: null,
	setSelected() {},
	setOpened() {},
});

type MenuItemState = Pick<
	MenuItemContainerProps,
	"id" | "accessKey" | "action"
>;

export type MenuContainerProps = {};

export function MenuContainer({
	children,
}: PropsWithChildren<MenuContainerProps>) {
	const { showAccessKey, setShowAccessKey, autoSelectFirst } = useContext(
		MenuGlobalContext,
	);
	const [selected, setSelected] = useState<string | null>(null);
	const [opened, setOpened] = useState<string | null>(null);
	const items = useRef<Map<object, MenuItemState>>(new Map());

	useEffect(() => {
		if (autoSelectFirst.current) {
			console.log("autoselect", items.current.size);
			autoSelectFirst.current = false;
			if (selected === null) {
				const entries = Array.from(items.current.entries());
				setSelected(entries[0][1].id);
			}
		}
	}, [autoSelectFirst, selected, setSelected]);

	useEffect(() => {
		function onLeave() {
			// setShowAccessKey(false);
			autoSelectFirst.current = false;
			setSelected(null);
			setOpened(null);
		}

		document.addEventListener("click", onLeave);
		document.addEventListener("keydown", onLeave);

		return () => {
			document.removeEventListener("click", onLeave);
			document.removeEventListener("keydown", onLeave);
		};
	}, [setShowAccessKey, autoSelectFirst]);

	return (
		<MenuContext.Provider
			value={{
				selected,
				setSelected,
				opened,
				setOpened,
				addOrUpdateMenuItem(ref, item) {
					items.current.set(ref, item);
				},
				removeMenuItem(ref) {
					items.current.delete(ref);
				},
				onKeyDown(e) {
					if (opened === null) {
						if (e.code === "ArrowDown") {
							e.preventDefault();
							e.stopPropagation();
							const entries = Array.from(items.current.entries());
							let selectedIdx = entries.findIndex(
								([, item]) => item.id === selected,
							);
							selectedIdx = (selectedIdx + 1) % entries.length;
							setSelected(entries[selectedIdx][1].id);
						} else if (e.code === "ArrowUp") {
							e.preventDefault();
							e.stopPropagation();
							const entries = Array.from(items.current.entries());
							let selectedIdx = entries.findIndex(
								([, item]) => item.id === selected,
							);
							if (selectedIdx === -1) {
								selectedIdx = entries.length - 1;
							} else {
								selectedIdx =
									(entries.length + (selectedIdx - 1)) %
									entries.length;
							}
							setSelected(entries[selectedIdx][1].id);
						} else if (e.code === "ArrowLeft") {
							setSelected(null);
							setOpened(null);
						} else if (showAccessKey) {
							const entries = Array.from(items.current.entries());
							const accessedItem = entries.find(
								([, item]) =>
									`Key${item.accessKey.toUpperCase()}` ===
									e.code,
							);
							if (accessedItem) {
								if (accessedItem[1].action) {
									setShowAccessKey(false);
									autoSelectFirst.current = false;
									setSelected(null);
									setOpened(null);
									accessedItem[1].action();
								} else {
									e.preventDefault();
									e.stopPropagation();
									autoSelectFirst.current = true;
									setSelected(accessedItem[1].id);
									setOpened(accessedItem[1].id);
								}
							}
						}
					} else {
						if (e.code === "ArrowLeft") {
							e.stopPropagation();
							setOpened(null);
						}
					}
				},
			}}
		>
			{children}
		</MenuContext.Provider>
	);
}

export type MenuItemContextData = {
	onPointerEnter: (event: React.PointerEvent) => void;
	onFocus: (event: React.FocusEvent) => void;
	onClick: (event: React.MouseEvent) => void;
	onKeyDown: (event: React.KeyboardEvent) => void;
};

export const MenuItemContext = createContext<MenuItemContextData>({
	onPointerEnter() {},
	onFocus() {},
	onClick() {},
	onKeyDown() {},
});

export type MenuItemContainerProps = {
	id: string;
	accessKey: string;
	hasChildren: boolean;
	action?: () => void;
};

export function MenuItemContainer({
	children,
	hasChildren,
	...props
}: PropsWithChildren<MenuItemContainerProps>) {
	const menuItemRef = useRef({});
	const {
		addOrUpdateMenuItem,
		removeMenuItem,
		opened,
		setSelected,
		setOpened,
	} = useContext(MenuContext);
	const { autoSelectFirst } = useContext(MenuGlobalContext);

	// Add and remove MenuItem when mounting/unmounting
	useEffect(() => {
		addOrUpdateMenuItem(menuItemRef, {
			id: props.id,
			accessKey: props.accessKey,
			action: props.action,
		});
		return () => {
			removeMenuItem(menuItemRef);
		};
	}, [addOrUpdateMenuItem, removeMenuItem]);

	// Update MenuItem on props change
	useEffect(() => {
		addOrUpdateMenuItem(menuItemRef, {
			id: props.id,
			accessKey: props.accessKey,
			action: props.action,
		});
	}, [props.id, props.accessKey, props.action]);

	function onClick(e: React.MouseEvent | React.KeyboardEvent) {
		// Not bubbling up
		if (e.target === e.currentTarget) {
			// Has children
			if (hasChildren) {
				e.preventDefault();
				e.stopPropagation();
				setOpened(opened === props.id ? null : props.id);
			}
			// Has action
			else if (props.action) {
				setOpened(null);
				props.action();
			}
		}
	}

	return (
		<MenuItemContext.Provider
			value={{
				onPointerEnter(e) {
					setSelected(props.id);
				},
				onFocus(e) {
					setSelected(props.id);
				},
				onClick,
				onKeyDown(e) {
					if (
						e.code === "Space" ||
						e.code === "Enter" ||
						e.code === "ArrowRight"
					) {
						autoSelectFirst.current = true;
						onClick(e);
					}
				},
			}}
		>
			{children}
		</MenuItemContext.Provider>
	);
}

export type KeyboardMenuContainer = {
	/**
	 * The HTML Element to attach keyboard events to
	 */
	container: HTMLElement;
};

export function KeyboardMenuContainer({
	children,
	container,
}: PropsWithChildren<KeyboardMenuContainer>) {
	const [showAccessKey, setShowAccessKey] = useState<boolean>(false);
	const autoSelectFirst = useRef<boolean>(false);

	useEffect(() => {
		function onKeyDown(e: KeyboardEvent) {
			if (e.code === "AltLeft") {
				e.preventDefault();
				e.stopPropagation();
				setShowAccessKey(!showAccessKey);
				autoSelectFirst.current = !showAccessKey;
			}
		}

		container.addEventListener("keydown", onKeyDown);

		return () => {
			container.removeEventListener("keydown", onKeyDown);
		};
	}, [container, showAccessKey, setShowAccessKey]);

	return (
		<MenuGlobalContext.Provider
			value={{
				showAccessKey,
				setShowAccessKey,
				autoSelectFirst,
			}}
		>
			{children}
		</MenuGlobalContext.Provider>
	);
}
