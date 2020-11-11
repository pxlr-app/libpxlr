import React, { useState, useEffect } from 'react';
import * as styled from './Pane.styled';

interface PaneProps {
	defaultPane: React.ReactNode,
}

interface PaneState {
	left: React.ReactNode,
	right?: React.ReactNode
}

export default function Pane(props: React.PropsWithChildren<PaneProps>) {
	let [state, setState] = useState<PaneState>(() => ({
		left: props.defaultPane,
		right: props.defaultPane,
	}));
	
	return (
		<styled.Container axe="horizontal">
			<styled.Body>
				<styled.Handle valign="top" halign="left"><styled.Triangle /></styled.Handle>
				<styled.Handle valign="top" halign="right"><styled.Triangle /></styled.Handle>
				<styled.Handle valign="bottom" halign="left"><styled.Triangle /></styled.Handle>
				<styled.Handle valign="bottom" halign="right"><styled.Triangle /></styled.Handle>
				<div>{state.left}</div>
			</styled.Body>
			{state.right && (
				<>
					<styled.Divider />
					<styled.Body>
						<styled.Handle valign="top" halign="left"><styled.Triangle /></styled.Handle>
						<styled.Handle valign="top" halign="right"><styled.Triangle /></styled.Handle>
						<styled.Handle valign="bottom" halign="left"><styled.Triangle /></styled.Handle>
						<styled.Handle valign="bottom" halign="right"><styled.Triangle /></styled.Handle>
						<div>{state.right}</div>
					</styled.Body>
				</>
			)}
		</styled.Container>
	)
}