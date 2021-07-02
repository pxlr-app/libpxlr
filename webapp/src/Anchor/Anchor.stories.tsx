import React, { useState } from "react";
import { Story, Meta } from "@storybook/react";
import {
	HorizontalAlign,
	Anchor,
	VerticalAlign,
	Alignement,
	AnchorContainerProps,
	Constraints,
} from "./Anchor";
import { faArrowAltSquareRight } from "@fortawesome/pro-light-svg-icons";

const LEFT = HorizontalAlign.LEFT;
const CENTER = HorizontalAlign.CENTER;
const RIGHT = HorizontalAlign.RIGHT;
const TOP = VerticalAlign.TOP;
const MIDDLE = VerticalAlign.MIDDLE;
const BOTTOM = VerticalAlign.BOTTOM;

export default {
	title: "Layout/Anchor",
	component: Anchor,
	parameters: { docs: { source: { type: "code" } } },
	argTypes: {
		alignment: {
			description: "Alignment",
			defaultValue: "TopLeft",
			control: { type: "select" },
			options: [
				"TopLeft",
				"TopCenter",
				"TopRight",
				"MiddleLeft",
				"MiddleCenter",
				"MiddleRight",
				"BottomLeft",
				"BottomCenter",
				"BottomRight",
			],
		},
		anchorOrigin: {
			description: "Anchor Origin",
			defaultValue: "TopLeft",
			control: { type: "select" },
			options: [
				"TopLeft",
				"TopCenter",
				"TopRight",
				"MiddleLeft",
				"MiddleCenter",
				"MiddleRight",
				"BottomLeft",
				"BottomCenter",
				"BottomRight",
			],
		},
		transformOrigin: {
			description: "Transform Origin",
			defaultValue: "TopLeft",
			control: { type: "select" },
			options: [
				"TopLeft",
				"TopCenter",
				"TopRight",
				"MiddleLeft",
				"MiddleCenter",
				"MiddleRight",
				"BottomLeft",
				"BottomCenter",
				"BottomRight",
			],
		},
		constraints: {
			table: {
				disable: true,
			},
		},
	},
} as Meta;

function mapStringToAlignment(value: string): Alignement {
	switch (value) {
		case "TopLeft":
			return [LEFT, TOP];
		case "TopCenter":
			return [CENTER, TOP];
		case "TopRight":
			return [RIGHT, TOP];
		case "MiddleLeft":
			return [LEFT, MIDDLE];
		case "MiddleCenter":
			return [CENTER, MIDDLE];
		case "MiddleRight":
			return [RIGHT, MIDDLE];
		case "BottomLeft":
			return [LEFT, BOTTOM];
		case "BottomCenter":
			return [CENTER, BOTTOM];
		case "BottomRight":
			return [RIGHT, BOTTOM];
		default:
			throw new Error(`Unknown alignment ${value}.`);
	}
}

export const Default: Story<{
	alignment: string;
	anchorOrigin: string;
	transformOrigin: string;
}> = (args) => {
	const anchorOrigin = mapStringToAlignment(args.anchorOrigin);
	const transformOrigin = mapStringToAlignment(args.transformOrigin);
	return (
		<div
			style={{
				display: "inline-block",
				position: "relative",
				background: "#c0c0c0",
				padding: "4px",
			}}
		>
			Parent
			<Anchor
				anchorOrigin={anchorOrigin}
				transformOrigin={transformOrigin}
			>
				{({ transformRef }) => (
					<div
						ref={transformRef as any}
						style={{
							display: "inline-block",
							position: "absolute",
							background: "#f0c0c0a0",
							padding: "4px",
						}}
					>
						Child
					</div>
				)}
			</Anchor>
		</div>
	);
};
Default.args = {
	alignment: "TopLeft",
	anchorOrigin: "TopRight",
	transformOrigin: "TopLeft",
};

export const Constrained: Story<{
	alignment: string;
	constraints: Constraints;
}> = (args) => {
	const alignment = mapStringToAlignment(args.alignment);
	const [constraintElement, setConstraint] = useState<HTMLElement>();

	const halign = alignment[0] === RIGHT ? "right" : "left";
	const hvalue = alignment[0] === CENTER ? "-50%" : "0";
	const valign = alignment[1] === BOTTOM ? "bottom" : "top";
	const vvalue = alignment[1] === MIDDLE ? "-50%" : "0";

	let constraints: Constraints | undefined;
	if (args.constraints) {
		constraints = { ...args.constraints, element: constraintElement };
	}

	return (
		<div
			ref={(element) => setConstraint(element!)}
			style={{
				display: "block",
				position: "relative",
				width: "100px",
				height: "100px",
				background: "rgba(0, 0, 0, 0.1)",
			}}
		>
			<div
				style={{
					display: "inline-block",
					position: "absolute",
					[halign]: alignment[0] === CENTER ? "50%" : "0",
					[valign]: alignment[1] === MIDDLE ? "50%" : "0",
					transform: `translate(${hvalue}, ${vvalue})`,
				}}
			>
				<div
					style={{
						display: "inline-block",
						position: "relative",
						background: "#c0c0c0",
						padding: "4px",
					}}
				>
					Parent
					<Anchor key={alignment.join("-")} constraints={constraints}>
						{({ transformRef }) => (
							<div
								ref={transformRef as any}
								style={{
									display: "inline-block",
									position: "absolute",
									background: "#f0c0c0a0",
									padding: "4px",
								}}
							>
								Child
							</div>
						)}
					</Anchor>
				</div>
			</div>
		</div>
	);
};
Constrained.args = {
	alignment: "TopLeft",
	constraints: {
		preventOverlap: true,
		origins: [
			{
				anchor: [RIGHT, TOP],
				transform: [LEFT, TOP],
			},
			{
				anchor: [LEFT, TOP],
				transform: [RIGHT, TOP],
			},
			{
				anchor: [RIGHT, BOTTOM],
				transform: [LEFT, BOTTOM],
			},
			{
				anchor: [LEFT, BOTTOM],
				transform: [RIGHT, BOTTOM],
			},
		],
	},
};
