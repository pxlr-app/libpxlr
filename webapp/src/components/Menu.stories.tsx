import React from "react";
import { Story, Meta } from "@storybook/react";
import { Menu, Item, Separator, ControlledMenu } from "./Menu";

export default {
	title: "Layout/Menu",
	component: Menu,
	argTypes: {
		//   backgroundColor: { control: 'color' },
	},
} as Meta;

const Template = () => (
	<Menu width="300px">
		<Item
			id="newfile"
			label="New File"
			accesskey="N"
			keybind="Ctrl+N"
			action={() => console.log("newfile")}
		/>
		<Item
			id="newwindow"
			label="New Window"
			accesskey="W"
			keybind="Ctrl+Shift+N"
			action={() => console.log("newwindow")}
		/>
		<Separator />
		<Item
			id="openfile"
			label="Open File…"
			accesskey="O"
			keybind="Ctrl+O"
			action={() => console.log("openfile")}
		/>
		<Item
			id="openrecent"
			label="Open Recent"
			accesskey="R"
			keybind="Ctrl+Shift+O"
			action={() => console.log("openrecent")}
		/>
		<Separator />
		<Item
			id="save"
			label="Save"
			accesskey="S"
			keybind="Ctrl+S"
			action={() => console.log("save")}
		/>
		<Item
			id="saveas"
			label="Save As…"
			accesskey="A"
			keybind="Ctrl+Shift+S"
			action={() => console.log("saveas")}
		/>
		<Item
			id="autosave"
			label="Auto Save"
			accesskey="t"
			checked
			action={() => console.log("autosave")}
		/>
		<Separator />
		<Item id="preferences" label="Preferences" accesskey="P">
			<Menu width="300px">
				<Item
					id="settings"
					label="Settings"
					accesskey="S"
					keybind="Ctrl+,"
					action={() => console.log("settings")}
				/>
				<Item
					id="keyboardshortcuts"
					label="Keyboard Shortcuts"
					accesskey="K"
					action={() => console.log("keyboardshortcuts")}
				/>
			</Menu>
		</Item>
		<Item
			id="useraccount"
			label="User Account"
			accesskey="U"
			action={() => console.log("useraccount")}
		/>
	</Menu>
);

export const Uncontrolled: Story<{}> = (args) => <Template />;

export const Controlled: Story<{}> = (args) => (
	<ControlledMenu containerRef={document as any}>
		<Template />
	</ControlledMenu>
);
