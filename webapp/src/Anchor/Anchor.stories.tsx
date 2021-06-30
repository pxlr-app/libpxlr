import React, { useState } from "react";
import { Story, Meta } from "@storybook/react";
import { HorizontalAlign, Anchor, VerticalAlign, Alignement } from "./Anchor";

const LEFT = HorizontalAlign.LEFT;
const CENTER = HorizontalAlign.CENTER;
const RIGHT = HorizontalAlign.RIGHT;
const TOP = VerticalAlign.TOP;
const MIDDLE = VerticalAlign.MIDDLE;
const BOTTOM = VerticalAlign.BOTTOM;

export default {
	title: "Layout/Anchor",
	component: Anchor,
	argTypes: {
		preventOverlap: {
			description: "Prevent overlap",
			defaultValue: false,
			control: { type: "boolean" },
		},
		parentHorizontal: {
			description: "Parent horizontal",
			defaultValue: "Left",
			control: { type: "select" },
			options: ["Left", "Center", "Right"],
		},
		parentVertical: {
			description: "Parent vertical",
			defaultValue: "Top",
			control: { type: "select" },
			options: ["Top", "Middle", "Bottom"],
		},
		anchorHorizontal: {
			description: "Anchor horizontal",
			defaultValue: "Left",
			control: { type: "inline-check" },
			options: ["Left", "Center", "Right"],
		},
		anchorVertical: {
			description: "Anchor vertical",
			defaultValue: "Top",
			control: { type: "inline-check" },
			options: ["Top", "Middle", "Bottom"],
		},
		transformHorizontal: {
			description: "Transform horizontal",
			defaultValue: "Left",
			control: { type: "inline-check" },
			options: ["Left", "Center", "Right"],
		},
		transformVertical: {
			description: "Transform vertical",
			defaultValue: "Top",
			control: { type: "inline-check" },
			options: ["Top", "Middle", "Bottom"],
		},
	},
} as Meta;

function mapHorizontal(value: string) {
	switch (value) {
		case "Center":
			return CENTER;
		case "Right":
			return RIGHT;
	}
	return LEFT;
}

function mapVertical(value: string) {
	switch (value) {
		case "Middle":
			return MIDDLE;
		case "Bottom":
			return BOTTOM;
	}
	return TOP;
}

const Template = ({
	constraintElement,
	preventOverlap,
	anchorOrigin,
	transformOrigin,
}: {
	constraintElement?: HTMLElement;
	preventOverlap: boolean;
	anchorOrigin: Alignement;
	transformOrigin: Alignement;
}) => (
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
			constraintElement={constraintElement}
			preventOverlap={preventOverlap}
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

export const Default: Story<{
	preventOverlap: boolean;
	anchorHorizontal: string[];
	anchorVertical: string[];
	transformHorizontal: string[];
	transformVertical: string[];
}> = (args) => {
	const anchorOrigin = {
		horizontal: args.anchorHorizontal.map((s) => mapHorizontal(s)),
		vertical: args.anchorVertical.map((s) => mapVertical(s)),
	};
	const transformOrigin = {
		horizontal: args.transformHorizontal.map((s) => mapHorizontal(s)),
		vertical: args.transformVertical.map((s) => mapVertical(s)),
	};

	return (
		<Template
			preventOverlap={args.preventOverlap}
			anchorOrigin={anchorOrigin}
			transformOrigin={transformOrigin}
		/>
	);
};
Default.args = {
	preventOverlap: false,
	anchorHorizontal: ["Right"],
	anchorVertical: ["Top"],
	transformHorizontal: ["Left"],
	transformVertical: ["Top"],
};

export const Constrained: Story<{
	preventOverlap: boolean;
	parentHorizontal: string;
	parentVertical: string;
	anchorHorizontal: string[];
	anchorVertical: string[];
	transformHorizontal: string[];
	transformVertical: string[];
}> = (args) => {
	const anchorOrigin = {
		horizontal: args.anchorHorizontal.map((s) => mapHorizontal(s)),
		vertical: args.anchorVertical.map((s) => mapVertical(s)),
	};
	const transformOrigin = {
		horizontal: args.transformHorizontal.map((s) => mapHorizontal(s)),
		vertical: args.transformVertical.map((s) => mapVertical(s)),
	};
	const [constraintElement, setConstraint] = useState<HTMLElement>();

	const halign = args.parentHorizontal === "Right" ? "right" : "left";
	const hvalue = args.parentHorizontal === "Center" ? "-50%" : "0";
	const valign = args.parentVertical === "Bottom" ? "bottom" : "top";
	const vvalue = args.parentVertical === "Middle" ? "-50%" : "0";

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
					[halign]: args.parentHorizontal === "Center" ? "50%" : "0",
					[valign]: args.parentVertical === "Middle" ? "50%" : "0",
					transform: `translate(${hvalue}, ${vvalue})`,
				}}
			>
				<Template
					constraintElement={constraintElement}
					preventOverlap={args.preventOverlap}
					anchorOrigin={anchorOrigin}
					transformOrigin={transformOrigin}
				/>
			</div>
		</div>
	);
};
Constrained.args = {
	preventOverlap: true,
	parentHorizontal: "Left",
	parentVertical: "Top",
	anchorHorizontal: ["Left", "Right"],
	anchorVertical: ["Top"],
	transformHorizontal: ["Left", "Right"],
	transformVertical: ["Top"],
};
