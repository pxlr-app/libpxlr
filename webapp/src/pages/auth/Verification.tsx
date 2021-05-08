import React, { FormEvent, useCallback, useEffect } from 'react';
import { useForm } from 'react-hook-form';
import { Redirect, useHistory } from 'react-router-dom';
import { sendEmailVerification } from 'firebase/auth';
import Input from '../../components/Input';
import { useAuth, useCurrentUser } from '../../hooks/auth';
import { useToasts } from '../../hooks/toast';
import successIcon from '../../assets/icons/check-circle-regular.svg?raw';
import errorIcon from '../../assets/icons/exclamation-circle-regular.svg?raw';

export default function Verification() {
	const currentUser = useCurrentUser();
	const history = useHistory();
	const { showToast } = useToasts();

	useEffect(() => {
		if (!currentUser) {
			history.push('/auth/login', true);
		}
	}, [currentUser]);

	const onSubmit = useCallback(
		async (e: FormEvent) => {
			e.preventDefault();
			try {
				await sendEmailVerification(currentUser!, {
					url: 'http://localhost:3000/',
				});
				showToast({
					type: 'DISMISSABLE',
					ttl: 4000,
					icon: successIcon,
					title: `Verification sent`,
					body: `Check your email for instructions`,
				});
			} catch (e) {
				debugger;
				showToast({
					type: 'DISMISSABLE',
					ttl: 4000,
					icon: errorIcon,
					title: 'Could not send verification email',
					body: e.code,
				});
			}
		},
		[currentUser],
	);

	return (
		<div className="min-h-screen bg-gray-50 flex flex-col justify-center py-12 sm:px-6 lg:px-8">
			<div className="sm:mx-auto sm:w-full sm:max-w-md">
				<img
					className="mx-auto h-12 w-auto"
					src="https://tailwindui.com/img/logos/workflow-mark-indigo-600.svg"
					alt="Workflow"
				/>
				<h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900">
					Verification needed
				</h2>
			</div>

			<div className="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
				<div className="bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10">
					<form className="space-y-6" onSubmit={onSubmit}>
						<div>
							<button
								type="submit"
								className="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
							>
								Resend verification email
							</button>
						</div>
					</form>
				</div>
			</div>
		</div>
	);
}
