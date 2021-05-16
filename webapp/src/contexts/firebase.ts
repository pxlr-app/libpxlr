import type { FirebaseApp } from 'firebase/app';
import { createContext } from 'react';

const FirebaseAppContext = createContext<FirebaseApp | undefined>(undefined);

export default FirebaseAppContext;
