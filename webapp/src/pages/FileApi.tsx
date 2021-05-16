import React, { useEffect, useState } from "react";
import { useDatabase } from "../hooks/indexedDb";

export default function FileAPI() {
	const db = useDatabase("test", {
		version: 1,
		onUpgradeNeeded: function () {
			const store = this.result.createObjectStore("handles");
			store.createIndex("handles_mtime", "mtime");
		},
	});
	const [dirty, setDirty] = useState(0);
	// https://web.dev/file-system-access/#storing-file-handles-or-directory-handles-in-indexeddb
	const [items, setItems] = useState<any[]>([]);
	useEffect(() => {
		const tx = db.transaction("handles", "readonly");
		const store = tx.objectStore("handles");
		const request = store.getAll();
		request.onsuccess = (e) => {
			setItems(request.result);
		};
	}, [db, dirty]);
	useEffect(() => {
		function onDragOver(e: DragEvent) {
			e.preventDefault();
		}
		async function onDrop(e: DragEvent) {
			e.stopPropagation();
			e.preventDefault();
			for (const item of e.dataTransfer?.items ?? []) {
				const handle = await (item as any).getAsFileSystemHandle();
				const tx = db.transaction("handles", "readwrite");
				const store = tx.objectStore("handles");
				await new Promise<void>((resolve, reject) => {
					const data = {
						name: handle.name,
						handle,
						mtime: new Date().getTime(),
					};
					const request = store.put(data, handle.name);
					request.onsuccess = (e) => resolve();
					request.onerror = (e) => reject();
				});
			}
			setDirty((dirty) => dirty + 1);
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
			{items.map((item) => {
				return (
					<li key={item.name}>
						<a
							onClick={async (e) => {
								e.preventDefault();
								const perm = await item.handle.requestPermission(
									{
										mode: "read",
									},
								);
								console.log("Permission:", perm);
								const file: File = await item.handle.getFile();
								// const blob = file.slice(0, 10);
								// const buffer = await new Promise<ArrayBuffer>(
								// 	(resolve, reject) => {
								// 		const reader = new FileReader();
								// 		reader.onload = (e) =>
								// 			resolve(
								// 				e.target!.result as ArrayBuffer,
								// 			);
								// 		reader.onerror = (e) => reject();
								// 		reader.readAsArrayBuffer(blob);
								// 	},
								// );
								const buffer = await file.arrayBuffer();
								const decoder = new TextDecoder();
								const text = decoder.decode(buffer);
								console.log("Content:", text);
								console.log("Done");
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
