import React, { useState, useEffect, createRef } from 'react';
import * as styled from './Splitview.styled';

export interface SplitviewProps {
	defaultView: React.ReactNode,
	axe?: 'horizontal' | 'vertical',
}

interface SplitviewState {
	left: {
		node: React.ReactNode,
		dom?: React.RefObject<HTMLDivElement>;
		width: number,
	}
	right?: SplitviewState['left'],
	dragging: boolean,
	startPos: number,
}

export default function Splitview(props: SplitviewProps) {
	const axe = props.axe ?? 'horizontal';
	const clientX = axe === 'horizontal' ? 'clientX' : 'clientY';
	const width = axe === 'horizontal' ? 'width' : 'height';
	const offsetWidth = axe === 'horizontal' ? 'offsetWidth' : 'offsetHeight';

	let [state, setState] = useState<SplitviewState>(() => ({
		left: {
			node: props.defaultView,
			dom: createRef<HTMLDivElement>(),
			width: 400,
		},
		right: {
			node: props.defaultView,
			dom: createRef<HTMLDivElement>(),
			width: 200,
		},
		dragging: false,
		startPos: 0,
	}));

	useEffect(() => {
		if (!state.dragging) {
			document.body.style.cursor = 'auto';
			return;
		}

		document.body.style.cursor = axe === 'horizontal' ? 'ew-resize' : 'ns-resize';

		const onMove = (e: PointerEvent) => {
			if (state.dragging) {
				const currentPos = e[clientX];
				const delta = currentPos - state.startPos;
				console.log('Start', state.startPos, 'Current', currentPos, 'Delta', delta);
			}
		};
		const onLeave = (e: PointerEvent) => {
			const currentPos = e[clientX];
			const delta = currentPos - state.startPos;

			setState({
				...state,
				left: {
					...state.left,
					width: state.left.width + delta,
				},
				right: state.right
					? {
						...state.right,
						//width: state.right.width - delta,
					}
					: undefined,
				dragging: false,
			});
		};

		document.addEventListener('pointerup', onLeave);
		document.addEventListener('pointermove', onMove);

		return () => {
			document.removeEventListener('pointerup', onLeave);
			document.removeEventListener('pointermove', onMove);
		};
	}, [state]);

	const onPointerDown = (e: React.PointerEvent<HTMLDivElement>) => {
		setState({
			...state,
			dragging: true,
			startPos: e[clientX]
		});
	};

	return (
		<styled.Splitview>
			<styled.HandleContainer>
				{state.right && <styled.HandleSplit axe="horizontal" offset={state.left.width} onPointerDown={onPointerDown} />}
				<styled.HandleSubdivide axe="horizontal" offset={0} width={state.left.width}>
					<styled.HandleSubdivideCorner corner="top-left" />
					<styled.HandleSubdivideCorner corner="top-right" />
					<styled.HandleSubdivideCorner corner="bottom-left" />
					<styled.HandleSubdivideCorner corner="bottom-right" />
				</styled.HandleSubdivide>
				{state.right && <styled.HandleSubdivide axe="horizontal" offset={state.left.width} width={state.right.width}>
					<styled.HandleSubdivideCorner corner="top-left" />
					<styled.HandleSubdivideCorner corner="top-right" />
					<styled.HandleSubdivideCorner corner="bottom-left" />
					<styled.HandleSubdivideCorner corner="bottom-right" />
				</styled.HandleSubdivide>}
			</styled.HandleContainer>
			<styled.ViewContainer>
				<styled.View axe="horizontal" offset={0} width={state.left.width}>
					{state.left.node}
				</styled.View>
				{state.right && <styled.View axe="horizontal" offset={state.left.width} width={state.right.width}>
					{state.right.node}
				</styled.View>}
			</styled.ViewContainer>
		</styled.Splitview>
	);
}