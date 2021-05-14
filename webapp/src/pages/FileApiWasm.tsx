import React, { useEffect, useState } from "react";
import { useDatabase } from "../hooks/indexedDb";
import init, { pxlr_hello_world, pxlr_print_file } from "libpxlr";

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
	useEffect(() => {
		init().then(() => {
			console.log(pxlr_hello_world("Blep"));
		});
	});
	return (
		<ul>
			{items.map((item) => {
				return (
					<li key={item.name}>
						<a
							onClick={async (e) => {
								e.preventDefault();
								console.log("Printing content");
								await pxlr_print_file(item.handle);
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
