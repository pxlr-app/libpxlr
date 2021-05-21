import React, { Fragment, useState } from "react";
import { Dialog, Menu, Transition } from "@headlessui/react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
	faBars,
	faBell,
	faClock,
	faFile,
	faGlobe,
	faSearch,
	faUserFriends,
} from "@fortawesome/pro-duotone-svg-icons";
import {
	faTimes,
	faPlus,
	faFileUpload,
} from "@fortawesome/pro-regular-svg-icons";
import logotype from "../assets/logotype.svg";
import { useAuth } from "../hooks/auth";
import filePlaceholder from "../assets/file-preview-placeholder.png";

const navigation = [
	{
		name: "Recent",
		href: "#",
		icon: faClock,
		current: true,
	},
	{
		name: "Documents",
		href: "#",
		icon: faFile,
	},
	{
		name: "Community",
		href: "#",
		icon: faGlobe,
	},
	{
		name: "Team project",
		href: "#",
		icon: faUserFriends,
	},
];

function classNames(...classes: string[]) {
	return classes.filter(Boolean).join(" ");
}

export default function Example() {
	const [sidebarOpen, setSidebarOpen] = useState(false);
	const auth = useAuth();

	return (
		<div className="h-screen flex overflow-hidden bg-gray-100">
			<Transition.Root show={sidebarOpen} as={Fragment}>
				<Dialog
					as="div"
					static
					className="fixed inset-0 flex z-40 md:hidden"
					open={sidebarOpen}
					onClose={setSidebarOpen}
				>
					<Transition.Child
						as={Fragment}
						enter="transition-opacity ease-linear duration-300"
						enterFrom="opacity-0"
						enterTo="opacity-100"
						leave="transition-opacity ease-linear duration-300"
						leaveFrom="opacity-100"
						leaveTo="opacity-0"
					>
						<Dialog.Overlay className="fixed inset-0 bg-gray-600 bg-opacity-75" />
					</Transition.Child>
					<Transition.Child
						as={Fragment}
						enter="transition ease-in-out duration-300 transform"
						enterFrom="-translate-x-full"
						enterTo="translate-x-0"
						leave="transition ease-in-out duration-300 transform"
						leaveFrom="translate-x-0"
						leaveTo="-translate-x-full"
					>
						<div className="relative flex-1 flex flex-col max-w-xs w-full pt-5 pb-4 bg-gray-800">
							<Transition.Child
								as={Fragment}
								enter="ease-in-out duration-300"
								enterFrom="opacity-0"
								enterTo="opacity-100"
								leave="ease-in-out duration-300"
								leaveFrom="opacity-100"
								leaveTo="opacity-0"
							>
								<div className="absolute top-0 right-0 -mr-12 pt-2">
									<button
										className="ml-1 flex items-center justify-center h-10 w-10 rounded-full focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white"
										onClick={() => setSidebarOpen(false)}
									>
										<span className="sr-only">
											Close sidebar
										</span>
										<FontAwesomeIcon
											icon={faTimes}
											className="h-6 w-6 text-white"
											aria-hidden="true"
										/>
									</button>
								</div>
							</Transition.Child>
							<div className="flex-shrink-0 flex items-center px-4">
								<img
									className="h-30 w-auto"
									src={logotype}
									alt="lipsum"
								/>
							</div>
							<div className="mt-5 flex-1 h-0 overflow-y-auto">
								<nav className="px-2 space-y-1">
									{navigation.map((item) => (
										<a
											key={item.name}
											href={item.href}
											className={classNames(
												item.current
													? "bg-gray-900 text-white"
													: "text-gray-300 hover:bg-gray-700 hover:text-white",
												"group flex items-center px-2 py-2 text-base font-medium rounded-md",
											)}
										>
											<FontAwesomeIcon
												icon={item.icon}
												className={classNames(
													item.current
														? "text-gray-300"
														: "text-gray-400 group-hover:text-gray-300",
													"mr-4 h-6 w-6",
												)}
												aria-hidden="true"
											/>
											{item.name}
										</a>
									))}
								</nav>
							</div>
						</div>
					</Transition.Child>
					<div className="flex-shrink-0 w-14" aria-hidden="true">
						{/* Dummy element to force sidebar to shrink to fit close icon */}
					</div>
				</Dialog>
			</Transition.Root>

			{/* Static sidebar for desktop */}
			<div className="hidden md:flex md:flex-shrink-0">
				<div className="flex flex-col w-64">
					{/* Sidebar component, swap this element with another sidebar if you like */}
					<div className="flex flex-col h-0 flex-1">
						<div className="flex items-center h-16 flex-shrink-0 px-4 bg-gray-900">
							<img
								className="h-30 w-auto"
								src={logotype}
								alt="lipsum"
							/>
						</div>
						<div className="flex-1 flex flex-col overflow-y-auto">
							<div className="px-8 py-4 bg-gray-800 space-y-8 sm:space-y-0 sm:flex sm:justify-between sm:items-center xl:block xl:space-y-8">
								<div className="flex flex-col sm:flex-row xl:flex-col">
									<button
										type="button"
										className="inline-flex items-center justify-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 xl:w-full"
									>
										New Document
									</button>
									<button
										type="button"
										className="mt-3 inline-flex items-center justify-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:mt-0 sm:ml-3 xl:ml-0 xl:mt-3 xl:w-full"
									>
										Import Document
									</button>
								</div>
							</div>
							<nav className="flex-1 px-2 py-4 bg-gray-800 space-y-1">
								{navigation.map((item) => (
									<a
										key={item.name}
										href={item.href}
										className={classNames(
											item.current
												? "bg-gray-900 text-white"
												: "text-gray-300 hover:bg-gray-700 hover:text-white",
											"group flex items-center px-2 py-2 text-sm font-medium rounded-md",
										)}
									>
										<FontAwesomeIcon
											icon={item.icon}
											className={classNames(
												item.current
													? "text-gray-300"
													: "text-gray-400 group-hover:text-gray-300",
												"mr-3 h-6 w-6",
											)}
											aria-hidden="true"
										/>
										{item.name}
									</a>
								))}
							</nav>
						</div>
					</div>
				</div>
			</div>
			<div className="flex flex-col w-0 flex-1 overflow-hidden">
				<div className="relative z-10 flex-shrink-0 flex h-16 bg-gray-800 shadow">
					<button
						className="px-4 border-r border-gray-200 text-gray-500 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500 md:hidden"
						onClick={() => setSidebarOpen(true)}
					>
						<span className="sr-only">Open sidebar</span>
						<FontAwesomeIcon
							icon={faBars}
							className="h-6 w-6"
							aria-hidden="true"
						/>
					</button>
					<div className="flex-1 px-4 flex justify-between">
						<div className="flex-1 flex">
							<form
								className="w-full flex md:ml-0"
								action="#"
								method="GET"
							>
								<label
									htmlFor="search_field"
									className="sr-only"
								>
									Search
								</label>
								<div className="relative w-full text-gray-500 focus-within:text-gray-300">
									<div className="absolute inset-y-0 left-0 flex items-center pointer-events-none">
										<FontAwesomeIcon
											icon={faSearch}
											className="h-5 w-5"
											aria-hidden="true"
										/>
									</div>
									<input
										id="search_field"
										className="block w-full h-full pl-8 pr-3 py-2 border-transparent bg-gray-800 text-gray-500 placeholder-gray-500 focus:text-gray-300 focus:outline-none focus:placeholder-gray-300 focus:ring-0 focus:border-transparent sm:text-sm"
										placeholder="Search"
										type="text"
										name="search"
									/>
								</div>
							</form>
						</div>
						<div className="ml-4 flex items-center md:ml-6">
							<button className="bg-transparent p-1 rounded-full text-gray-300 hover:text-gray-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-800 focus:ring-white">
								<span className="sr-only">
									View notifications
								</span>
								<FontAwesomeIcon
									icon={faBell}
									className="h-6 w-6"
									aria-hidden="true"
								/>
							</button>

							{/* Profile dropdown */}
							<Menu as="div" className="ml-3 relative">
								{({ open }) => (
									<>
										<div>
											<Menu.Button className="max-w-xs bg-transparent flex items-center text-sm rounded-full focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-800 focus:ring-white">
												<span className="sr-only">
													Open user menu
												</span>
												<img
													className="h-8 w-8 rounded-full"
													src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"
													alt=""
												/>
											</Menu.Button>
										</div>
										<Transition
											show={open}
											as={Fragment}
											enter="transition ease-out duration-100"
											enterFrom="transform opacity-0 scale-95"
											enterTo="transform opacity-100 scale-100"
											leave="transition ease-in duration-75"
											leaveFrom="transform opacity-100 scale-100"
											leaveTo="transform opacity-0 scale-95"
										>
											<Menu.Items
												static
												className="origin-top-right absolute right-0 mt-2 w-48 rounded-md shadow-lg py-1 bg-white ring-1 ring-black ring-opacity-5 focus:outline-none"
											>
												<Menu.Item key={"profile"}>
													{({ active }) => (
														<a
															href={"#"}
															className={classNames(
																active
																	? "bg-gray-100"
																	: "",
																"block px-4 py-2 text-sm text-gray-700",
															)}
														>
															Your profile
														</a>
													)}
												</Menu.Item>
												<Menu.Item key={"signout"}>
													{({ active }) => (
														<a
															onClick={(e) =>
																auth?.signOut()
															}
															className={classNames(
																active
																	? "bg-gray-100"
																	: "",
																"block px-4 py-2 text-sm text-gray-700",
															)}
														>
															Sign out
														</a>
													)}
												</Menu.Item>
											</Menu.Items>
										</Transition>
									</>
								)}
							</Menu>
						</div>
					</div>
				</div>

				<main className="flex-1 relative overflow-y-auto focus:outline-none">
					<div className="py-6">
						<div className="max-w-7xl mx-auto px-4 sm:px-6 md:px-8">
							<h1 className="text-2xl font-semibold text-gray-900">
								Recent
							</h1>
						</div>
						<div className="max-w-7xl mx-auto px-4 sm:px-6 md:px-8">
							<div className="col-span-3 py-8 grid sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6 sm:gap-y-8 lg:gap-x-8">
								{[2, 3, 4].map((day) => (
									<a
										href="#"
										className="group relative bg-white rounded-lg shadow-sm overflow-hidden ring-1 ring-black ring-opacity-5"
									>
										<figure>
											<div className="relative bg-gray-100 pt-[50%] overflow-hidden">
												<div className="absolute inset-0 w-full h-full rounded-t-lg overflow-hidden">
													<img
														src={filePlaceholder}
														alt=""
														className="absolute inset-0 w-full h-full"
													/>
												</div>
											</div>
											<figcaption className="py-3 px-4">
												<p className="text-sm font-medium text-gray-900 mb-1">
													Untitle document
												</p>
												<p className="text-xs font-medium text-gray-300">
													Edited {day} days ago
												</p>
											</figcaption>
										</figure>
									</a>
								))}
							</div>
						</div>
					</div>
				</main>
			</div>
		</div>
	);
}
