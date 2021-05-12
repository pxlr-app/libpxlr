import { useDatabase } from './indexedDb';

export default function useLocalFiles<T>() {
	const db = useDatabase('localfiles', {
		version: 1,
		onUpgradeNeeded(e) {
			this.result.createObjectStore('localfiles');
		}
	});

	return {
		get(id: string): Promise<T> {
			let tx = db.transaction("localfiles", "readonly");
			let store = tx.objectStore("localfiles");
			return new Promise<T>((resolve, reject) => {
				let request = store.get(id);
				request.onsuccess = (e) => resolve(request.result as T);
				request.onerror = (e) => reject();
			});
		},
		all(): Promise<T[]> {
			let tx = db.transaction("localfiles", "readonly");
			let store = tx.objectStore("localfiles");
			return new Promise<T[]>((resolve, reject) => {
				let request = store.getAll();
				request.onsuccess = (e) => resolve(request.result as T[]);
				request.onerror = (e) => reject();
			});
		},
		insert(id: string, data: T): Promise<string> {
			let tx = db.transaction("localfiles", "readwrite");
			let store = tx.objectStore("localfiles");
			return new Promise<string>((resolve, reject) => {
				let request = store.add(data, id);
				request.onsuccess = (e) => resolve(id);
				request.onerror = (e) => reject();
			});
		},
		replace(id: string, data: T): Promise<string> {
			let tx = db.transaction("localfiles", "readwrite");
			let store = tx.objectStore("localfiles");
			return new Promise<string>((resolve, reject) => {
				let request = store.put(data, id);
				request.onsuccess = (e) => resolve(id);
				request.onerror = (e) => reject();
			});
		},
		delete(id: string): Promise<void> {
			return Promise.reject();
		}
	}
}