import React, { useState, useEffect, useRef } from 'react';
import * as styled from './Splitview.styled';
import useAnimationFrame from '../helpers/useAnimationFrame';

export interface SplitviewProps {
	defaultView: React.ReactElement,
	left?: React.ReactElement,
	right?: React.ReactElement,
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
	snapPointerEvent?: PointerEvent,
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
		left: props.left ?? props.defaultView,
		right: props.right,
		main: 'left',
		split: 50.,
	}));

	function getSplitMetric(e: PointerEvent, clientX: 'clientX' | 'clientY', left: 'left' | 'top', width: 'width' | 'height') {
		const pos = e[clientX];
		const target = dividerRef.current;
		if (target) {
			const bounds = (target?.parentElement ?? target).getBoundingClientRect()!;
			const relativePos = pos - bounds[left];
			const percentPos = Math.max(0, Math.min(1, relativePos / bounds[width]));
			return [percentPos, relativePos, bounds[width]];
		}
		return [undefined, undefined];
	}

	useEffect(() => {
		function onLeave(e: PointerEvent) {
			if (internalState.current.dragging) {
				internalState.current = { dragging: false };
				document.body.style.cursor = 'auto';

				const [percent] = state.axe === 'horizontal'
									? getSplitMetric(e,  'clientX', 'left', 'width')
									: getSplitMetric(e,  'clientY', 'top', 'height');
				if (percent !== undefined) {
					setState({ ...state, split: percent * 100 });
				}
			}
		}

		function onMove(pointerEvent: PointerEvent) {
			internalState.current.pointerEvent = pointerEvent;

			if (internalState.current.dragging) {
				const { dragging, lastPointerEvent, snapPointerEvent, corner } = internalState.current;

				// Skip this callback if position hasn't changed
				if (pointerEvent?.clientX === lastPointerEvent?.clientX && pointerEvent?.clientY === lastPointerEvent?.clientY) {
					return;
				}
				internalState.current.lastPointerEvent = pointerEvent;
				internalState.current.snapPointerEvent = snapPointerEvent ?? pointerEvent;

				if (dragging && pointerEvent) {
					// Dragging corner?
					if (corner) {
						const snapPointerEvent = internalState.current.snapPointerEvent;
						const subdivider = subdivideRef.current;
						if (subdivider) {
							const deltaX = pointerEvent.clientX - snapPointerEvent.clientX;
							const deltaY = pointerEvent.clientY - snapPointerEvent.clientY;

							if (deltaX * deltaX + deltaY * deltaY >= 20 * 20) {
								internalState.current.snapPointerEvent = pointerEvent;

								const isHorizontal = Math.abs(deltaX) > Math.abs(deltaY);

								let left: SplitviewState['right'] = undefined;
								let right: SplitviewState['right'] = undefined;
								let main: SplitviewState['main'] = 'left';

								if (
									(corner === 'top-left') ||
									(corner === 'top-right' && !isHorizontal) ||
									(corner === 'bottom-left' && isHorizontal)
								) {
									left = <Splitview defaultView={props.defaultView} />;
									right = <Splitview defaultView={internalState.current.originalMain!} />;
									main = 'right';
								} else {
									left = <Splitview defaultView={internalState.current.originalMain!} />;
									right = <Splitview defaultView={props.defaultView} />;
								}

								const axe = isHorizontal ? 'horizontal' : 'vertical';

								// Flip splitview
								if (!state.left || !state.right || state.axe !== axe) {
									// TODO recalculate split to be under current pointer
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
					}
					
					// Dragging split
					const [percent, pos, width] = state.axe === 'horizontal'
									? getSplitMetric(pointerEvent,  'clientX', 'left', 'width')
									: getSplitMetric(pointerEvent,  'clientY', 'top', 'height');
					if (percent !== undefined) {
						const viewRef = state.main === 'left' ? leftViewRef : rightViewRef;
						const otherViewRef = state.main === 'left' ? rightViewRef : leftViewRef;

						const L = state.axe === 'horizontal' ? 'left' : 'top';
						const T = state.axe === 'horizontal' ? 'top' : 'left';
						const W = state.axe === 'horizontal' ? 'width' : 'height';
						const H = state.axe === 'horizontal' ? 'height' : 'width';

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
		e.preventDefault();
		e.stopPropagation();
		if (!e.ctrlKey) {
			internalState.current.dragging = true;
		}
	};

	const onSplitUp = (e: React.PointerEvent<HTMLDivElement>) => {
		e.preventDefault();
		e.stopPropagation();
		if (e.ctrlKey) {
			let { split } = state;
			if (internalState.current.pointerEvent) {
				const [percent] = state.axe === 'horizontal'
									? getSplitMetric(internalState.current.pointerEvent,  'clientY', 'top', 'height')
									: getSplitMetric(internalState.current.pointerEvent,  'clientX', 'left', 'width');
				if (percent !== undefined) {
					split = percent * 100;
				}
			}
			setState({
				...state,
				split,
				axe: state.axe === 'horizontal' ? 'vertical' : 'horizontal'
			});
		}
	};

	const onSubdivideDown = (corner: Exclude<SplitviewInternalState['corner'], undefined>) => (e: React.PointerEvent<HTMLDivElement>) => {
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

	console.log('render', state);

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
					onPointerUp={onSplitUp}
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