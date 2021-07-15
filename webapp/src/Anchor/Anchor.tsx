import {
	Component,
	PropsWithChildren,
	JSX,
	createEffect,
	createMemo,
	onCleanup,
	onMount,
} from "solid-js";

export enum VerticalAlign {
	TOP = "TOP",
	MIDDLE = "MIDDLE",
	BOTTOM = "BOTTOM",
}

export enum HorizontalAlign {
	LEFT = "LEFT",
	CENTER = "CENTER",
	RIGHT = "RIGHT",
}

export type Alignement = [HorizontalAlign, VerticalAlign];

export type Constraints = {
	element?: HTMLElement | (() => HTMLElement);
	origins: { anchor: Alignement; transform: Alignement }[];
};

export type AnchorProps =
	| {
			anchorOrigin: Alignement;
			transformOrigin: Alignement;
			class?: string;
	  }
	| {
			constraints: Constraints;
			class?: string;
	  };

function getOrRetrieve<T>(value: T | (() => T)): T {
	return value instanceof Function ? value() : value;
}

function rectContains(a: DOMRect, b: DOMRect): boolean {
	return (
		a.left <= b.left &&
		b.right <= a.right &&
		a.top <= b.top &&
		b.bottom <= a.bottom
	);
}
function rectOverlaps(a: DOMRect, b: DOMRect): boolean {
	return (
		Math.max(a.left, b.left) < Math.min(a.right, b.right) &&
		Math.max(a.top, b.top) < Math.min(a.bottom, b.bottom)
	);
}

function rectIntersection(a: DOMRect, b: DOMRect): DOMRect {
	const left = Math.max(a.left, b.left);
	const right = Math.min(a.right, b.right);
	const top = Math.max(a.top, b.top);
	const bottom = Math.min(a.bottom, b.bottom);
	return new DOMRect(
		Math.min(left, right),
		Math.min(top, bottom),
		Math.max(left, right) - Math.min(left, right),
		Math.max(top, bottom) - Math.min(top, bottom),
	);
}

export const Anchor: Component<PropsWithChildren<AnchorProps>> = (props) => {
	let anchor: HTMLDivElement | undefined;
	let transform: HTMLDivElement | undefined;

	const recalc = createMemo(() => () => {
		if (anchor && transform) {
			let newAnchorOrigin: Alignement | undefined = undefined;
			let newTransformOrigin: Alignement | undefined = undefined;

			if ("anchorOrigin" in props && "transformOrigin" in props) {
				newAnchorOrigin = props.anchorOrigin;
				newTransformOrigin = props.transformOrigin;
			} else if ("constraints" in props) {
				if (props.constraints.origins.length === 0) {
					throw new Error("Needs at least one constraints.origins.");
				}

				const anchorParentBounds = anchor.parentElement!.getBoundingClientRect();
				const transformBounds = transform.getBoundingClientRect();
				const constraintElement = props.constraints.element
					? getOrRetrieve(props.constraints.element)
					: document.body.parentElement!;
				const constraintsBounds = constraintElement.getBoundingClientRect();

				const w = transformBounds.width;
				const h = transformBounds.height;

				const origins: {
					overflow: number;
					align: { anchor: Alignement; transform: Alignement };
				}[] = [];

				for (const origin of props.constraints.origins) {
					let anchorX = anchorParentBounds.left;
					if (origin.anchor[0] === HorizontalAlign.CENTER) {
						anchorX =
							anchorParentBounds.left +
							anchorParentBounds.width / 2;
					} else if (origin.anchor[0] === HorizontalAlign.RIGHT) {
						anchorX = anchorParentBounds.right;
					}
					let anchorY = anchorParentBounds.top;
					if (origin.anchor[1] === VerticalAlign.MIDDLE) {
						anchorY =
							anchorParentBounds.top +
							anchorParentBounds.height / 2;
					} else if (origin.anchor[1] === VerticalAlign.BOTTOM) {
						anchorY = anchorParentBounds.bottom;
					}
					let x = anchorX;
					if (origin.transform[0] === HorizontalAlign.CENTER) {
						x = anchorX - w / 2;
					} else if (origin.transform[0] === HorizontalAlign.RIGHT) {
						x = anchorX - w;
					}
					let y = anchorY;
					if (origin.transform[1] === VerticalAlign.MIDDLE) {
						y = anchorY - h / 2;
					} else if (origin.transform[1] === VerticalAlign.BOTTOM) {
						y = anchorY - h;
					}
					const prospectBounds = new DOMRect(x, y, w, h);
					const prospectOverlaps = rectOverlaps(
						constraintsBounds,
						prospectBounds,
					);
					const overflowBounds = rectIntersection(
						constraintsBounds,
						prospectBounds,
					);

					const overflow = prospectOverlaps
						? (overflowBounds.width * overflowBounds.height) /
						  (prospectBounds.width * prospectBounds.height)
						: 0;

					origins.push({
						overflow,
						align: origin,
					});
				}

				origins.sort((a, b) => b.overflow - a.overflow);

				newAnchorOrigin = origins[0].align.anchor;
				newTransformOrigin = origins[0].align.transform;
			}

			if (newAnchorOrigin && newTransformOrigin) {
				anchor.style.position = "absolute";

				if (newAnchorOrigin[0] === HorizontalAlign.LEFT) {
					anchor.style.left = "0";
					anchor.style.right = "auto";
				} else if (newAnchorOrigin[0] === HorizontalAlign.CENTER) {
					anchor.style.left = "50%";
					anchor.style.right = "auto";
				} else {
					anchor.style.left = "auto";
					anchor.style.right = "0";
				}

				if (newAnchorOrigin[1] === VerticalAlign.TOP) {
					anchor.style.top = "0";
					anchor.style.bottom = "auto";
				} else if (newAnchorOrigin[1] === VerticalAlign.MIDDLE) {
					anchor.style.top = "50%";
					anchor.style.bottom = "auto";
				} else {
					anchor.style.top = "auto";
					anchor.style.bottom = "0";
				}

				transform.style.position = "absolute";
				const t = ["0", "0"];

				if (newTransformOrigin[0] === HorizontalAlign.LEFT) {
					transform.style.left = "0";
					transform.style.right = "auto";
				} else if (newTransformOrigin[0] === HorizontalAlign.CENTER) {
					transform.style.left = "50%";
					transform.style.right = "auto";
					t[0] = "-50%";
				} else {
					transform.style.left = "auto";
					transform.style.right = "0";
				}

				if (newTransformOrigin[1] === VerticalAlign.TOP) {
					transform.style.top = "0";
					transform.style.bottom = "auto";
				} else if (newTransformOrigin[1] === VerticalAlign.MIDDLE) {
					transform.style.top = "0";
					transform.style.bottom = "auto";
					t[1] = "-50%";
				} else {
					transform.style.top = "auto";
					transform.style.bottom = "0";
				}
				transform.style.transform = `translate(${t[0]},${t[1]})`;
			}
		}
	});

	window.addEventListener("resize", recalc);

	onCleanup(() => {
		window.removeEventListener("resize", recalc);
	});

	createEffect(() => {
		recalc()();
	});

	return (
		<div data-role="anchor-origin" ref={anchor}>
			<div data-role="anchor-transform" ref={transform}>
				{props.children}
			</div>
		</div>
	);
};
