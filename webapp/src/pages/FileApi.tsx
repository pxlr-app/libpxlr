import React, { useEffect, useState } from "react";
import { useDatabase } from "../hooks/indexedDb";

export default function FileAPI() {
	let db = useDatabase("test", {
		version: 1,
		onUpgradeNeeded: function () {
			this.result.createObjectStore("localfiles");
		},
	});
	// https://web.dev/file-system-access/#storing-file-handles-or-directory-handles-in-indexeddb
	let [files, setFiles] = useState<any[]>([]);
	useEffect(() => {
		let tx = db.transaction("localfiles", "readonly");
		let store = tx.objectStore("localfiles");
		let request = store.getAll();
		request.onsuccess = (e) => {
			setFiles(request.result);
		};
	}, [db]);
	useEffect(() => {
		function onDragOver(e: DragEvent) {
			e.preventDefault();
		}
		async function onDrop(e: DragEvent) {
			e.stopPropagation();
			e.preventDefault();
			for (let item of e.dataTransfer?.items ?? []) {
				let handle = await (item as any).getAsFileSystemHandle();
				let tx = db.transaction("localfiles", "readwrite");
				let store = tx.objectStore("localfiles");
				await new Promise<void>((resolve, reject) => {
					let request = store.put(handle, handle.name);
					request.onsuccess = (e) => resolve();
					request.onerror = (e) => reject();
				});
			}
		}

		document.addEventListener("dragover", onDragOver);
		document.addEventListener("drop", onDrop);

		return () => {
			document.removeEventListener("dragover", onDragOver);
			document.removeEventListener("drop", onDrop);
		};
	});
	return (
		<ul>
			{files.map((item) => {
				console.debug(item);
				return (
					<li key={item.name}>
						<a
							onClick={async (e) => {
								e.preventDefault();
								await item.requestPermission({
									mode: "readwrite",
								});
								let file: File = await item.getFile();
								let blob = file.slice(0, 10);
								let reader = new FileReader();
								let buffer = await new Promise<ArrayBuffer>(
									(resolve, reject) => {
										reader.onload = (e) =>
											resolve(
												e.target!.result as ArrayBuffer,
											);
										reader.onerror = (e) => reject();
										reader.readAsArrayBuffer(blob);
									},
								);
								let decoder = new TextDecoder();
								let text = decoder.decode(buffer);
								console.log("Content:", text);
							}}
						>
							{item.name}
						</a>
					</li>
				);
			})}
		</ul>
	);
}
