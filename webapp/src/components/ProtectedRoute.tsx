import { onAuthStateChanged, User } from "firebase/auth";
import React, { useEffect, useState } from "react";
import {
	Route,
	Redirect,
	RouteProps,
	RouteComponentProps,
} from "react-router-dom";
import { useAuth } from "../hooks/auth";

export default function ProtectedRoute({ children, ...rest }: RouteProps) {
	const auth = useAuth();
	const [currentUser, setCurrentUser] = useState<User | undefined>(
		auth?.currentUser ?? undefined,
	);
	useEffect(() => {
		if (auth) {
			const unsubscribe = onAuthStateChanged(auth, (u) =>
				setCurrentUser(u ?? undefined),
			);
			return () => unsubscribe();
		}
	}, [auth]);
	const render = ({ location }: RouteComponentProps) => {
		if (!currentUser) {
			return (
				<Redirect
					to={{ pathname: "/auth/login", state: { from: location } }}
				/>
			);
		} else if (!currentUser.emailVerified) {
			return (
				<Redirect
					to={{
						pathname: "/auth/verification",
						state: { from: location },
					}}
				/>
			);
		} else {
			return children;
		}
	};
	return <Route {...rest} render={render} />;
}
