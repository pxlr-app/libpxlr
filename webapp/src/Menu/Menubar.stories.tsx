import React from "react";
import { Story, Meta } from "@storybook/react";
import { AccessibleMenuContainer } from "./MenuContainer";
import { Menu, MenuItem, Separator } from "./Menu";
import { Menubar, MenubarItem } from "./Menubar";

export default {
	title: "Layout/Menubar",
	component: Menubar,
	argTypes: {
		//   backgroundColor: { control: 'color' },
	},
} as Meta;

const MenuTemplate = () => (
	<Menu width="300px">
		<MenuItem
			key="newfile"
			id="newfile"
			label="New File"
			accessKey="N"
			keybind="Ctrl+N"
			action={() => alert("newfile")}
		/>
		<MenuItem
			key="newwindow"
			id="newwindow"
			label="New Window"
			accessKey="W"
			keybind="Ctrl+Shift+N"
			action={() => alert("newwindow")}
		/>
		<Separator key="sep1" />
		<MenuItem
			key="openfile"
			id="openfile"
			label="Open File…"
			accessKey="O"
			keybind="Ctrl+O"
			action={() => alert("openfile")}
		/>
		<MenuItem
			key="openrecent"
			id="openrecent"
			label="Open Recent"
			accessKey="R"
			keybind="Ctrl+Shift+O"
		>
			<Menu width="300px">
				<MenuItem
					key="reopen"
					id="reopen"
					label="Reopen Closed File"
					accessKey="R"
					keybind="Ctrl+Shift+T"
					action={() => alert("reopen")}
				/>
				<MenuItem
					key="recentfiles"
					id="recentfiles"
					label="Recent Files"
					accessKey="F"
				>
					<Menu width="300px">
						<MenuItem
							key="filea"
							id="filea"
							label="File A"
							accessKey="A"
							action={() => alert("filea")}
						/>
						<MenuItem
							key="fileb"
							id="fileb"
							label="File B"
							accessKey="B"
							action={() => alert("fileb")}
						/>
						<MenuItem
							key="filec"
							id="filec"
							label="File C"
							accessKey="C"
							action={() => alert("filec")}
						/>
					</Menu>
				</MenuItem>

				<MenuItem
					key="clearrecent"
					id="clearrecent"
					label="Clear Recent Files"
					accessKey="C"
					action={() => alert("clearrecent")}
				/>
			</Menu>
		</MenuItem>
		<Separator />
		<MenuItem
			key="save"
			id="save"
			label="Save"
			accessKey="S"
			keybind="Ctrl+S"
			action={() => alert("save")}
		/>
		<MenuItem
			key="saveas"
			id="saveas"
			label="Save As…"
			accessKey="A"
			keybind="Ctrl+Shift+S"
			action={() => alert("saveas")}
		/>
		<MenuItem
			key="autosave"
			id="autosave"
			label="Auto Save"
			accessKey="t"
			checked
			action={() => alert("autosave")}
		/>
		<Separator key="sep2" />
		<MenuItem id="preferences" label="Preferences" accessKey="P">
			<Menu width="300px">
				<MenuItem
					key="settings"
					id="settings"
					label="Settings"
					accessKey="S"
					keybind="Ctrl+,"
					action={() => alert("settings")}
				/>
				<MenuItem
					key="keyboardshortcuts"
					id="keyboardshortcuts"
					label="Keyboard Shortcuts"
					accessKey="K"
					action={() => alert("keyboardshortcuts")}
				/>
			</Menu>
		</MenuItem>
		<MenuItem
			key="useraccount"
			id="useraccount"
			label="User Account"
			accessKey="U"
			action={() => alert("useraccount")}
		/>
	</Menu>
);

const Template = () => (
	<Menubar>
		<MenubarItem id="file" label="File" accessKey="F">
			<MenuTemplate />
		</MenubarItem>
		<MenubarItem id="edit" label="Edit" accessKey="E" />
		<MenubarItem id="selection" label="Selection" accessKey="S" />
		<MenubarItem id="view" label="View" accessKey="V" />
		<MenubarItem id="help" label="Help" accessKey="H" />
	</Menubar>
);

export const Default: Story<{}> = (args) => <Template />;

export const Accessible: Story<{}> = (args) => (
	<AccessibleMenuContainer container={document as any}>
		<Template />
	</AccessibleMenuContainer>
);
