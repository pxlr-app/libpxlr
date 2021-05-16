import { IconDefinition } from "@fortawesome/fontawesome-svg-core";
import React, {
	createContext,
	useContext,
	PropsWithChildren,
	useState,
} from "react";

const ToastContext = createContext<
	| {
			toasts: IdentifiableToast[];
			showToast: (toast: Omit<Toast, "time">) => string;
			removeToast: (id: string) => void;
	  }
	| undefined
>(undefined);

export type DismissableToast = {
	type: "DISMISSABLE";
	time: Date;
	ttl?: number;
	icon?: IconDefinition;
	title: string;
	body?: string;
};

export type Toast = DismissableToast;
export type IdentifiableToast = Toast & { id: string };

let nextToastId = 0;

/**
 * Returns the current toasts and functions to show or remove toast
 */
export function useToasts() {
	const ctx = useContext(ToastContext);
	if (ctx === undefined) {
		throw new Error("useToasts must be used within a ToastProvider.");
	}
	return ctx;
}

/**
 * Message container
 */
export function ToastProvider({
	children,
	initialMessages: initialToasts,
}: PropsWithChildren<{ initialMessages?: Toast[] }>) {
	const [toasts, setToasts] = useState<IdentifiableToast[]>([]);
	const value = {
		toasts,
		showToast: (toast: Omit<Toast, "time">) => {
			const id = `toast-${++nextToastId}`;
			setToasts([
				...toasts,
				{
					...toast,
					id,
					time: new Date(),
				},
			]);
			return id;
		},
		removeToast: (id: string) => {
			setToasts([...toasts].filter((t) => t.id !== id));
		},
	};
	return (
		<ToastContext.Provider value={value}>{children}</ToastContext.Provider>
	);
}
