import React, { useState, useEffect, useRef } from 'react';
import * as styled from './Splitview.styled';
import useAnimationFrame from '../helpers/useAnimationFrame';

export interface SplitviewProps {
	defaultView: React.ReactElement,
	axe?: 'horizontal' | 'vertical',
}

interface SplitviewState {
	axe: 'horizontal' | 'vertical',
	left: React.ReactElement,
	right?: React.ReactElement,
	main: 'left' | 'right',
	split: number,
	
}

interface SplitviewInternalState {
	originalMain?: React.ReactElement,
	dragging: boolean,
	corner?: 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right',
	pointerEvent?: PointerEvent,
	lastPointerEvent?: PointerEvent,
}

export default function Splitview(props: SplitviewProps) {
	const dividerRef = useRef<HTMLDivElement>(null);
	const leftViewRef = useRef<HTMLDivElement>(null);
	const rightViewRef = useRef<HTMLDivElement>(null);
	const subdivideRef = useRef<HTMLDivElement>(null);
	const internalState = useRef<SplitviewInternalState>({
		dragging: false,
	});
	const [state, setState] = useState<SplitviewState>(() => ({
		axe: props.axe ?? 'horizontal',
		left: props.defaultView,
		// right: props.defaultView,
		main: 'left',
		split: 30.,
	}));

	function getSplitMetric(e: PointerEvent, clientX: 'clientX' | 'clientY') {
		const pos = e[clientX];
		const target = dividerRef.current;
		if (target) {
			const bounds = (target?.parentElement ?? target).getBoundingClientRect()!;
			const relativePos = pos - bounds[L];
			const percentPos = Math.max(0, Math.min(1, relativePos / bounds[W]));
			return [percentPos, relativePos, bounds[W]];
		}
		return [undefined, undefined];
	}

	useEffect(() => {
		function onLeave(e: PointerEvent) {
			if (internalState.current.dragging) {
				internalState.current = { dragging: false };
				document.body.style.cursor = 'auto';

				const [percent] = getSplitMetric(e, state.axe === 'horizontal' ? 'clientX' : 'clientY');
				if (percent !== undefined) {
					setState({ ...state, split: percent * 100 });
				}
			}
		}

		function onMove(pointerEvent: PointerEvent) {
			if (internalState.current.dragging) {
				const { dragging, lastPointerEvent, corner } = internalState.current;

				// Skip this callback if position hasn't changed
				if (pointerEvent?.clientX === lastPointerEvent?.clientX && pointerEvent?.clientY === lastPointerEvent?.clientY) {
					return;
				}
				internalState.current.lastPointerEvent = pointerEvent;

				if (dragging && pointerEvent) {
					// Dragging corner?
					if (corner) {
						const subdivider = subdivideRef.current;
						if (subdivider) {
							const bounds = (subdivider?.parentElement ?? subdivider).getBoundingClientRect()!;
							const x = pointerEvent.clientX - bounds.left;
							const y = pointerEvent.clientY - bounds.top;
							const origX = corner === 'top-left' || corner === 'bottom-left' ? bounds.left : bounds.right;
							const origY = corner === 'top-left' || corner === 'top-right' ? bounds.top : bounds.bottom;
							const dirX = x - origX;
							const dirY = y - origY;
							
							const isHorizontal = Math.abs(dirX) > Math.abs(dirY);
							// document.body.style.cursor = isHorizontal ? 'ew-resize' : 'ns-resize';

							let left: SplitviewState['right'] = undefined;
							let right: SplitviewState['right'] = undefined;
							let main: SplitviewState['main'] = 'left';

							if (
								(corner === 'top-left') ||
								(corner === 'top-right' && !isHorizontal) ||
								(corner === 'bottom-left' && isHorizontal)
							) {
								left = props.defaultView;
								right = internalState.current.originalMain;
								main = 'right';
							} else {
								left = internalState.current.originalMain;
								right = props.defaultView;
							}

							const axe = isHorizontal ? 'horizontal' : 'vertical';

							// Flip splitview
							if (!state.left || !state.right || state.axe !== axe) {
								setState({
									...state,
									axe,
									main,
									left: left!,
									right,
								});
							}
						}
					}
					
					// Dragging split
					const [percent, pos, width] = getSplitMetric(pointerEvent, state.axe === 'horizontal' ? 'clientX' : 'clientY');
					if (percent !== undefined) {
						const viewRef = state.main === 'left' ? leftViewRef : rightViewRef;
						const otherViewRef = state.main === 'left' ? rightViewRef : leftViewRef;

						const L = state.axe === 'horizontal' ? 'left' : 'top';
						const T = state.axe === 'horizontal' ? 'top' : 'left';
						const W = state.axe === 'horizontal' ? 'width' : 'height';
						const H = state.axe === 'horizontal' ? 'height' : 'width';

						// console.log('dragging split', state.main, state.axe, percent, pos, width);
						if (dividerRef.current?.style) {
							dividerRef.current.style[L] = `${(percent * 100).toFixed(4)}%`;
							dividerRef.current.style[T] = 'auto';
						}

						if (viewRef.current?.style) {
							viewRef.current.style[W] = state.main == 'left' ? `${(percent * 100).toFixed(4)}%` : `${(100 - percent * 100).toFixed(4)}%`;
							viewRef.current.style[H] = 'auto';
						}

						if (otherViewRef.current) {
							otherViewRef.current.style[W] = 'auto';
							otherViewRef.current.style[H] = 'auto';
						}

						// TODO collapse on near 0% or 100%

						document.body.style.cursor = axe === 'horizontal' ? 'ew-resize' : 'ns-resize';
					}
				} else {
					document.body.style.cursor = 'auto';
				}
			}
		}

		document.addEventListener('pointerup', onLeave);
		// document.addEventListener('pointerleave', onLeave);
		document.addEventListener('pointermove', onMove);
		return () => {
			document.removeEventListener('pointerup', onLeave);
			// document.removeEventListener('pointerleave', onLeave);
			document.removeEventListener('pointermove', onMove);
		}
	}, [state]);

	const onSplitDown = (e: React.PointerEvent<HTMLDivElement>) => {
		console.log('onSplitDown', e.target);
		e.preventDefault();
		e.stopPropagation();
		internalState.current.dragging = true;
	};

	const onSubdivideDown = (corner: Exclude<SplitviewInternalState['corner'], undefined>) => (e: React.PointerEvent<HTMLDivElement>) => {
		console.log('onSubdivideDown', e.target);
		e.preventDefault();
		e.stopPropagation();
		internalState.current = {
			dragging: true,
			corner,
			originalMain: state.left
		};
	};

	const axe = state.axe;
	const L = axe === 'horizontal' ? 'left' : 'top';
	const T = axe === 'horizontal' ? 'top' : 'left';
	const W = axe === 'horizontal' ? 'width' : 'height';
	const H = axe === 'horizontal' ? 'height' : 'width';

	console.log('render', state.main);

	return (
		<styled.Splitview>
			{state.left && state.right && <styled.HandleContainer>
				<styled.HandleSplit
					ref={dividerRef}
					axe={axe}
					style={{
						[L]: `${state.split.toFixed(4)}%`,
						[T]: 'auto'
					}}
					onPointerDown={onSplitDown}
				/>
			</styled.HandleContainer>}
			{(!state.left !== !state.right || internalState.current.corner) && <styled.SubdivideContainer ref={subdivideRef}>
				<styled.HandleSubdivide corner="top-left" onPointerDown={onSubdivideDown('top-left')} />
				<styled.HandleSubdivide corner="top-right" onPointerDown={onSubdivideDown('top-right')} />
				<styled.HandleSubdivide corner="bottom-left" onPointerDown={onSubdivideDown('bottom-left')} />
				<styled.HandleSubdivide corner="bottom-right" onPointerDown={onSubdivideDown('bottom-right')} />
			</styled.SubdivideContainer>}
			<styled.ViewContainer axe={axe}>
				{state.left && 
					<styled.View
						ref={leftViewRef}
						axe={axe}
						resizable={state.main == 'left'}
						style={{
							[W]: state.main == 'left' && state.right ? `${state.split.toFixed(4)}%` : 'auto',
							[H]: 'auto'
						}}
					>
						{state.left}
					</styled.View>
				}
				{state.right &&
					<styled.View
						ref={rightViewRef}
						axe={axe}
						resizable={state.main == 'right'}
						style={{
							[W]: state.main == 'right' && state.left ? `${(100 - state.split).toFixed(4)}%` : 'auto',
							[H]: 'auto'
						}}
					>
						{state.right}
					</styled.View>
				}
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
