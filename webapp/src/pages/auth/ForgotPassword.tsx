import React from "react";
import { useForm } from "react-hook-form";
import { useHistory } from "react-router-dom";
import { sendPasswordResetEmail } from "firebase/auth";
import Input from "../../components/Input";
import { useAuth } from "../../hooks/auth";
import { useToasts } from "../../hooks/toast";
import {
	faCheckCircle,
	faExclamationCircle,
} from "@fortawesome/pro-duotone-svg-icons";

type ForgotForm = {
	email: string;
};

export default function ForgotPassword() {
	const { register, handleSubmit } = useForm<ForgotForm>({
		mode: "onChange",
	});

	const auth = useAuth();
	const history = useHistory();
	const { showToast } = useToasts();

	const onSubmit = handleSubmit(async ({ email }) => {
		try {
			await sendPasswordResetEmail(auth!, email, {
				url: "http://localhost:3000/auth/reset",
			});
			showToast({
				type: "DISMISSABLE",
				ttl: 4000,
				icon: faCheckCircle,
				title: `Reset password sent`,
				body: `Check your email for instructions`,
			});
			history.push("/auth/login", true);
		} catch (e) {
			showToast({
				type: "DISMISSABLE",
				ttl: 4000,
				icon: faExclamationCircle,
				title: "Could not send reset password",
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
					Reset your password
				</h2>
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

						<div>
							<button
								type="submit"
								className="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
							>
								Send password reset email
							</button>
						</div>
					</form>
				</div>
			</div>
		</div>
	);
}
