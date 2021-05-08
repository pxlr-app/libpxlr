import React from 'react';
import {
	Route,
	Redirect,
	RouteProps,
	RouteComponentProps,
} from 'react-router-dom';
import { useCurrentUser } from '../hooks/auth';

export default function ProtectedRoute({ children, ...rest }: RouteProps) {
	const currentUser = useCurrentUser();
	const render = ({ location }: RouteComponentProps) => {
		if (!currentUser) {
			return (
				<Redirect
					to={{ pathname: '/auth/login', state: { from: location } }}
				/>
			);
		} else if (!currentUser.emailVerified) {
			return (
				<Redirect
					to={{
						pathname: '/auth/verification',
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
