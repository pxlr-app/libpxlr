import styled from 'styled-components';

export const Divider = styled.span`
	box-sizing: border-box;
	background: #000;
	opacity: 0.5;
	z-index: 1;
	background-clip: padding-box;
`;

export const Body = styled.div`
	position: relative;
	display: flex;
	flex-grow: 1;
	flex-shrink: 1;
	overflow: auto;
`;

export const Triangle = styled.div`
	width: 0;
	height: 0;
	border-style: solid;
	border-width: 0;
	border-color: transparent;
	transition: 0.3s;
`;

export const Handle = styled.div<{ valign: 'top' | 'bottom', halign: 'left' | 'right' }>`
	position: absolute;
	${props => (props.valign === 'top' ? 'top' : 'bottom')}: 0px;
	${props => (props.halign === 'left' ? 'left' : 'right')}: 0px;

	width: 10px;
	height: 10px;

	cursor: ${props => (props.valign === 'top' ? 'n' : 's')}${props => (props.halign === 'left' ? 'w' : 'e')}-resize;

	& > ${Triangle} {
		position: absolute;
		${props => (props.valign === 'top' ? 'top' : 'bottom')}: 0px;
		${props => (props.halign === 'left' ? 'left' : 'right')}: 0px;
	}

	&:hover > ${Triangle} {
		border-width: ${props => (props.halign === 'left' ? '10px' : '0')} ${props => (props.valign === 'top' ? '10px' : '0')} ${props => (props.halign === 'left' ? '0' : '10px')} ${props => (props.valign === 'top' ? '0' : '10px')};
		border-color: ${props => (props.valign === 'top' && props.halign == 'left' ? '#007bff' : 'transparent')} ${props => (props.valign === 'top' && props.halign == 'right' ? '#007bff' : 'transparent')} ${props => (props.valign === 'bottom' && props.halign == 'right' ? '#007bff' : 'transparent')} ${props => (props.valign === 'bottom' && props.halign == 'left' ? '#007bff' : 'transparent')};
	}
`;

export const Container = styled.div<{ axe: 'horizontal' | 'vertical' }>`
	position: relative;
	display: flex;
	flex: 1 1 0%;
	flex-direction: ${props => (props.axe === 'horizontal' ? 'row' : 'column')};

	& > ${Divider} {
		${props => (props.axe === 'horizontal' ? 'width' : 'height')}: 1px;
		cursor: ${props => (props.axe === 'horizontal' ? 'ew-resize' : 'ns-resize')};
		margin: ${props => (props.axe === 'horizontal' ? '0 -1px' : '-1px 0')};
		${props => (props.axe === 'horizontal' ? 'border-left' : 'border-top')}: 1px solid transparent;
		${props => (props.axe === 'horizontal' ? 'border-right' : 'border-bottom')}: 1px solid transparent;
	}
`;