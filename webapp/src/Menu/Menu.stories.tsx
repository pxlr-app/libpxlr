import { Menu, MenuItem, Separator } from "./Menu";

export default {
	title: "Components/Menu",
};

export const Default = () => (
	<Menu>
		<MenuItem
			id="newfile"
			label="New File"
			accessKey="N"
			keybind="Ctrl+N"
			action={() => console.log("click newfile")}
		/>
		<MenuItem
			id="newwindow"
			label="New Window"
			accessKey="W"
			keybind="Ctrl+Shift+N"
			action={() => console.log("click newwindow")}
		/>
		<Separator />
		<MenuItem
			id="openfile"
			label="Open File…"
			accessKey="O"
			keybind="Ctrl+O"
			action={() => console.log("click openfile")}
		/>
		<MenuItem id="openrecent" label="Open Recent" accessKey="R" keybind="Ctrl+Shift+O">
			<Menu>
				<MenuItem
					id="reopen"
					label="Reopen Closed File"
					accessKey="R"
					keybind="Ctrl+Shift+T"
					action={() => console.log("click reopen")}
				/>
				<MenuItem id="recentfiles" label="Recent Files" accessKey="F">
					<Menu>
						<MenuItem id="filea" label="File A" accessKey="A" action={() => console.log("click filea")} />
						<MenuItem id="fileb" label="File B" accessKey="B" action={() => console.log("click fileb")} />
						<MenuItem id="filec" label="File C" accessKey="C" action={() => console.log("click filec")} />
					</Menu>
				</MenuItem>

				<MenuItem
					id="clearrecent"
					label="Clear Recent Files"
					accessKey="C"
					action={() => console.log("click clearrecent")}
				/>
			</Menu>
		</MenuItem>
		<Separator />
		<MenuItem id="save" label="Save" accessKey="S" keybind="Ctrl+S" action={() => console.log("click save")} />
		<MenuItem
			id="saveas"
			label="Save As…"
			accessKey="A"
			keybind="Ctrl+Shift+S"
			action={() => console.log("click saveas")}
		/>
		<MenuItem id="autosave" label="Auto Save" accessKey="t" checked action={() => console.log("click autosave")} />
		<Separator />
		<MenuItem id="preferences" label="Preferences" accessKey="P">
			<Menu>
				<MenuItem
					id="settings"
					label="Settings"
					accessKey="S"
					keybind="Ctrl+,"
					action={() => console.log("click settings")}
				/>
				<MenuItem
					id="keyboardshortcuts"
					label="Keyboard Shortcuts"
					accessKey="K"
					action={() => console.log("click keyboardshortcuts")}
				/>
			</Menu>
		</MenuItem>
		<MenuItem id="useraccount" label="User Account" accessKey="U" action={() => console.log("click useraccount")} />
	</Menu>
);
