import suspend from '../utils/suspend';

const cacheIndexedDb = new Map<string, () => IDBDatabase>();

/**
 * Retrieve an IDBDatabase
 */
export function useDatabase(name: string, options?: { version?: number, onUpgradeNeeded?: (this: IDBOpenDBRequest, ev: IDBVersionChangeEvent) => any }): IDBDatabase {
	const key = `${name}:${options?.version ?? 0}`;
	if (!cacheIndexedDb.has(key)) {
		cacheIndexedDb.set(key, suspend(new Promise<IDBDatabase>((resolve, reject) => {
			let request = indexedDB.open(name, options?.version ?? 0);
			request.onerror = e => {
				reject(e);
			};
			request.onupgradeneeded = options?.onUpgradeNeeded ?? null;
			request.onsuccess = e => {
				resolve(request.result);
			};
		})))
	}
	const suspender = cacheIndexedDb.get(key)!;
	// Suspend till first onAuthStateChanged
	return suspender();
}