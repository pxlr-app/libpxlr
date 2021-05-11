import { useContext, useEffect, useState } from 'react';
import type { FirebaseApp } from 'firebase/app';
import { getAuth, useAuthEmulator, onAuthStateChanged } from 'firebase/auth';
import type { User, Auth } from 'firebase/auth';
import FirebaseAppContext from '../contexts/firebase';
import suspend from '../utils/suspend';

// Cache auth for each firebase app
const cacheFirebaseAppAuth = new WeakMap<FirebaseApp, () => Auth>();

/**
 * Returns Firebase's auth from context
 */
export function useAuth() {
	const app = useContext(FirebaseAppContext);
	if (app) {
		if (!cacheFirebaseAppAuth.has(app)) {
			const auth = getAuth(app);
			// Configure emulator for auth
			if (
				(auth as any)._canInitEmulator &&
				import.meta.env.VITE_FIREBASE_EMULATED
			) {
				useAuthEmulator(auth, 'http://localhost:9099');
			}
			// Create suspender
			cacheFirebaseAppAuth.set(app, suspend(new Promise((resolve, reject) => {
				try {
					const unsubscribe = onAuthStateChanged(auth, user => {
						unsubscribe();
						resolve(auth);
					});
				} catch (err) {
					reject(err);
				}
			})));
		}
		let suspender = cacheFirebaseAppAuth.get(app)!;
		// Suspend till first onAuthStateChanged
		return suspender();
	}
}