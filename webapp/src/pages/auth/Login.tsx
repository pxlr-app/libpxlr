import React from "react";
import { useForm } from "react-hook-form";
import { Link, useHistory } from "react-router-dom";
import {
	signInWithEmailAndPassword,
	setPersistence,
	inMemoryPersistence,
	browserLocalPersistence,
} from "firebase/auth";
import { useAuth } from "../../hooks/auth";
import { useToasts } from "../../hooks/toast";
import successIcon from "../../assets/icons/check-circle-regular.svg?raw";
import errorIcon from "../../assets/icons/exclamation-circle-regular.svg?raw";
import Input from "../../components/Input";
import Checkbox from "../../components/Checkbox";
import suspend from "../../utils/suspend";

type LoginForm = {
	email: string;
	password: string;
	remember_me: boolean;
};

export default function Login(props: React.PropsWithChildren<{}>) {
	const { register, handleSubmit } = useForm<LoginForm>({
		mode: "onChange",
	});
	const auth = useAuth();
	const history = useHistory();
	const { showToast } = useToasts();

	const onSubmit = handleSubmit(async ({ email, password, remember_me }) => {
		try {
			await setPersistence(
				auth!,
				remember_me ? browserLocalPersistence : inMemoryPersistence,
			);
			const userCred = await signInWithEmailAndPassword(
				auth!,
				email,
				password,
			);
			showToast({
				type: "DISMISSABLE",
				ttl: 4000,
				icon: successIcon,
				title: `Authentification successful`,
				body: `Welcome ${
					userCred.user.displayName ?? userCred.user.email
				}`,
			});
			history.push("/", true);
		} catch (e) {
			showToast({
				type: "DISMISSABLE",
				ttl: 4000,
				icon: errorIcon,
				title: "Authentification error",
				body: e.code,
			});
		}
	});

	return (
		<div className="min-h-screen bg-gray-50 flex flex-col justify-center py-12 sm:px-6 lg:px-8">
			<div className="sm:mx-auto sm:w-full sm:max-w-md">
				<img
					className="mx-auto h-12 w-auto"
					src="https://tailwindui.com/img/logos/workflow-mark-indigo-600.svg"
					alt="Workflow"
				/>
				<h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900">
					Sign in to your account
				</h2>
				<p className="mt-2 text-center text-sm text-gray-600 max-w">
					Or{" "}
					<Link
						to="/auth/register"
						className="font-medium text-indigo-600 hover:text-indigo-500"
					>
						start your 14-day free trial
					</Link>
				</p>
			</div>

			<div className="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
				<div className="bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10">
					<form className="space-y-6" onSubmit={onSubmit}>
						<Input
							label="Email address"
							// name="email"
							type="email"
							autoComplete="email"
							required
							{...register("email", {
								required: true,
								minLength: 4,
							})}
							// error={errors.email}
						/>

						<Input
							label="Password"
							// name="password"
							type="password"
							autoComplete="new_password"
							required
							{...register("password", {
								required: true,
								minLength: 1,
							})}
							// error={errors.password}
						/>

						<div className="flex items-center justify-between">
							<Checkbox
								label="Remember me"
								// name="remember_me"
								{...register("remember_me")}
							/>

							<div className="text-sm">
								<Link
									to="/auth/forgot"
									className="font-medium text-indigo-600 hover:text-indigo-500"
								>
									Forgot your password?
								</Link>
							</div>
						</div>

						<div>
							<button
								type="submit"
								className="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
							>
								Sign in
							</button>
						</div>
					</form>
				</div>
			</div>
		</div>
	);
}
