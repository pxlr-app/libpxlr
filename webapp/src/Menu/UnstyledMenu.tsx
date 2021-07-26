import {
	Accessor,
	batch,
	createContext,
	createEffect,
	createRenderEffect,
	createSignal,
	JSX,
	onCleanup,
	onMount,
	useContext,
} from "solid-js";
import { Alignement, AnchorContext, VerticalAlign } from "../Anchor";

export type Orientation = "horizontal" | "vertical";

export type NavigationDevice = "pointer" | "keyboard";

export type AutoSelectItem = "first" | "last" | "off";

export type Setter<T> = (v: T | ((prev: T) => T)) => T;

export type MenuItemDeclaration = {
	id: string;
	accessKey: string;
	action?: () => void;
};

export type RootMenuContextData = {
	showAccessKey: Accessor<boolean>;
	setShowAccessKey: Setter<boolean>;
	navigationInput: Accessor<NavigationDevice>;
	setNavigationInput: Setter<NavigationDevice>;
	autoSelectItem: Accessor<boolean>;
	setAutoSelectItem: Setter<boolean>;
};

export type MenuContextData = {
	selected: Accessor<string | undefined>;
	select: Setter<string | undefined>;
	opened: Accessor<string | undefined>;
	open: Setter<string | undefined>;
	items: MenuItemDeclaration[];
	createMenuItem: (path: string[], item: MenuItemDeclaration) => void;
};

const RootMenuContext = createContext<RootMenuContextData | undefined>(undefined);
const MenuContext = createContext<MenuContextData | undefined>(undefined);

const PathContext = createContext<string[]>([]);

export type UnstyledMenuData = {
	showAccessKey: Accessor<boolean>;
	setShowAccessKey: Setter<boolean>;
	navigationInput: Accessor<NavigationDevice>;
	setNavigationInput: Setter<NavigationDevice>;
	selected: Accessor<string | undefined>;
	select: Setter<string | undefined>;
	opened: Accessor<string | undefined>;
	open: Setter<string | undefined>;
	props: {
		tabIndex: number;
		onKeyDown: (e: KeyboardEvent) => void;
	};
};

export type UnstyledMenuProps = {
	/**
	 * The HTML Element to attach keyboard events to
	 */
	accessibilityContainer?: HTMLElement;

	/**
	 * Orientation of this menu
	 */
	orientation: Orientation;

	/**
	 * Render child
	 */
	children: (menu: UnstyledMenuData) => JSX.Element;
};

