import React, { useState, useEffect, useRef } from 'react';
import * as styled from './Splitview.styled';

export interface SplitviewProps {
	defaultView: React.ReactElement,
	axe?: 'horizontal' | 'vertical',
}

interface SplitviewState {
	left?: React.ReactElement,
	right?: React.ReactElement,
	main: 'left' | 'right',
	dragging: boolean,
	split: number,
}

export default function Splitview(props: SplitviewProps) {
	const axe = props.axe ?? 'horizontal';
	const clientX = axe === 'horizontal' ? 'clientX' : 'clientY';
	const left = axe === 'horizontal' ? 'left' : 'top';
	const width = axe === 'horizontal' ? 'width' : 'height';

	const dividerRef = useRef<HTMLDivElement>(null);
	const viewRef = useRef<HTMLDivElement>(null);

	const [state, setState] = useState<SplitviewState>(() => ({
		left: props.defaultView,
		right: props.defaultView,
		main: 'left',
		dragging: false,
		split: 30.
	}));

	useEffect(() => {
		if (!state.dragging) {
			document.body.style.cursor = 'auto';
			return;
		}

		document.body.style.cursor = axe === 'horizontal' ? 'ew-resize' : 'ns-resize';

		function clientXasPercent(e: PointerEvent) {
			const viewportPos = e[clientX];
			const target = dividerRef.current;
			if (target) {
				const bounds = (target?.parentElement ?? target).getBoundingClientRect()!;
				const relativePos = viewportPos - bounds[left];
				const percentPos = Math.max(0, Math.min(1, relativePos / bounds[width]));
				return percentPos;
			}
			return undefined;
		}

		const onMove = (e: PointerEvent) => {
			if (state.dragging) {
				const percentPos = clientXasPercent(e);
				if (percentPos !== undefined) {
					if (dividerRef.current?.style) {
						dividerRef.current.style[left] = `${(percentPos * 100).toFixed(4)}%`;
					}

					if (viewRef.current?.style) {
						viewRef.current.style[width] = state.main == 'left' ? `${(percentPos * 100).toFixed(4)}%` : `${(100 - percentPos * 100).toFixed(4)}%`;
					}
				}
			}
		}
		const onLeave = (e: PointerEvent) => {
			const percentPos = clientXasPercent(e);
			if (percentPos !== undefined) {
				setState({
					...state,
					dragging: false,
					split: percentPos * 100,
				});
			}
		}

		document.addEventListener('pointerup', onLeave);
		document.addEventListener('pointermove', onMove);

		return () => {
			document.removeEventListener('pointerup', onLeave);
			document.removeEventListener('pointermove', onMove);
		}
	}, [state]);

	const onPointerDown = (e: React.PointerEvent<HTMLDivElement>) => {
		setState({ ...state, dragging: true });
	};

	return (
		<styled.Splitview>
			<styled.HandleContainer>
				{(state.left && state.right) && <styled.HandleSplit ref={dividerRef} axe={axe} style={{[left]: `${state.split.toFixed(4)}%`}} onPointerDown={onPointerDown} />}
			</styled.HandleContainer>
			<styled.ViewContainer axe={axe}>
				{state.left && <styled.View ref={state.main == 'left' ? viewRef : undefined} axe={axe} resizable={state.main == 'left'} style={{[width]: state.main == 'left' && state.right ? `${state.split.toFixed(4)}%` : 'auto'}}>
					{state.left}
				</styled.View>}
				{state.right && <styled.View ref={state.main == 'right' ? viewRef : undefined} axe={axe} resizable={state.main == 'right'} style={{[width]: state.main == 'right' && state.left ? `${(100 - state.split).toFixed(4)}%` : 'auto'}}>
					{state.right}
				</styled.View>}
			</styled.ViewContainer>
		</styled.Splitview>
	);
}


// const getComputedStyle = document.defaultView!.getComputedStyle;
// function getComputedSize(
// 	element: HTMLElement,
// 	prop: 'width' | 'min-width' | 'max-width' | 'height' | 'min-height' | 'max-height'
// ) {
// 	const styles = getComputedStyle(element);
// 	// eslint-disable-next-line @typescript-eslint/no-explicit-any
// 	const value = styles[prop as any] as string;
// 	const match = value.match(/^(\d+)(px|em|rem|%|vw|vh)$/i);
// 	if (!match) {
// 		return undefined;
// 	}
// 	const [, size, unit] = match;
// 	switch (unit.toLowerCase()) {
// 		case 'px':
// 			return parseFloat(size);
// 		case 'em':
// 			return parseFloat(size) * parseFloat(getComputedStyle(element.parentElement!).fontSize);
// 		case 'rem':
// 			return parseFloat(size) * parseFloat(getComputedStyle(document.body).fontSize);
// 		case '%':
// 			return (
// 				(parseFloat(size) / 100) *
// 				element.parentElement![prop.substr(-5) === 'width' ? 'offsetWidth' : 'offsetHeight']
// 			);
// 		case 'vw':
// 			return (parseFloat(size) / 100) * window.innerWidth;
// 		case 'vh':
// 			return (parseFloat(size) / 100) * window.innerHeight;
// 	}
// 	return undefined;
// };
