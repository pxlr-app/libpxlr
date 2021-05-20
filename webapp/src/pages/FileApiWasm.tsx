import React, { useEffect, useState } from "react";
import { useDatabase } from "../hooks/indexedDb";
import init, {
	pxlr_editor_create,
	pxlr_editor_destroy,
	pxlr_file_reader_create,
	pxlr_file_reader_close,
	pxlr_document_read,
	pxlr_editor_document_set,
} from "libpxlr";

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
			console.log("Init");
		});
	}, []);
	async function test_api(handle: any) {
		// Create editor
		const editor = pxlr_editor_create();
		// Create file reader
		const reader = await pxlr_file_reader_create(editor, handle);
		// Parse document
		const document = await pxlr_document_read(reader);
		// Assign document to editor
		pxlr_editor_document_set(editor, document);

		// Close reader
		pxlr_file_reader_close(reader);
		// Free editor
		pxlr_editor_destroy(editor);
	}
	return (
		<div>
			<ul>
				{items.map((item) => {
					return (
						<li key={item.name}>
							{item.name}{" "}
							<a
								onClick={(e) => {
									e.preventDefault();
									test_api(item.handle);
								}}
							>
								Print
							</a>
						</li>
					);
				})}
			</ul>
		</div>
	);
}
