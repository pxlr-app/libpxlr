import React from "react";
import { Story, Meta } from "@storybook/react";
import { Menu, MenuItem, Separator, KeyboardMenu } from "./Menu";

export default {
	title: "Layout/Menu",
	component: Menu,
	argTypes: {
		//   backgroundColor: { control: 'color' },
	},
} as Meta;

const Template = () => (
	<Menu width="300px">
		<MenuItem
			id="newfile"
			label="New File"
			accesskey="N"
			keybind="Ctrl+N"
			action={() => alert("newfile")}
		/>
		<MenuItem
			id="newwindow"
			label="New Window"
			accesskey="W"
			keybind="Ctrl+Shift+N"
			action={() => alert("newwindow")}
		/>
		<Separator />
		<MenuItem
			id="openfile"
			label="Open File…"
			accesskey="O"
			keybind="Ctrl+O"
			action={() => alert("openfile")}
		/>
		<MenuItem
			id="openrecent"
			label="Open Recent"
			accesskey="R"
			keybind="Ctrl+Shift+O"
		>
			<Menu width="300px">
				<MenuItem
					id="reopen"
					label="Reopen Closed File"
					accesskey="R"
					keybind="Ctrl+Shift+T"
					action={() => alert("reopen")}
				/>
				<MenuItem id="recentfiles" label="Recent Files" accesskey="F">
					<Menu width="300px">
						<MenuItem
							id="filea"
							label="File A"
							accesskey="A"
							action={() => alert("filea")}
						/>
						<MenuItem
							id="fileb"
							label="File B"
							accesskey="B"
							action={() => alert("fileb")}
						/>
						<MenuItem
							id="filec"
							label="File C"
							accesskey="C"
							action={() => alert("filec")}
						/>
					</Menu>
				</MenuItem>

				<MenuItem
					id="clearrecent"
					label="Clear Recent Files"
					accesskey="C"
					action={() => alert("clearrecent")}
				/>
			</Menu>
		</MenuItem>
		<Separator />
		<MenuItem
			id="save"
			label="Save"
			accesskey="S"
			keybind="Ctrl+S"
			action={() => alert("save")}
		/>
		<MenuItem
			id="saveas"
			label="Save As…"
			accesskey="A"
			keybind="Ctrl+Shift+S"
			action={() => alert("saveas")}
		/>
		<MenuItem
			id="autosave"
			label="Auto Save"
			accesskey="t"
			checked
			action={() => alert("autosave")}
		/>
		<Separator />
		<MenuItem id="preferences" label="Preferences" accesskey="P">
			<Menu width="300px">
				<MenuItem
					id="settings"
					label="Settings"
					accesskey="S"
					keybind="Ctrl+,"
					action={() => alert("settings")}
				/>
				<MenuItem
					id="keyboardshortcuts"
					label="Keyboard Shortcuts"
					accesskey="K"
					action={() => alert("keyboardshortcuts")}
				/>
			</Menu>
		</MenuItem>
		<MenuItem
			id="useraccount"
			label="User Account"
			accesskey="U"
			action={() => alert("useraccount")}
		/>
	</Menu>
);

export const Pointer: Story<{}> = (args) => <Template />;

export const Keyboard: Story<{}> = (args) => (
	<KeyboardMenu container={document as any}>
		<Template />
	</KeyboardMenu>
);