export const UnstyledMenu = (props: UnstyledMenuProps) => {
	const path = useContext(PathContext);
	const fullpath = ("/" + path.join("/") + "/").replace(/^\/+/, "/");
	const items: MenuItemDeclaration[] = [];
	const [selected, select] = createSignal<string | undefined>(undefined);
	const [opened, open] = createSignal<string | undefined>(undefined);

	const menuContext: MenuContextData = {
		selected,
		select,
		opened,
		open,
		items,
		createMenuItem(path, item) {
			items.push(item);
		},
	};

	const rootContext =
		useContext(RootMenuContext) ??
		(() => {
			const [showAccessKey, setShowAccessKey] = createSignal(false);
			const [navigationInput, setNavigationInput] = createSignal<NavigationDevice>("pointer");
			const [autoSelectItem, setAutoSelectItem] = createSignal(false);

			onMount(() => {
				const onLeave = (e: MouseEvent | KeyboardEvent) => {
					batch(() => {
						rootContext.setNavigationInput("pointer");
						rootContext.setShowAccessKey(false);
						rootContext.setAutoSelectItem(false);
					});
				};

				document.addEventListener("click", onLeave);
				document.addEventListener("keydown", onLeave);

				onCleanup(() => {
					document.removeEventListener("click", onLeave);
					document.removeEventListener("keydown", onLeave);
				});
			});

			const data: RootMenuContextData = {
				showAccessKey,
				setShowAccessKey,
				navigationInput,
				setNavigationInput,
				autoSelectItem,
				setAutoSelectItem,
			};

			return data;
		})();

	const anchorContext = useContext(AnchorContext);

	onMount(() => {
		const onLeave = (e: MouseEvent | KeyboardEvent) => {
			batch(() => {
				menuContext.select(undefined);
				menuContext.open(undefined);
			});
		};

		document.addEventListener("click", onLeave);
		document.addEventListener("keydown", onLeave);

		onCleanup(() => {
			document.removeEventListener("click", onLeave);
			document.removeEventListener("keydown", onLeave);
		});
	});

	createEffect(() => {
		if (rootContext.navigationInput() === "keyboard" && rootContext.autoSelectItem() && !menuContext.opened()) {
			const verticalAlign = anchorContext()?.transform?.[1] ?? VerticalAlign.TOP;
			if (verticalAlign === VerticalAlign.BOTTOM) {
				menuContext.select(items[items.length - 1].id);
			} else {
				menuContext.select(items[0].id);
			}
		}
	});

	const data: UnstyledMenuData = {
		showAccessKey: rootContext.showAccessKey,
		setShowAccessKey: rootContext.setShowAccessKey,
		navigationInput: rootContext.navigationInput,
		setNavigationInput: rootContext.setNavigationInput,
		selected: menuContext.selected,
		select: menuContext.select,
		opened: menuContext.opened,
		open: menuContext.open,

		props: {
			tabIndex: 0,
			onKeyDown(e: KeyboardEvent) {
				if (e.code === "AltLeft") {
					e.preventDefault();
					e.stopImmediatePropagation();
					batch(() => {
						rootContext.setShowAccessKey((state) => !state);
						rootContext.setNavigationInput("keyboard");
					});
				} else if (!menuContext.opened()) {
					const selected = menuContext.selected();
					let selectedIdx = items.findIndex((p) => p.id === selected);

					// Down
					if (
						(props.orientation === "vertical" && e.code === "ArrowDown") ||
						(props.orientation === "horizontal" && e.code === "ArrowRight")
					) {
						e.preventDefault();
						e.stopImmediatePropagation();
						selectedIdx = (selectedIdx + 1) % items.length;
						batch(() => {
							rootContext.setNavigationInput("keyboard");
							rootContext.setAutoSelectItem(false);
							menuContext.select(items[selectedIdx].id);
						});
					}
					// Up
					else if (
						(props.orientation === "vertical" && e.code === "ArrowUp") ||
						(props.orientation === "horizontal" && e.code === "ArrowLeft")
					) {
						e.preventDefault();
						e.stopImmediatePropagation();
						if (selectedIdx === -1) {
							selectedIdx = items.length - 1;
						} else {
							selectedIdx = (items.length + (selectedIdx - 1)) % items.length;
						}
						batch(() => {
							rootContext.setNavigationInput("keyboard");
							rootContext.setAutoSelectItem(false);
							menuContext.select(items[selectedIdx].id);
						});
					}
					// Forward
					else if (
						(props.orientation === "vertical" && e.code === "ArrowRight") ||
						(props.orientation === "horizontal" && e.code === "ArrowDown")
					) {
						const selectedItem = items[selectedIdx];
						if (selectedItem.action) {
							selectedItem.action();
						} else {
							e.preventDefault();
							e.stopImmediatePropagation();
							batch(() => {
								rootContext.setNavigationInput("keyboard");
								rootContext.setAutoSelectItem(true);
								menuContext.open(selectedItem.id);
							});
						}
					}
					// AccessKey
					else if (rootContext.showAccessKey()) {
						const accessedItem = items.find(
							({ id, accessKey }) => `Key${accessKey.toUpperCase()}` === e.code,
						);
						if (accessedItem) {
							if (accessedItem.action) {
								if (document.activeElement) {
									(document.activeElement as HTMLElement).blur();
								}
								accessedItem.action();
							} else {
								e.preventDefault();
								e.stopImmediatePropagation();
								batch(() => {
									rootContext.setNavigationInput("keyboard");
									rootContext.setAutoSelectItem(true);
									menuContext.select(accessedItem.id);
									menuContext.open(accessedItem.id);
								});
							}
						}
					}
				}
				// Back
				else if (
					(props.orientation === "vertical" && e.code === "ArrowLeft") ||
					(props.orientation === "horizontal" && e.code === "ArrowUp")
				) {
					if (menuContext.opened()) {
						e.preventDefault();
						e.stopImmediatePropagation();
						batch(() => {
							rootContext.setNavigationInput("keyboard");
							rootContext.setAutoSelectItem(false);
							menuContext.select(selected);
							menuContext.open(undefined);
						});
					}
				}
			},
		},
	};

	return (
		<RootMenuContext.Provider value={rootContext}>
			<MenuContext.Provider value={menuContext}>{props.children(data)}</MenuContext.Provider>
		</RootMenuContext.Provider>
	);
};

