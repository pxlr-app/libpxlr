import styled, { css } from 'styled-components';

export interface HandleSplitProps {
	axe: 'horizontal' | 'vertical',
	visible?: boolean,
	disabled?: boolean,
	offset: number,
}

export interface ViewContainerProps {
	axe: 'horizontal' | 'vertical',
}

export interface ViewProps {
	axe: 'horizontal' | 'vertical',
	visible?: boolean,
	disabled?: boolean,
	width?: number,
}

export const Splitview = styled.div`
	position: relative;
	width: 100%;
	height: 100%;
	display: flex;
	--splithandle-size: 3px;
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
		left: ${props => props.offset}%;
		transform: translateX(calc(var(--splithandle-size) / -2));
	`}
	${props => props.axe === 'vertical' && css<HandleSplitProps>`
		left: 0;
		width: 100%;
		height: var(--splithandle-size);
		top: ${props => props.offset}%;
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
	overflow: auto;
	white-space: normal;
	${props => props.axe === 'horizontal' && css<ViewProps>`
		${props => typeof props.width !== 'undefined' ? `width: ${props.width}%` : `flex: 1`};
		height: 100%;
		border-right: 1px solid black;

		&:last-child {
			border-right: 0;
		}
	`}
	${props => props.axe === 'vertical' && css<ViewProps>`
		${props => typeof props.width !== 'undefined' ? `height: ${props.width}%` : `flex: 1`};
		width: 100%;
		border-bottom: 1px solid black;

		&:last-child {
			border-bottom: 0;
		}
	`}
`;