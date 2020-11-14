import styled, { css } from 'styled-components';

export interface HandleSplitProps {
	axe: 'horizontal' | 'vertical',
	visible?: boolean,
	disabled?: boolean,
}

export interface HandleSubdivideProps {
	corner: 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right',
	visible?: boolean,
	disabled?: boolean,
}

export interface ViewContainerProps {
	axe: 'horizontal' | 'vertical',
}

export interface ViewProps {
	axe: 'horizontal' | 'vertical',
	resizable: boolean
}

export const Splitview = styled.div`
	position: relative;
	width: 100%;
	height: 100%;
	display: flex;
	--splithandle-size: 10px;
	--subdividehandle-size: 10px;
`;

export const HandleContainer = styled.div`
	position: absolute;
	width: 100%;
	height: 100%;
	pointer-events: none;
`;

export const HandleSplit = styled.div<HandleSplitProps>`
	position: absolute;
	z-index: 35;
	touch-action: none;
	pointer-events: ${props => (props.disabled ? 'none !important' : 'auto')};
	cursor: ${props => (props.disabled ? 'default !important' : (props.axe == 'horizontal' ? 'e-resize' : 'n-resize'))};
	${props => props.axe === 'horizontal' && css<HandleSplitProps>`
		top: 0;
		height: 100%;
		width: var(--splithandle-size);
		transform: translateX(calc(var(--splithandle-size) / -2));
	`}
	${props => props.axe === 'vertical' && css<HandleSplitProps>`
		left: 0;
		width: 100%;
		height: var(--splithandle-size);
		transform: translateY(calc(var(--splithandle-size) / -2));
	`}
`;

export const ViewContainer = styled.div<ViewContainerProps>`
	position: relative;
	white-space: nowrap;
	display: flex;
	flex: 1;
	flex-direction: ${props => props.axe === 'horizontal' ? 'row' : 'column'};
`;

export const View = styled.div<ViewProps>`
	display: flex;
	flex: ${props => props.resizable ? 'unset' : '1'};
	position: relative;
	overflow: auto;
	white-space: normal;
	${props => props.axe === 'horizontal' && css<ViewProps>`
		height: 100%;
		border-right: 1px solid black;

		&:last-child {
			border-right: 0;
		}
	`}
	${props => props.axe === 'vertical' && css<ViewProps>`
		width: 100%;
		border-bottom: 1px solid black;

		&:last-child {
			border-bottom: 0;
		}
	`}
`;

export const SubdivideContainer = styled.div`
	position: absolute;
	width: 100%;
	height: 100%;
	pointer-events: none;
`;

export const HandleSubdivide = styled.div<HandleSubdivideProps>`
	position: absolute;
	z-index: 35;
	touch-action: none;
	pointer-events: ${props => (props.disabled ? 'none !important' : 'auto')};
	width: var(--subdividehandle-size);
	height: var(--subdividehandle-size);
	
	&::before {
		content: ' ';
		font-size: 0;
		line-height: 0;
		display: block;
		width: 0;
		height: 0;
		border-style: solid;
		border-width: 0;
		border-color: transparent;
		transition: 0.3s;
		transform: scale(0.0);
	}
	&:hover::before {
		transform: scale(1.0);
	}
	${props => props.corner === 'top-left' && css`
		&, &::before {
			top: 0;
			left: 0;
			cursor: nw-resize;
		}
	`}
	${props => props.corner === 'top-right' && css`
		&, &::before {
			top: 0;
			right: 0;
			cursor: ne-resize;
		}
	`}
	${props => props.corner === 'bottom-left' && css`
		&, &::before {
			bottom: 0;
			left: 0;
			cursor: sw-resize;
		}
	`}
	${props => props.corner === 'bottom-right' && css`
		&, &::before {
			bottom: 0;
			right: 0;
			cursor: se-resize;
		}
	`}
	${props => props.corner === 'top-left' && css`
		&::before {
			border-width: var(--subdividehandle-size) var(--subdividehandle-size) 0 0;
			border-color: black transparent transparent transparent;
			transform-origin: 0 0;
		}
	`}
	${props => props.corner === 'top-right' && css`
		&::before {
			border-width: 0 var(--subdividehandle-size) var(--subdividehandle-size) 0;
			border-color:  transparent black transparent transparent;
			transform-origin: 100% 0;
		}
	`}
	${props => props.corner === 'bottom-left' && css`
		&::before {
			border-width: var(--subdividehandle-size) 0 0 var(--subdividehandle-size);
			border-color:  transparent transparent transparent black;
			transform-origin: 0 100%;
		}
	`}
	${props => props.corner === 'bottom-right' && css`
		&::before {
			border-width: 0 0 var(--subdividehandle-size) var(--subdividehandle-size);
			border-color:  transparent transparent black transparent;
			transform-origin: 100% 100%;
		}
	`}
`;