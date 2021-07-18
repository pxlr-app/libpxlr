import {
	Accessor,
	batch,
	children,
	Component,
	createContext,
	createEffect,
	createMemo,
	createSignal,
	JSX,
	onCleanup,
	onMount,
	PropsWithChildren,
	useContext,
} from "solid-js";

export type Setter<T> = (v: T | ((prev: T) => T)) => T;

export type Orientation = "horizontal" | "vertical";

export type NavigationDevice = "pointer" | "keyboard";

export const AccessibleMenuContext = createContext<
	| {
			showAccessKey: Accessor<boolean>;
			setShowAccessKey: Setter<boolean>;
			navigationInput: Accessor<NavigationDevice>;
			setNavigationInput: Setter<NavigationDevice>;
	  }
	| undefined
>(undefined);

export const MenuContext = createContext<
	| {
			selected: Accessor<string | undefined>;
			opened: Accessor<string | undefined>;
			select: Setter<string | undefined>;
			open: Setter<string | undefined>;
			orientation: Orientation;
	  }
	| undefined
>(undefined);

export type UnstyledMenuChildParams = {
	selected: Accessor<string | undefined>;
	opened: Accessor<string | undefined>;
	select: Setter<string | undefined>;
	open: Setter<string | undefined>;

	props: {
		tabIndex: number;
		onKeyDown: (event: KeyboardEvent) => void;
	};
};

export type UnstyledMenuProps = {
	/**
	 * The HTML Element to attach keyboard events to
	 */
	accessibleContainer?: HTMLElement;

	orientation: Orientation;
	children: (params: UnstyledMenuChildParams) => JSX.Element;
};

