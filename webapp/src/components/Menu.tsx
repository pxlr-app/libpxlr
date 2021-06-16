import React, {
	PropsWithChildren,
	ReactNode,
	ReactElement,
	createElement,
	createContext,
	useState,
	useContext,
	useCallback,
	Fragment,
} from "react";
import { Spring, animated } from "react-spring";

export const MenuContext = createContext<{
	onSelected?: () => void;
}>({});

function MenuItem(props: {
	as?: string;
	className?: string;
	children?: ReactNode;
}) {
	return createElement(
		props.as ?? "div",
		{ className: props.className },
		props.children,
	);
}

function MenuGroup(props: {
	as?: string;
	className?: string;
	children?: (open: boolean) => ReactElement;
}) {
	const [opened, setOpen] = useState(false);
	const onSelected = () => setOpen(true);

	return createElement(
		props.as ?? "div",
		{ className: props.className, onClick: onSelected },
		props.children && props.children(opened),
	);
}

function Menu(
	props: PropsWithChildren<{
		as?: string;
		className?: string;
	}>,
) {
	return createElement(
		props.as ?? "div",
		{ className: props.className ?? "" },
		props.children,
	);
}

export type ControlledMenuItem = {
	label: string;
	icon?: string;
	checked?: boolean;
	key?: string;
	keybinding?: string;
	onSelected?: () => void;
};

export type ControlledMenuGroup = {
	label: string;
	icon?: string;
	key?: string;
	keybinding?: string;
	children: ControlledMenuProps["items"];
};

export type ControlledMenuSeparator = "-";

export type ControlledMenuItems =
	| ControlledMenuItem
	| ControlledMenuGroup
	| ControlledMenuSeparator;

export type ControlledMenuProps = {
	rootClassName?: string;
	itemClassName?: string;
	groupClassName?: string;
	separatorClassName?: string;
	items: ControlledMenuItems[];
};

export default function ControlledMenu(props: ControlledMenuProps) {
	return (
		<Menu as="ul" className={props.rootClassName}>
			{props.items.map((item, i) => {
				if (item === "-") {
					return <li key={i} className={props.separatorClassName} />;
				} else if ("children" in item) {
					return (
						<MenuGroup
							key={i}
							as="li"
							className={props.itemClassName}
						>
							{(opened) => (
								<>
									<div>{item.label}</div>
									{opened && (
										<Spring
											from={{ opacity: `0` }}
											to={{ opacity: `1` }}
										>
											{(spring) => (
												<animated.div style={spring}>
													<ControlledMenu
														{...props}
														rootClassName={
															props.groupClassName
														}
														items={item.children}
													/>
												</animated.div>
											)}
										</Spring>
									)}
								</>
							)}
						</MenuGroup>
					);
				} else {
					return (
						<MenuItem
							key={i}
							as="li"
							className={props.itemClassName}
						>
							<div>{item.label}</div>
						</MenuItem>
					);
				}
			})}
		</Menu>
	);
}
