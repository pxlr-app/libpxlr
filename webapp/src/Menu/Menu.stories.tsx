import { Menu, MenuItem, Separator } from "./Menu";

export default {
	title: "Components/Menu",
};

export const Uncontrolled = () => (
	<Menu>
		<MenuItem id="newfile" label="New File" accessKey="N" keybind="Ctrl+N" action={() => alert("newfile")} />
		<MenuItem
			id="newwindow"
			label="New Window"
			accessKey="W"
			keybind="Ctrl+Shift+N"
			action={() => alert("newwindow")}
		/>
		<Separator />
		<MenuItem id="openfile" label="Open File…" accessKey="O" keybind="Ctrl+O" action={() => alert("openfile")} />
		<MenuItem id="openrecent" label="Open Recent" accessKey="R" keybind="Ctrl+Shift+O">
			<Menu>
				<MenuItem
					id="reopen"
					label="Reopen Closed File"
					accessKey="R"
					keybind="Ctrl+Shift+T"
					action={() => alert("reopen")}
				/>
				<MenuItem id="recentfiles" label="Recent Files" accessKey="F">
					<Menu>
						<MenuItem id="filea" label="File A" accessKey="A" action={() => alert("filea")} />
						<MenuItem id="fileb" label="File B" accessKey="B" action={() => alert("fileb")} />
						<MenuItem id="filec" label="File C" accessKey="C" action={() => alert("filec")} />
					</Menu>
				</MenuItem>

				<MenuItem
					id="clearrecent"
					label="Clear Recent Files"
					accessKey="C"
					action={() => alert("clearrecent")}
				/>
			</Menu>
		</MenuItem>
		<Separator />
		<MenuItem id="save" label="Save" accessKey="S" keybind="Ctrl+S" action={() => alert("save")} />
		<MenuItem id="saveas" label="Save As…" accessKey="A" keybind="Ctrl+Shift+S" action={() => alert("saveas")} />
		<MenuItem id="autosave" label="Auto Save" accessKey="t" checked action={() => alert("autosave")} />
		<Separator />
		<MenuItem id="preferences" label="Preferences" accessKey="P">
			<Menu>
				<MenuItem
					id="settings"
					label="Settings"
					accessKey="S"
					keybind="Ctrl+,"
					action={() => alert("settings")}
				/>
				<MenuItem
					id="keyboardshortcuts"
					label="Keyboard Shortcuts"
					accessKey="K"
					action={() => alert("keyboardshortcuts")}
				/>
			</Menu>
		</MenuItem>
		<MenuItem id="useraccount" label="User Account" accessKey="U" action={() => alert("useraccount")} />
	</Menu>
);
