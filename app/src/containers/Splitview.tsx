import React, { useState, useEffect, useRef } from 'react';
import * as styled from './Splitview.styled';

export interface SplitviewProps {
	defaultView: React.ReactElement,
	axe?: 'horizontal' | 'vertical',
}

interface SplitviewState {
	axe: 'horizontal' | 'vertical',
	left: React.ReactElement,
	right?: React.ReactElement,
	main: 'left' | 'right',
	dragging: boolean,
	split: number,
	corner?: 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right',
	originalMain?: React.ReactElement,
}

export default function Splitview(props: SplitviewProps) {
	const dividerRef = useRef<HTMLDivElement>(null);
	const viewRef = useRef<HTMLDivElement>(null);
	const subdivideRef = useRef<HTMLDivElement>(null);

	const [state, setState] = useState<SplitviewState>(() => ({
		axe: props.axe ?? 'horizontal',
		left: props.defaultView,
		// right: props.defaultView,
		main: 'left',
		dragging: false,
		split: 30.,
	}));

	const axe = state.axe;
	const X = axe === 'horizontal' ? 'clientX' : 'clientY';
	const L = axe === 'horizontal' ? 'left' : 'top';
	const W = axe === 'horizontal' ? 'width' : 'height';

	useEffect(() => {
		if (!state.dragging) {
			document.body.style.cursor = 'auto';
			return;
		}

		const draggingCorner = state.corner !== undefined;

		function splitPosition(e: PointerEvent) {
			const viewportPos = e[X];
			const target = dividerRef.current;
			if (target) {
				const bounds = (target?.parentElement ?? target).getBoundingClientRect()!;
				const relativePos = viewportPos - bounds[L];
				const percentPos = Math.max(0, Math.min(1, relativePos / bounds[W]));
				return [percentPos, relativePos, bounds[W]];
			}
			return [undefined, undefined];
		}

		const onMove = (e: PointerEvent) => {
			if (draggingCorner) {
				const target = subdivideRef.current;
				if (target) {
					const bounds = (target?.parentElement ?? target).getBoundingClientRect()!;
					const x = e.clientX - bounds.left;
					const y = e.clientY - bounds.top;
					const origX = state.corner === 'top-left' || state.corner === 'bottom-left' ? bounds.left : bounds.right;
					const origY = state.corner === 'top-left' || state.corner === 'top-right' ? bounds.top : bounds.bottom;
					const dirX = x - origX;
					const dirY = y - origY;
					
					const isHorizontal = Math.abs(dirX) > Math.abs(dirY);
					document.body.style.cursor = isHorizontal ? 'ew-resize' : 'ns-resize';

					let left: SplitviewState['right'] = undefined;
					let right: SplitviewState['right'] = undefined;
					let original: SplitviewState['left'];

					if (
						(state.corner === 'top-left') ||
						(state.corner === 'top-right' && !isHorizontal) ||
						(state.corner === 'bottom-left' && isHorizontal)
					 ) {
						left = undefined;
						right = state.originalMain ?? state.left;
						original = right;
					} else {
						left = state.originalMain ?? state.left;
						right = undefined;
						original = left;
					}

					// // console.log(isHorizontal ? 'horizontal' : 'vertical', left, right);
					// setState({
					// 	...state,
					// 	axe: isHorizontal ? 'horizontal' : 'vertical',
					// 	originalMain: state.originalMain ?? original,
					// 	left: left!,
					// 	right,
					// });
				} else {
					return;
				}
			} else {
				document.body.style.cursor = axe === 'horizontal' ? 'ew-resize' : 'ns-resize';
			}

			const [percent, pos, width] = splitPosition(e);
			if (percent !== undefined) {
				if (dividerRef.current?.style) {
					dividerRef.current.style[L] = `${(percent * 100).toFixed(4)}%`;
				}

				if (viewRef.current?.style) {
					viewRef.current.style[W] = state.main == 'left' ? `${(percent * 100).toFixed(4)}%` : `${(100 - percent * 100).toFixed(4)}%`;
				}

				// TODO collapse on near 0% or 100%
			}
		};

		const onLeave = (e: PointerEvent) => {
			if (draggingCorner) {
				setState({
					...state,
					dragging: false,
					corner: undefined,
				});
			} else {
				const [percent] = splitPosition(e);
				if (percent !== undefined) {
					setState({
						...state,
						dragging: false,
						corner: undefined,
						split: percent * 100
					});
				}
			}
		};
	
		document.addEventListener('pointerup', onLeave);
		document.addEventListener('pointermove', onMove);
		return () => {
			document.removeEventListener('pointerup', onLeave);
			document.removeEventListener('pointermove', onMove);
		}
		
	}, [state]);

	const onSplitDown = (e: React.PointerEvent<HTMLDivElement>) => {
		setState({ ...state, dragging: true });
	};

	const onSubdivideDown = (corner: Exclude<SplitviewState['corner'], undefined>) => (e: React.PointerEvent<HTMLDivElement>) => {
		setState({ ...state, dragging: true, corner });
	};

	return (
		<styled.Splitview>
			{state.left && state.right && <styled.HandleContainer>
				<styled.HandleSplit ref={dividerRef} axe={axe} style={{[L]: `${state.split.toFixed(4)}%`}} onPointerDown={onSplitDown} />
			</styled.HandleContainer>}
			{!state.left !== !state.right && <styled.SubdivideContainer ref={subdivideRef}>
				<styled.HandleSubdivide corner="top-left" onPointerDown={onSubdivideDown('top-left')} />
				<styled.HandleSubdivide corner="top-right" onPointerDown={onSubdivideDown('top-right')} />
				<styled.HandleSubdivide corner="bottom-left" onPointerDown={onSubdivideDown('bottom-left')} />
				<styled.HandleSubdivide corner="bottom-right" onPointerDown={onSubdivideDown('bottom-right')} />
			</styled.SubdivideContainer>}
			<styled.ViewContainer axe={axe}>
				{state.left && <styled.View ref={state.main == 'left' ? viewRef : undefined} axe={axe} resizable={state.main == 'left'} style={{[W]: state.main == 'left' && state.right ? `${state.split.toFixed(4)}%` : 'auto'}}>
					{state.left}
				</styled.View>}
				{state.right && <styled.View ref={state.main == 'right' ? viewRef : undefined} axe={axe} resizable={state.main == 'right'} style={{[W]: state.main == 'right' && state.left ? `${(100 - state.split).toFixed(4)}%` : 'auto'}}>
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
