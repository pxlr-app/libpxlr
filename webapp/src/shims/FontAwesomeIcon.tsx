import { Component, createMemo } from "solid-js";
import {
	AbstractElement,
	icon,
	parse,
} from "@fortawesome/fontawesome-svg-core";
import {
	IconParams,
	IconName,
	IconLookup,
} from "@fortawesome/fontawesome-svg-core";
import { Dynamic, createComponent } from "solid-js/web";

export type FontAwesomeIconProps = IconParams & {
	icon: IconName | IconLookup;
};

function createElementFromAbstract(
	abstract: AbstractElement,
): HTMLElement | SVGElement {
	function inner(abstract: AbstractElement): SVGElement {
		const element = document.createElementNS(
			"http://www.w3.org/2000/svg",
			abstract.tag,
		);

		if (abstract.attributes) {
			Object.keys(abstract.attributes).map((key) => {
				element.setAttribute(key, abstract.attributes[key]);
			});
		}
		if (abstract.children) {
			for (const child of abstract.children) {
				element.appendChild(inner(child));
			}
		}
		return element;
	}
	return inner(abstract);
}

const FontAwesomeIcon: Component<FontAwesomeIconProps> = (props) => {
	const renderedIcon = createMemo(() => icon(props.icon, props));
	if (!renderedIcon()) {
		console.error("Could not find icon", props.icon);
		return null;
	}

	const element = createMemo(() => {
		const {
			abstract: [icon],
		} = renderedIcon();
		const element = createElementFromAbstract(icon);

		return element;
	});

	return element;
};

export default FontAwesomeIcon;