export type UnstyledMenuItemData = {
	showAccessKey: Accessor<boolean>;
	navigationInput: Accessor<NavigationDevice>;
	selected: Accessor<boolean>;
	opened: Accessor<boolean>;

	props: {
		tabIndex: number;
		ref: (e: HTMLElement) => void;
		onMouseEnter: (e: MouseEvent) => void;
		onClick: (e: MouseEvent) => void;
	};
};

export type UnstyledMenuItemProps = {
	/**
	 * A unique identifier for this menu item
	 */
	id: string;
	/**
	 * The access key used for accessibility navigation
	 */
	accessKey: string;
	/**
	 * The action to execute when clicking on the menu item
	 */
	action?: () => void;

	/**
	 * Render child
	 */
	children: (item: UnstyledMenuItemData) => JSX.Element;
};

export const UnstyledMenuItem = (props: UnstyledMenuItemProps) => {
	const path = useContext(PathContext).concat([props.id]);
	const fullpath = "/" + path.join("/") + "/";

	const rootContext = useContext(RootMenuContext);
	if (!rootContext) {
		throw new Error(
			"UnstyledMenuItem requires a RootMenuContext somewhere in the DOM. Did you forget to wrap your component in RootMenuContext?",
		);
	}

	const menuContext = useContext(MenuContext);
	if (!menuContext) {
		throw new Error(
			"UnstyledMenuItem requires a MenuContext somewhere in the DOM. Did you forget to wrap your component in MenuContext?",
		);
	}

	menuContext.createMenuItem(path, { id: props.id, accessKey: props.accessKey, action: props.action });

	const data: UnstyledMenuItemData = {
		showAccessKey: rootContext.showAccessKey,
		navigationInput: rootContext.navigationInput,
		selected() {
			return menuContext.selected() === props.id;
		},
		opened() {
			return menuContext.opened() === props.id;
		},
		props: {
			tabIndex: -1,
			ref(elem) {
				createEffect(() => {
					const _ = menuContext.opened(); // track change on opened
					if (menuContext.selected() === props.id) {
						elem.focus();
						setTimeout(() => elem.focus(), 0); // Wait till element is visible?
					} else if (elem === document.activeElement) {
						elem.blur();
					}
				});
			},
			onMouseEnter(e) {
				batch(() => {
					rootContext.setNavigationInput("pointer");
					menuContext.select(props.id);
					if (menuContext.selected() !== props.id) {
						menuContext.open(undefined);
					}
				});
			},
			onClick(e) {
				if (e.target === e.currentTarget) {
					if (props.action) {
						props.action();
					} else {
						e.preventDefault();
						e.stopImmediatePropagation();
						batch(() => {
							rootContext.setNavigationInput("pointer");
							menuContext.open(props.id);
						});
					}
				}
			},
		},
	};

	return <PathContext.Provider value={path}>{props.children(data)}</PathContext.Provider>;
};
