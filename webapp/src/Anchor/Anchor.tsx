import React, {
	useEffect,
	useLayoutEffect,
	useMemo,
	useRef,
	useState,
} from "react";

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

const LEFT = HorizontalAlign.LEFT;
const CENTER = HorizontalAlign.CENTER;
const RIGHT = HorizontalAlign.RIGHT;
const TOP = VerticalAlign.TOP;
const MIDDLE = VerticalAlign.MIDDLE;
const BOTTOM = VerticalAlign.BOTTOM;

export type Alignement = [HorizontalAlign, VerticalAlign];

export type Constraints = {
	element?: HTMLElement | (() => HTMLElement);
	preventOverlap?: boolean;
	origins: { anchor: Alignement; transform: Alignement }[];
};

export type AnchorData = {
	transformRef: React.MutableRefObject<HTMLElement | undefined>;
	anchorOrigin: Alignement;
	transformOrigin: Alignement;
};

export type AnchorContainerProps = {
	anchorOrigin?: Alignement;
	transformOrigin?: Alignement;
	constraints?: Constraints;

	className?: string;
	children: (context: AnchorData) => React.ReactNode;
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
	return new DOMRect(left, top, right - left, bottom - top);
}

export function Anchor({
	className,
	constraints,
	anchorOrigin,
	transformOrigin,
	children,
}: AnchorContainerProps) {
	if (!constraints && (!anchorOrigin || !transformOrigin)) {
		throw new Error(
			"Need to specify either `constraints` attribute, or both `anchorOrigin` and `transformOrigin` attributes.",
		);
	}

	if (constraints && (anchorOrigin || transformOrigin)) {
		throw new Error(
			"Can not specify `constraints` attribute along side `anchorOrigin` and `transformOrigin`.",
		);
	}

	if (constraints && constraints.origins.length === 0) {
		throw new Error("Needs at least one constraints.origins.");
	}

	const anchorRef = useRef<HTMLDivElement>(null);
	const transformRef = useRef<HTMLElement>();
	const [currentAnchorOrigin, setCurrentAnchorOrigin] = useState<Alignement>([
		CENTER,
		MIDDLE,
	]);
	const [
		currentTransformOrigin,
		setCurrentTransformOrigin,
	] = useState<Alignement>([CENTER, MIDDLE]);

	const recalcTransformPosition = useMemo(
		() => () => {
			if (anchorRef.current && transformRef.current) {
				let hAnchorOrigin: HorizontalAlign = CENTER;
				let vAnchorOrigin: VerticalAlign = MIDDLE;
				let hTransformOrigin: HorizontalAlign = CENTER;
				let vTransformOrigin: VerticalAlign = MIDDLE;

				// Use specified origins
				if (anchorOrigin && transformOrigin) {
					hAnchorOrigin = anchorOrigin[0];
					vAnchorOrigin = anchorOrigin[1];
					hTransformOrigin = transformOrigin[0];
					vTransformOrigin = transformOrigin[1];
				}
				// Use constraints
				else if (constraints) {
					const transformBounds = transformRef.current.getBoundingClientRect();
					const anchorParentBounds = anchorRef.current.parentElement!.getBoundingClientRect();

					const constraintElement = constraints?.element
						? getOrRetrieve(constraints.element)
						: document.body.parentElement!;
					const constraintsBounds = constraintElement.getBoundingClientRect();

					const w = transformBounds.width;
					const h = transformBounds.height;

					hAnchorOrigin = constraints.origins[0].anchor[0];
					vAnchorOrigin = constraints.origins[0].anchor[1];
					hTransformOrigin = constraints.origins[0].transform[0];
					vTransformOrigin = constraints.origins[0].transform[1];

					for (const {
						anchor: anchorOrigin,
						transform: transformOrigin,
					} of constraints.origins) {
						let anchorX = anchorParentBounds.left;
						if (anchorOrigin[0] === CENTER) {
							anchorX =
								anchorParentBounds.left +
								anchorParentBounds.width / 2;
						} else if (anchorOrigin[0] === RIGHT) {
							anchorX = anchorParentBounds.right;
						}
						let anchorY = anchorParentBounds.top;
						if (anchorOrigin[1] === MIDDLE) {
							anchorY =
								anchorParentBounds.top +
								anchorParentBounds.height / 2;
						} else if (anchorOrigin[1] === BOTTOM) {
							anchorX = anchorParentBounds.bottom;
						}
						let x = anchorX;
						if (transformOrigin[0] === CENTER) {
							x = anchorX - w / 2;
						} else if (transformOrigin[0] === RIGHT) {
							x = anchorX - w;
						}
						let y = anchorY;
						if (transformOrigin[1] === MIDDLE) {
							y = anchorY - h / 2;
						} else if (transformOrigin[1] === BOTTOM) {
							y = anchorY - h;
						}
						const prospectBounds = new DOMRect(x, y, w, h);
						if (
							rectContains(constraintsBounds, prospectBounds) &&
							(constraints.preventOverlap !== true ||
								!rectOverlaps(
									anchorParentBounds,
									prospectBounds,
								))
						) {
							console.log(
								constraintsBounds,
								prospectBounds,
								rectIntersection(
									constraintsBounds,
									prospectBounds,
								),
							);
							hAnchorOrigin = anchorOrigin[0];
							vAnchorOrigin = anchorOrigin[1];
							hTransformOrigin = transformOrigin[0];
							vTransformOrigin = transformOrigin[1];
							break;
						}
					}
				}

				setCurrentAnchorOrigin([hAnchorOrigin!, vAnchorOrigin!]);
				setCurrentTransformOrigin([
					hTransformOrigin!,
					vTransformOrigin!,
				]);

				anchorRef.current.style.position = "absolute";

				if (hAnchorOrigin === LEFT) {
					anchorRef.current.style.left = "0";
					anchorRef.current.style.right = "auto";
				} else if (hAnchorOrigin === CENTER) {
					anchorRef.current.style.left = "50%";
					anchorRef.current.style.right = "auto";
				} else {
					anchorRef.current.style.left = "auto";
					anchorRef.current.style.right = "0";
				}

				if (vAnchorOrigin === TOP) {
					anchorRef.current.style.top = "0";
					anchorRef.current.style.bottom = "auto";
				} else if (vAnchorOrigin === MIDDLE) {
					anchorRef.current.style.top = "50%";
					anchorRef.current.style.bottom = "auto";
				} else {
					anchorRef.current.style.top = "auto";
					anchorRef.current.style.bottom = "0";
				}

				transformRef.current.style.position = "absolute";
				const transform = ["0", "0"];

				if (hTransformOrigin === LEFT) {
					transformRef.current.style.left = "0";
					transformRef.current.style.right = "auto";
				} else if (hTransformOrigin === CENTER) {
					transformRef.current.style.left = "50%";
					transformRef.current.style.right = "auto";
					transform[0] = "-50%";
				} else {
					transformRef.current.style.left = "auto";
					transformRef.current.style.right = "0";
				}

				if (vTransformOrigin === TOP) {
					transformRef.current.style.top = "0";
					transformRef.current.style.bottom = "auto";
				} else if (vTransformOrigin === MIDDLE) {
					transformRef.current.style.top = "0";
					transformRef.current.style.bottom = "auto";
					transform[1] = "-50%";
				} else {
					transformRef.current.style.top = "auto";
					transformRef.current.style.bottom = "0";
				}
				transformRef.current.style.transform = `translate(${transform.join(
					", ",
				)})`;
			}
		},
		[
			anchorOrigin,
			transformOrigin,
			constraints?.element,
			constraints?.preventOverlap,
			constraints?.origins,
			anchorRef.current,
			transformRef.current,
		],
	);

	useEffect(() => {
		window.addEventListener("resize", recalcTransformPosition);
		return () => {
			window.removeEventListener("resize", recalcTransformPosition);
		};
	}, [recalcTransformPosition]);

	useLayoutEffect(() => {
		recalcTransformPosition();
	}, [recalcTransformPosition]);

	const context = useMemo<AnchorData>(
		() => ({
			transformRef,
			anchorOrigin: currentAnchorOrigin,
			transformOrigin: currentTransformOrigin,
		}),
		[currentAnchorOrigin, currentTransformOrigin],
	);

	return (
		<div ref={anchorRef} className={className}>
			{children(context)}
		</div>
	);
}