export const UnstyledMenu: (props: UnstyledMenuProps) => JSX.Element = (
	props,
) => {
	const accessible =
		useContext(AccessibleMenuContext) ??
		(() => {
			const [showAccessKey, setShowAccessKey] = createSignal(false);
			const [
				navigationInput,
				setNavigationInput,
			] = createSignal<NavigationDevice>("pointer");

			function onKeyDown(e: KeyboardEvent) {
				if (e.code === "AltLeft") {
					e.preventDefault();
					e.stopPropagation();
					batch(() => {
						setShowAccessKey((state) => !state);
						setNavigationInput("keyboard");
					});
				}
			}

			const accessibleContainer =
				props.accessibleContainer ?? document.body;

			console.info("UnstyledMenu::accessInit");

			accessibleContainer.addEventListener("keydown", onKeyDown);
			onCleanup(() => {
				console.info("UnstyledMenu::accessDestroy");
				accessibleContainer.removeEventListener("keydown", onKeyDown);
			});

			return {
				showAccessKey,
				setShowAccessKey,
				navigationInput,
				setNavigationInput,
			};
		})();

	const [selected, select2] = createSignal<string | undefined>(undefined);
	const select = (v: any) => {
		console.trace("Menu select", v);
		return select2(v);
	};
	const [opened, open] = createSignal<string | undefined>(undefined);

	onMount(() => {
		const onLeave = (e: MouseEvent | KeyboardEvent) => {
			batch(() => {
				accessible.setNavigationInput("pointer");
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

	const params: UnstyledMenuChildParams = {
		selected,
		select,
		opened,
		open,
		props: {
			tabIndex: -1,
			onKeyDown(e: KeyboardEvent) {
				if (e.code === "AltLeft") {
					e.preventDefault();
					e.stopPropagation();
					batch(() => {
						accessible.setShowAccessKey((state) => !state);
						accessible.setNavigationInput("keyboard");
					});
				} else if (opened()) {
					console.info("UnstyledMenu::onKeyDown Opened");
				} else if (
					(props.orientation === "vertical" &&
						e.code === "ArrowLeft") ||
					(props.orientation === "horizontal" && e.code === "ArrowUp")
				) {
					e.preventDefault();
					e.stopPropagation();
					batch(() => {
						accessible.setNavigationInput("keyboard");
						open(undefined);
					});
				}
			},
		},
	};

	createEffect(() => {
		console.log("Menu selected", selected());
	});
	createEffect(() => {
		console.log("Menu opened", opened());
	});

	debugger; // props.children madness!

	return (
		<MenuContext.Provider
			value={{
				orientation: props.orientation,
				selected,
				select,
				opened,
				open,
			}}
		>
			<AccessibleMenuContext.Provider value={accessible}>
				{props.children(params)}
			</AccessibleMenuContext.Provider>
		</MenuContext.Provider>
	);
};

export type UnstyledMenuItemChildParams = {
	selected: Accessor<string | undefined>;
	opened: Accessor<string | undefined>;
	select: Setter<string | undefined>;
	open: Setter<string | undefined>;
	orientation: Orientation;
	showAccessKey: Accessor<boolean>;
	setShowAccessKey: Setter<boolean>;
	navigationInput: Accessor<NavigationDevice>;
	setNavigationInput: Setter<NavigationDevice>;

	props: {
		tabIndex: number;
		onPointerEnter: (event: PointerEvent) => void;
		onFocus: (event: FocusEvent) => void;
		onClick: (event: MouseEvent) => void;
		onKeyDown: (event: KeyboardEvent) => void;
	};
};

export type UnstyledMenuItemProps = {
	id: string;
	accessKey: string;
	action?: () => void;
	hasChildren: boolean;
	children: (params: UnstyledMenuItemChildParams) => JSX.Element;
};

export const UnstyledMenuItem: (props: UnstyledMenuItemProps) => JSX.Element = (
	props,
) => {
	const menuContext = useContext(MenuContext);
	if (!menuContext) {
		throw new Error(
			"UnstyledMenuItem needs to be a descendant of UnstyledMenu.",
		);
	}

	const accessibleContext = useContext(AccessibleMenuContext)!;

	const onClick = (e: MouseEvent | KeyboardEvent) => {
		// Not bubbling up
		if (e.target === e.currentTarget) {
			// Has children
			if (props.hasChildren) {
				e.preventDefault();
				e.stopPropagation();
				if (menuContext.opened() === props.id) {
					batch(() => {
						menuContext.select(undefined);
						menuContext.open(undefined);
					});
				} else {
					batch(() => {
						menuContext.select(props.id);
						menuContext.open(props.id);
					});
				}
			}
			// Has action
			else if (props.action) {
				batch(() => {
					menuContext.select(undefined);
					menuContext.open(undefined);
				});
				props.action();
			}
		}
	};

	const params: UnstyledMenuItemChildParams = {
		selected: menuContext.selected,
		select: menuContext.select,
		opened: menuContext.opened,
		open: menuContext.open,
		orientation: menuContext.orientation,
		showAccessKey: accessibleContext.showAccessKey,
		setShowAccessKey: accessibleContext.setShowAccessKey,
		navigationInput: accessibleContext.navigationInput,
		setNavigationInput: accessibleContext.setNavigationInput,
		props: {
			tabIndex: -1,
			onPointerEnter() {
				batch(() => {
					accessibleContext.setNavigationInput("pointer");
					menuContext.select(props.id);
					if (menuContext.opened() !== props.id) {
						menuContext.open(undefined);
					}
				});
			},
			onFocus() {
				batch(() => {
					menuContext.select(props.id);
					if (menuContext.opened() !== props.id) {
						menuContext.open(undefined);
					}
				});
			},
			onClick(e: MouseEvent) {
				accessibleContext.setNavigationInput("pointer");
				onClick(e);
			},
			onKeyDown(e: KeyboardEvent) {
				if (e.code === "AltLeft") {
					e.preventDefault();
					e.stopPropagation();
					batch(() => {
						accessibleContext.setShowAccessKey((state) => !state);
						accessibleContext.setNavigationInput("keyboard");
					});
				} else if (
					e.code === "Space" ||
					e.code === "Enter" ||
					(menuContext.orientation === "vertical" &&
						e.code === "ArrowRight") ||
					(menuContext.orientation === "horizontal" &&
						e.code === "ArrowDown")
				) {
					accessibleContext.setNavigationInput("keyboard");
					onClick(e);
				}
			},
		},
	};

	debugger; // props.children madness!

	return <>{props.children(params)}</>;
};
