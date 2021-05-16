/**
 * Suspend until promise resolve or throws
 * 
 * ```ts
 * const waiter = suspend(new Promise(r => setTimeout(r, 2000)));
 * 
 * function Component() {
 *   const _ = waiter(); // suspend component for 2 seconds
 *   // ...
 * }
 * ```
 */
export default function <T = unknown, E = Error>(promise: Promise<T>) {
	let status = 0;
	let result: T;
	let error: E;
	const suspender = promise.then(r => {
		result = r;
		status = 1;
	}, e => {
		error = e;
		status = 2;
	});

	return () => {
		switch (status) {
			case 0: throw suspender;
			case 1: return result;
			default: throw error;
		}
	};
}