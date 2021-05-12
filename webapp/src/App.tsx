import React, { Suspense, lazy } from "react";
import { Switch, BrowserRouter as Router, Route } from "react-router-dom";
import { getApps, initializeApp } from "firebase/app";
import FirebaseAppContext from "./contexts/firebase";
import "./App.css";
import ProtectedRoute from "./components/ProtectedRoute";
import Error404 from "./pages/404";
import Toasts from "./components/Toasts";
import { ToastProvider } from "./hooks/toast";

const firebaseApp =
	getApps().find((app) => app.name === "pxlrapp") ??
	initializeApp(
		{
			apiKey: import.meta.env.VITE_FIREBASE_API_KEY as string,
			authDomain: import.meta.env.VITE_FIREBASE_AUTH_DOMAIN as string,
			databaseURL: import.meta.env.VITE_FIREBASE_DATABASE_URL as string,
			projectId: import.meta.env.VITE_FIREBASE_PROJECT_ID as string,
			storageBucket: import.meta.env
				.VITE_FIREBASE_STORAGE_BUCKET as string,
			messagingSenderId: import.meta.env
				.VITE_FIREBASE_MESSAGING_SENDER_ID as string,
			appId: import.meta.env.VITE_FIREBASE_APP_ID as string,
		},
		"pxlrapp",
	);

const LoginPage = lazy(() => import("./pages/auth/Login"));
const ForgotPasswordPage = lazy(() => import("./pages/auth/ForgotPassword"));
const ActionPage = lazy(() => import("./pages/auth/Action"));
const VerificationPage = lazy(() => import("./pages/auth/Verification"));
const DemoPage = lazy(() => import("./pages/Demo"));
const FileApiPage = lazy(() => import("./pages/FileApi"));

export default function App() {
	const Loading = () => <div>Loading...</div>;
	return (
		<FirebaseAppContext.Provider value={firebaseApp}>
			<ToastProvider>
				<Toasts />
				<Router>
					<Suspense fallback={<Loading />}>
						<Switch>
							<Route path="/auth/login">
								<LoginPage />
							</Route>
							<Route path="/auth/forgot">
								<ForgotPasswordPage />
							</Route>
							<Route path="/auth/action">
								<ActionPage />
							</Route>
							<Route path="/auth/verification">
								<VerificationPage />
							</Route>
							<ProtectedRoute exact path="/">
								<DemoPage />
							</ProtectedRoute>
							<ProtectedRoute path="/fileapi">
								<FileApiPage />
							</ProtectedRoute>
							<Route path="*">
								<Error404 />
							</Route>
						</Switch>
					</Suspense>
				</Router>
			</ToastProvider>
		</FirebaseAppContext.Provider>
	);
}
