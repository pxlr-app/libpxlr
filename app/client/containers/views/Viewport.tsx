import React, { useContext, useEffect } from 'react';
import { EditorContext } from '../../contexts/Editor';
import { PaneContext } from '../Layout';

export default function Viewport({ children }: React.PropsWithChildren<{}>) {
	const editorState = useContext(EditorContext);
	const pane = useContext(PaneContext);

	useEffect(() => {
		editorState.registerViewport(pane.key);
		return () => {
			editorState.unregisterViewport(pane.key);
		}
	}, []);

	return (<div>{children}</div>);
}