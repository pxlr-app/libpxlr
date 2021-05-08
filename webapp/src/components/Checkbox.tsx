import React, {
	ClassAttributes,
	InputHTMLAttributes,
	forwardRef,
	ForwardedRef,
} from 'react';
import { FieldValues, FieldError } from 'react-hook-form';
import errorIcon from '../assets/icons/exclamation-circle-regular.svg?raw';

export type InputProps<TFieldValues extends FieldValues = FieldValues> = {
	label: string;
	error?: FieldError;
} & Omit<InputHTMLAttributes<HTMLInputElement>, 'id' | 'type'> &
	ClassAttributes<HTMLInputElement>;

export default forwardRef(function Checkbox<
	TFieldValues extends FieldValues = FieldValues
>(
	{ label, error, ...props }: InputProps<TFieldValues>,
	ref: ForwardedRef<HTMLInputElement>,
) {
	const { name, ...attrs } = props;

	return (
		<div className="flex items-center">
			<input
				{...attrs}
				type="checkbox"
				id={name}
				name={name}
				ref={ref}
				className="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded"
			/>
			<label htmlFor={name} className="ml-2 block text-sm text-gray-900">
				{label}
			</label>
		</div>
	);
});
