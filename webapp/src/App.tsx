import type { Component } from "solid-js";
import { createContext } from "solid-js";
import { createStore } from "solid-js/store";

import styles from "./App.module.css";

export const CounterContext =
	createContext<
		[{ count: number }, { increment: () => void; decrement: () => void }]
	>();

export const CounterProvider: Component = (props: any) => {
	const [state, setState] = createStore({ count: 0 });
	const store: [
		{ count: number },
		{ increment: () => void; decrement: () => void },
	] = [
		state,
		{
			increment() {
				setState("count", (c) => c + 1);
			},
			decrement() {
				setState("count", (c) => c - 1);
			},
		},
	];

	return (
		<CounterContext.Provider value={store}>
			{props.children}
		</CounterContext.Provider>
	);
};

const App: Component = () => {
	return (
		<div class={styles.App}>
			<header class={styles.header}>
				<p>
					Edit <code>src/App.tsx</code> and save to reload.
				</p>
				<CounterProvider>
					<a
						class={styles.link}
						href="https://github.com/ryansolid/solid"
						target="_blank"
						rel="noopener noreferrer"
					>
						Learn Solid
					</a>
				</CounterProvider>
			</header>
		</div>
	);
};

export default App;
