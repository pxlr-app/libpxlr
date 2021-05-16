import React, { useState } from "react";
import { useToasts, IdentifiableToast } from "../hooks/toast";
import { Transition, animated, Spring } from "react-spring";
import { faTimes } from "@fortawesome/pro-regular-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

export default function Toasts() {
	const { toasts, removeToast } = useToasts();
	const [refMap] = useState(new WeakMap<IdentifiableToast, HTMLDivElement>());

	return (
		<div className="fixed z-40 inset-0 flex flex-col items-end px-4 py-6 pointer-events-none sm:p-6">
			<Transition
				items={toasts}
				keys={(t: IdentifiableToast) => t.id}
				from={{ transform: "translate3d(120%, 0, 0)" }}
				enter={(toast) => async (next) => {
					await next({
						transform: "translate3d(0%, 0, 0)",
						height: refMap.get(toast)!.offsetHeight,
					});
				}}
				leave={(toast) => async (next) => {
					await next({
						transform: "translate3d(120%, 0, 0)",
					});
					await next({ height: 0 });
				}}
			>
				{(props, toast) => (
					<animated.div
						style={props}
						className="relative max-w-sm w-full overflow-hidden"
					>
						<div
							ref={(ref) => ref && refMap.set(toast, ref)}
							className="p-1 pb-4"
						>
							<div className="relative w-full bg-white shadow-lg rounded-lg pointer-events-auto ring-1 ring-black ring-opacity-5">
								<div className="p-4">
									<div className="flex items-start">
										<div className="flex-shrink-0">
											{toast.icon && (
												<FontAwesomeIcon
													icon={toast.icon}
													className="h-6 w-6 text-gray-400"
												/>
											)}
										</div>
										<div className="ml-3 w-0 flex-1 pt-0.5">
											<p className="text-sm font-medium text-gray-900">
												{toast.title}
											</p>
											{toast.body && (
												<p className="mt-1 text-sm text-gray-500">
													{toast.body}
												</p>
											)}
											{/* <div className="mt-2">
										<button className="bg-white rounded-md text-sm font-medium text-indigo-600 hover:text-indigo-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
											Undo
										</button>
										<button className="ml-6 bg-white rounded-md text-sm font-medium text-gray-700 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
											Dismiss
										</button>
									</div> */}
										</div>
										<div className="ml-4 flex-shrink-0 flex">
											<button
												onClick={(e) => {
													e.preventDefault();
													removeToast(toast.id);
												}}
												className="bg-white rounded-md inline-flex text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
											>
												<span className="sr-only">
													Close
												</span>
												{/* <!-- Heroicon name: solid/x --> */}
												<FontAwesomeIcon
													icon={faTimes}
													className="h-5 w-5"
												/>
											</button>
										</div>
									</div>
								</div>
								{toast.ttl && (
									<div className="absolute bottom-0 w-full h-1 bg-pink-300">
										<Spring
											from={{ transform: `scaleX(1)` }}
											to={{ transform: `scaleX(0)` }}
											config={{ duration: toast.ttl }}
											onRest={() => removeToast(toast.id)}
										>
											{(props) => (
												<animated.div
													className="absolute inset-0 bg-pink-500 origin-top-left"
													style={props}
												/>
											)}
										</Spring>
									</div>
								)}
							</div>
						</div>
					</animated.div>
				)}
			</Transition>
		</div>
	);
}
