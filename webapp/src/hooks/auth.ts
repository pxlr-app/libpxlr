import { useContext, useEffect, useState } from 'react';
import type { FirebaseApp } from 'firebase/app';
import { getAuth, useAuthEmulator, onAuthStateChanged } from 'firebase/auth';
import type { User, Auth } from 'firebase/auth';
import FirebaseAppContext from '../contexts/firebase';

// Cache auth for each firebase app
const cacheFirebaseAppAuth = new WeakMap<FirebaseApp, Auth>();

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
			cacheFirebaseAppAuth.set(app, auth);
		}
		return cacheFirebaseAppAuth.get(app);
	}
}

/**
 * Retrive current user
 */
export function useCurrentUser() {
	const auth = useAuth();
	const [user, setUser] = useState<User | undefined>(
		auth?.currentUser ?? undefined,
	);
	useEffect(() => {
		if (auth) {
			const dispose = onAuthStateChanged(auth, u =>
				setUser(u ?? undefined),
			);
			return () => dispose();
		}
	}, [auth]);
	return user;
}