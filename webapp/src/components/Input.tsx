import { faExclamationCircle } from "@fortawesome/pro-duotone-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import React, {
	ClassAttributes,
	InputHTMLAttributes,
	forwardRef,
	ForwardedRef,
} from "react";
import { FieldValues, FieldError } from "react-hook-form";

export type InputProps<TFieldValues extends FieldValues = FieldValues> = {
	label: string;
	error?: FieldError;
} & Omit<InputHTMLAttributes<HTMLInputElement>, "id"> &
	ClassAttributes<HTMLInputElement>;

export default forwardRef(function Input<
	TFieldValues extends FieldValues = FieldValues
>(
	{ label, error, ...props }: InputProps<TFieldValues>,
	ref: ForwardedRef<HTMLInputElement>,
) {
	const { name, ...attrs } = props;

	return (
		<div>
			<label
				htmlFor={name}
				className="block text-sm font-medium text-gray-700"
			>
				{label}
			</label>
			<div className="mt-1 relative">
				<input
					{...attrs}
					id={name}
					name={name}
					ref={ref}
					className="appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm placeholder-gray-400 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
				/>
				{error && (
					<div className="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
						<FontAwesomeIcon
							icon={faExclamationCircle}
							className="h-5 w-5 text-red-500"
						/>
					</div>
				)}
			</div>
			{error && error.message && (
				<p className="mt-2 text-sm text-red-600" id="email-error">
					{error.message}
				</p>
			)}
		</div>
	);
});
