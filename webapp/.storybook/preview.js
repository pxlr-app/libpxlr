import { createRoot } from "solid-js";
import { render } from "solid-js/web";
import "../src/App.css";
import "./preview.css";

export const parameters = {
	actions: { argTypesRegex: "^on[A-Z].*" },
	controls: {
		matchers: {
			color: /(background|color)$/i,
			date: /Date$/,
		},
	},
};

export const decorators = [
	Story => {
		const root = document.getElementById("root");
		const solid = document.createElement("div");
		solid.setAttribute('id', 'solid-root');

		root.appendChild(solid);
		render(Story, solid);
		return solid;
		// return createRoot(() => Story());
	}
]