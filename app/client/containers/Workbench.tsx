import React, { useState, useRef, useEffect, createRef } from 'react';
import Layout, { PaneContext } from './Layout';
 
import View from './View';
import Viewport from './views/Viewport';
import Outline from './views/Outline';
import Properties from './views/Properties';
import './Workbench.scss';
import { Editor, EditorContext } from '../contexts/Editor';

export default function Workbench() {

	const [, update] = useState({});
	const editor = useRef<Editor>(new Editor());
	const canvasRef = createRef<HTMLCanvasElement>();

	useEffect(() => {
		(window as any).pxlrEditor = editor.current;

		editor.current.onUpdate = () => update({});
		editor.current.init(canvasRef.current!);

		// TODO : Initialize editor state from localStorage
		//editorState.current.loadFromLocalStorage();
		editor.current.setPanes([
			{
				key: 'main',
				top: 0,
				right: 60,
				bottom: 100,
				left: 0,
				elem: <PaneContext.Consumer>{pane => <View><Viewport options={{ type: 'Blank' }} /></View>}</PaneContext.Consumer>
			},
			{
				key: 'outline',
				top: 0,
				right: 100,
				bottom: 40,
				left: 60,
				elem: <PaneContext.Consumer>{pane => <View><Outline /></View>}</PaneContext.Consumer>
			},
			{
				key: 'properties',
				top: 40,
				right: 100,
				bottom: 100,
				left: 60,
				elem: <PaneContext.Consumer>{pane => <View><Properties /></View>}</PaneContext.Consumer>
			}
		]);
	}, []);

	return (
		<EditorContext.Provider value={editor.current}>
			<div className="workbench">
				<canvas ref={canvasRef} id="mainCanvas" />
				<Layout panes={editor.current.panes} onChange={panes => editor.current.setPanes(panes)} onDragging={panes => editor.current.setPanes(panes, false)} />
			</div>
		</EditorContext.Provider>
	)
}