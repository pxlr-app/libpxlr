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
	const context = useMemo<MenuContextData>(
		() => ({
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
				if (e.code === "AltLeft") {
					console.log("AltLeft Menu");
					e.preventDefault();
					e.stopPropagation();
					setShowAccessKey(!showAccessKey);
				} else if (opened === null) {
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
								`Key${item.accessKey.toUpperCase()}` === e.code,
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
		}),
		[
			selected,
			setSelected,
			opened,
			setOpened,
			showAccessKey,
			setShowAccessKey,
			// autoSelectFirst,
			// items,
		],
	);

	useEffect(() => {
		if (autoSelectFirst.current) {
			autoSelectFirst.current = false;
			if (selected === null) {
				const entries = Array.from(items.current.entries());
				setSelected(entries[0][1].id);
			}
		}
	}, [autoSelectFirst, selected, setSelected]);

	useEffect(() => {
		function onLeave() {
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

	console.log(selected, opened);

	return (
		<MenuContext.Provider value={context}>{children}</MenuContext.Provider>
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
	const { autoSelectFirst, setShowAccessKey, showAccessKey } = useContext(
		MenuGlobalContext,
	);

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

	const onClick = useMemo(
		() => (e: React.MouseEvent | React.KeyboardEvent) => {
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
		},
		[hasChildren, opened, setOpened, props.id, props.action],
	);

	const context = useMemo<MenuItemContextData>(
		() => ({
			onPointerEnter(e) {
				setSelected(props.id);
			},
			onFocus(e) {
				setSelected(props.id);
			},
			onClick,
			onKeyDown(e) {
				if (e.code === "AltLeft") {
					console.log("AltLeft Item");
					e.preventDefault();
					e.stopPropagation();
					setShowAccessKey(!showAccessKey);
				} else if (
					e.code === "Space" ||
					e.code === "Enter" ||
					e.code === "ArrowRight"
				) {
					autoSelectFirst.current = true;
					onClick(e);
				}
			},
		}),
		[
			props.id,
			setSelected,
			showAccessKey,
			setShowAccessKey,
			// autoSelectFirst,
		],
	);

	return (
		<MenuItemContext.Provider value={context}>
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
	const context = useMemo(
		() => ({
			showAccessKey,
			// setShowAccessKey,
			setShowAccessKey: () => {},
			autoSelectFirst,
		}),
		[
			showAccessKey,
			setShowAccessKey,
			// autoSelectFirst
		],
	);

	useEffect(() => {
		function onKeyDown(e: KeyboardEvent) {
			if (e.code === "AltLeft") {
				console.log("AltLeft KB");
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
		<MenuGlobalContext.Provider value={context}>
			{children}
		</MenuGlobalContext.Provider>
	);
}
