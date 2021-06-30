import React, { useEffect, useMemo, useRef } from "react";

export enum VerticalAlign {
	TOP,
	MIDDLE,
	BOTTOM,
}

export enum HorizontalAlign {
	LEFT,
	CENTER,
	RIGHT,
}

export type Alignement = {
	horizontal: HorizontalAlign[];
	vertical: VerticalAlign[];
};

export type AnchorData = {
	transformRef: React.MutableRefObject<HTMLElement | undefined>;
};

export type AnchorContainerProps = {
	constraintElement?: HTMLElement | (() => HTMLElement);
	preventOverlap?: boolean;
	anchorOrigin: Alignement;
	transformOrigin: Alignement;
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

export function Anchor({
	constraintElement,
	preventOverlap,
	anchorOrigin,
	transformOrigin,
	children,
}: AnchorContainerProps) {
	const anchorRef = useRef<HTMLDivElement>(null);
	const transformRef = useRef<HTMLElement>();

	const context = useMemo<AnchorData>(
		() => ({
			transformRef,
		}),
		[],
	);

	const recalcTransformPosition = useMemo(
		() => () => {
			if (anchorRef.current && transformRef.current) {
				const transformBounds = transformRef.current.getBoundingClientRect();
				const anchorParentBounds = anchorRef.current.parentElement!.getBoundingClientRect();
				const constraints = getOrRetrieve(
					constraintElement ?? document.body,
				);
				const constraintsBounds = constraints.getBoundingClientRect();

				let hAnchorOrigin = HorizontalAlign.CENTER;
				let vAnchorOrigin = VerticalAlign.MIDDLE;
				let hTransformOrigin = HorizontalAlign.CENTER;
				let vTransformOrigin = VerticalAlign.MIDDLE;

				const anchorHorizontals = anchorOrigin.horizontal;
				const anchorVerticals = anchorOrigin.vertical;
				const transformHorizontals = transformOrigin.horizontal;
				const transformVerticals = transformOrigin.vertical;

				const w = transformBounds.width;
				const h = transformBounds.height;

				auto: for (const hanchor of anchorHorizontals) {
					let anchorX = 0;
					if (hanchor === HorizontalAlign.LEFT) {
						anchorX = anchorParentBounds.left;
					}
					if (hanchor === HorizontalAlign.CENTER) {
						anchorX =
							anchorParentBounds.left +
							anchorParentBounds.width / 2;
					}
					if (hanchor === HorizontalAlign.RIGHT) {
						anchorX = anchorParentBounds.right;
					}
					for (const vanchor of anchorVerticals) {
						let anchorY = 0;
						if (vanchor === VerticalAlign.TOP) {
							anchorY = anchorParentBounds.top;
						}
						if (vanchor === VerticalAlign.MIDDLE) {
							anchorY =
								anchorParentBounds.top +
								anchorParentBounds.height / 2;
						}
						if (vanchor === VerticalAlign.BOTTOM) {
							anchorY = anchorParentBounds.bottom;
						}
						for (const htrans of transformHorizontals) {
							let x = 0;
							if (htrans === HorizontalAlign.LEFT) {
								x = anchorX;
							}
							if (htrans === HorizontalAlign.CENTER) {
								x = anchorX - w / 2;
							}
							if (htrans === HorizontalAlign.RIGHT) {
								x = anchorX - w;
							}
							for (const vtrans of transformVerticals) {
								let y = 0;
								if (vtrans === VerticalAlign.TOP) {
									y = anchorY;
								}
								if (vtrans === VerticalAlign.MIDDLE) {
									y = anchorY - h / 2;
								}
								if (vtrans === VerticalAlign.BOTTOM) {
									y = anchorY - h;
								}
								const prospectBounds = new DOMRect(x, y, w, h);
								if (
									rectContains(
										constraintsBounds,
										prospectBounds,
									) &&
									(!preventOverlap ||
										!rectContains(
											anchorParentBounds,
											prospectBounds,
										))
								) {
									hAnchorOrigin = hanchor;
									vAnchorOrigin = vanchor;
									hTransformOrigin = htrans;
									vTransformOrigin = vtrans;
									break auto;
								}
							}
						}
					}
				}

				anchorRef.current.style.position = "absolute";

				if (hAnchorOrigin === HorizontalAlign.LEFT) {
					anchorRef.current.style.left = "0";
					anchorRef.current.style.right = "auto";
				} else if (hAnchorOrigin === HorizontalAlign.CENTER) {
					anchorRef.current.style.left = "50%";
					anchorRef.current.style.right = "auto";
				} else {
					anchorRef.current.style.left = "auto";
					anchorRef.current.style.right = "0";
				}

				if (vAnchorOrigin === VerticalAlign.TOP) {
					anchorRef.current.style.top = "0";
					anchorRef.current.style.bottom = "auto";
				} else if (vAnchorOrigin === VerticalAlign.MIDDLE) {
					anchorRef.current.style.top = "50%";
					anchorRef.current.style.bottom = "auto";
				} else {
					anchorRef.current.style.top = "auto";
					anchorRef.current.style.bottom = "0";
				}

				transformRef.current.style.position = "absolute";
				const transform = ["0", "0"];

				if (hTransformOrigin === HorizontalAlign.LEFT) {
					transformRef.current.style.left = "0";
					transformRef.current.style.right = "auto";
				} else if (hTransformOrigin === HorizontalAlign.CENTER) {
					transformRef.current.style.left = "50%";
					transformRef.current.style.right = "auto";
					transform[0] = "-50%";
				} else {
					transformRef.current.style.left = "auto";
					transformRef.current.style.right = "0";
				}

				if (vTransformOrigin === VerticalAlign.TOP) {
					transformRef.current.style.top = "0";
					transformRef.current.style.bottom = "auto";
				} else if (vTransformOrigin === VerticalAlign.MIDDLE) {
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
			anchorRef.current,
			transformRef.current,
			constraintElement,
		],
	);

	useEffect(() => {
		window.addEventListener("resize", recalcTransformPosition);
		recalcTransformPosition();
		return () => {
			window.removeEventListener("resize", recalcTransformPosition);
		};
	}, [recalcTransformPosition]);

	return <div ref={anchorRef}>{children(context)}</div>;
}
