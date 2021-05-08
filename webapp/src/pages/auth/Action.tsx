import React, { useEffect, useState } from "react";
import { useForm } from "react-hook-form";
import { useHistory, useLocation } from "react-router-dom";
import {
	applyActionCode,
	confirmPasswordReset,
	verifyPasswordResetCode,
} from "firebase/auth";
import Input from "../../components/Input";
import { useAuth } from "../../hooks/auth";
import { useToasts } from "../../hooks/toast";
import successIcon from "../../assets/icons/check-circle-regular.svg?raw";
import errorIcon from "../../assets/icons/exclamation-circle-regular.svg?raw";
import Error404 from "../404";
import suspend from "../../utils/suspend";

export default function Action() {
	const location = useLocation();
	const searchParams = new URLSearchParams(location.search);
	const mode = searchParams.get("mode");
	const oobCode = searchParams.get("oobCode") ?? "";

	switch (mode) {
		case "reset":
			return <ResetForm oobCode={oobCode} />;
		case "verifyEmail":
			return <VerifyEmailForm oobCode={oobCode} />;
	}
	return <Error404 />;
}

function ResetForm({ oobCode }: { oobCode: string }) {
	const { register, handleSubmit } = useForm<{ newPassword: string }>({
		mode: "onChange",
	});

	const history = useHistory();
	const { showToast } = useToasts();
	const auth = useAuth();

	const [email, setEmail] = useState("");

	useEffect(() => {
		verifyPasswordResetCode(auth!, oobCode)
			.then((email) => setEmail(email))
			.catch((err) => {
				showToast({
					type: "DISMISSABLE",
					ttl: 4000,
					icon: errorIcon,
					title: "Password reset error",
					body: `Could not verify password reset code`,
				});
				history.push("/auth/forgot", true);
			});
	}, [oobCode]);

	const onSubmit = handleSubmit(async ({ newPassword }) => {
		try {
			await confirmPasswordReset(auth!, oobCode, newPassword);
			showToast({
				type: "DISMISSABLE",
				ttl: 4000,
				icon: successIcon,
				title: "Password reset successful",
				body: `You can now sign in with new password`,
			});
			history.push("/auth/login", true);
		} catch (e) {
			showToast({
				type: "DISMISSABLE",
				ttl: 4000,
				icon: errorIcon,
				title: "Password reset error",
				body: `Could not verify password reset code`,
			});
			history.push("/auth/forgot", true);
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
					Confirm new password
				</h2>
			</div>

			<div className="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
				<div className="bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10">
					<form className="space-y-6" onSubmit={onSubmit}>
						<Input
							label={`New password for ${email}`}
							// name="newPassword"
							type="password"
							autoComplete="newPassword"
							required
							{...register("newPassword", {
								required: true,
							})}
							// error={errors.newPassword}
						/>

						<div>
							<button
								type="submit"
								className="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
							>
								Set new password
							</button>
						</div>
					</form>
				</div>
			</div>
		</div>
	);
}

function VerifyEmailForm({ oobCode }: { oobCode: string }) {
	const history = useHistory();
	const { showToast } = useToasts();
	const auth = useAuth();

	const _ = suspend(
		applyActionCode(auth!, oobCode)
			.then((_) => {
				showToast({
					type: "DISMISSABLE",
					ttl: 4000,
					icon: successIcon,
					title: "Verification success",
					body: `Successfully verificated email`,
				});
				history.push("/", true);
			})
			.catch((_) => {
				showToast({
					type: "DISMISSABLE",
					ttl: 4000,
					icon: errorIcon,
					title: "Verification error",
					body: `Could not verify email`,
				});
				history.push("/auth/verification", true);
			}),
	);

	return null;
}
