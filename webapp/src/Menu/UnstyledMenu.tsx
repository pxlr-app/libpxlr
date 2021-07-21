import {
	Accessor,
	batch,
	createContext,
	createEffect,
	createSignal,
	JSX,
	onCleanup,
	onMount,
	useContext,
} from "solid-js";
import { AnchorContext } from "../Anchor";

export type Orientation = "horizontal" | "vertical";

export type NavigationDevice = "pointer" | "keyboard";

export type Setter<T> = (v: T | ((prev: T) => T)) => T;

export type MenuItemDeclaration = {
	id: string;
	accessKey: string;
	action?: () => void;
};

export type MenuContextData = {
	showAccessKey: Accessor<boolean>;
	setShowAccessKey: Setter<boolean>;
	navigationInput: Accessor<NavigationDevice>;
	setNavigationInput: Setter<NavigationDevice>;
	selected: Accessor<string | undefined>;
	select: Setter<string | undefined>;
	opened: Accessor<string | undefined>;
	open: Setter<string | undefined>;
	// tree: Accessor<Map<string, MenuItemDeclaration>>;
	createMenuItem: (path: string[], item: MenuItemDeclaration) => void;
};

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
	const isRoot = !useContext(MenuContext);
	const menu: Map<string, MenuItemDeclaration> = new Map();

	const context =
		useContext(MenuContext) ??
		(() => {
			const [showAccessKey, setShowAccessKey] = createSignal(false);
			const [navigationInput, setNavigationInput] = createSignal<NavigationDevice>("pointer");
			const [selected, select] = createSignal<string | undefined>(undefined);
			const [opened, open] = createSignal<string | undefined>(undefined);

			onMount(() => {
				const onKeyDown = (e: KeyboardEvent) => {
					if (e.code === "AltLeft") {
						e.preventDefault();
						e.stopImmediatePropagation();
						batch(() => {
							setShowAccessKey((state) => !state);
							setNavigationInput("keyboard");
						});
					}
				};

				const a11yContainer = props.accessibilityContainer ?? document.body;
				a11yContainer.addEventListener("keydown", onKeyDown);

				onCleanup(() => {
					a11yContainer.removeEventListener("keydown", onKeyDown);
				});
			});

			onMount(() => {
				const onLeave = (e: MouseEvent | KeyboardEvent) => {
					batch(() => {
						setNavigationInput("pointer");
						select(undefined);
						open(undefined);
					});
				};

				document.addEventListener("click", onLeave);
				document.addEventListener("keydown", onLeave);

				onCleanup(() => {
					document.removeEventListener("click", onLeave);
					document.removeEventListener("keydown", onLeave);
				});
			});

			const data: MenuContextData = {
				showAccessKey,
				setShowAccessKey,
				navigationInput,
				setNavigationInput,
				selected,
				select,
				opened,
				open,
				createMenuItem(path, item) {
					menu.set("/" + path.join("/") + "/", item);
				},
			};

			return data;
		})();

	const data: UnstyledMenuData = {
		showAccessKey: context.showAccessKey,
		setShowAccessKey: context.setShowAccessKey,
		navigationInput: context.navigationInput,
		setNavigationInput: context.setNavigationInput,
		selected: context.selected,
		select: context.select,
		opened: context.opened,
		open: context.open,

		props: {
			tabIndex: 0,
			onKeyDown(e: KeyboardEvent) {
				if (isRoot) {
					const opened = context.opened() ?? "/";
					const selected = context.selected() ?? "/";
					const items = Array.from(menu.keys())
						.filter((k) => k.substr(0, opened.length) === opened)
						.filter((k) => k.substr(opened.length).split("/").length === 2);
					let selectedIdx = items.findIndex((k) => k === selected);

					// Down
					if (
						(props.orientation === "vertical" && e.code === "ArrowDown") ||
						(props.orientation === "horizontal" && e.code === "ArrowRight")
					) {
						e.preventDefault();
						e.stopImmediatePropagation();
						selectedIdx = (selectedIdx + 1) % items.length;
						batch(() => {
							context.setNavigationInput("keyboard");
							context.select(items[selectedIdx]);
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
							context.setNavigationInput("keyboard");
							context.select(items[selectedIdx]);
						});
					}
					// Forward
					else if (
						(props.orientation === "vertical" && e.code === "ArrowRight") ||
						(props.orientation === "horizontal" && e.code === "ArrowDown")
					) {
						const nextItems = Array.from(menu.keys()).filter(
							(k) => k.substr(0, selected.length) === selected && k !== selected,
						);
						if (nextItems.length) {
							e.preventDefault();
							e.stopImmediatePropagation();
							batch(() => {
								context.setNavigationInput("keyboard");
								context.open(selected);
								context.select(nextItems[0]);
							});
						} else {
							const selectedItem = menu.get(selected);
							if (selectedItem?.action) {
								selectedItem.action();
							}
						}
					}
					// Back
					else if (
						(props.orientation === "vertical" && e.code === "ArrowLeft") ||
						(props.orientation === "horizontal" && e.code === "ArrowUp")
					) {
						e.preventDefault();
						e.stopImmediatePropagation();
						batch(() => {
							context.setNavigationInput("keyboard");
							const prevOpened = opened.split("/").slice(0, -2);
							const prevSelected = selected.split("/").slice(0, -2);
							context.select(prevSelected.length ? prevSelected.join("/") + "/" : undefined);
							context.open(prevOpened.length ? prevOpened.join("/") + "/" : undefined);
						});
					}
				}
			},
		},
	};

	return <MenuContext.Provider value={context}>{props.children(data)}</MenuContext.Provider>;
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
	const context = useContext(MenuContext);
	const anchor = useContext(AnchorContext);

	if (!context) {
		throw new Error(
			"UnstyledMenuItem requires a MenuContext somewhere in the DOM. Did you forget to wrap your component in MenuContext?",
		);
	}

	context.createMenuItem(path, { id: props.id, accessKey: props.accessKey, action: props.action });

	const data: UnstyledMenuItemData = {
		showAccessKey: context.showAccessKey,
		navigationInput: context.navigationInput,
		selected() {
			const selected = context.selected();
			return !!selected && selected.substr(0, fullpath.length) === fullpath;
		},
		opened() {
			const opened = context.opened();
			return !!opened && opened.substr(0, fullpath.length) === fullpath;
		},
		props: {
			tabIndex: -1,
			ref(elem) {
				createEffect(() => {
					if (context.selected() === fullpath) {
						elem.focus();
						setTimeout(() => elem.focus(), 0); // Wait till element is visible?
					} else if (elem === document.activeElement) {
						elem.blur();
					}
				});
			},
			onMouseEnter(e) {
				batch(() => {
					context.setNavigationInput("pointer");
					context.select(fullpath);
					if (context.selected() !== fullpath) {
						context.open(fullpath.split("/").slice(0, -2).join("/") + "/");
					}
				});
			},
			onClick(e) {
				if (props.action) {
					props.action();
				} else {
					batch(() => {
						e.preventDefault();
						e.stopImmediatePropagation();
						context.setNavigationInput("pointer");
						context.open(fullpath);
					});
				}
			},
		},
	};

	return <PathContext.Provider value={path}>{props.children(data)}</PathContext.Provider>;
};
