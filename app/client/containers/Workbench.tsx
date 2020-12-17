import React, { useState, createContext } from 'react';
import Layout, { PaneContext } from './Layout';
import { EditorContext, EditorState } from '../contexts/Editor';
import View from './View';
import Viewport from './views/Viewport';
import Outline from './views/Outline';
import Properties from './views/Properties';
import './Workbench.scss';

// type Command = 
// 	{
// 		cmd: 'Ready'
// 	};

// const worker = new Worker(
// 	new URL('./worker.js', import.meta.url),
// 	{ type: 'module' }
// );
// document.addEventListener('readystatechange', () => {
// 	worker.addEventListener('message', (msg: MessageEvent<Command>) => {
// 		console.log('Main', msg.data);

// 		if (msg.data.cmd === 'Ready') {
// 			const canvas = document.getElementById('mainCanvas') as HTMLCanvasElement;
// 			const offscreen_canvas = canvas.transferControlToOffscreen();
// 			const bounds = canvas.getBoundingClientRect();
// 			offscreen_canvas.width = bounds.width;
// 			offscreen_canvas.height = bounds.height;
// 			worker.postMessage({ cmd: 'Init', offscreen_canvas }, [offscreen_canvas]);

// 			window.addEventListener('resize', e => {
// 				const bounds = canvas.getBoundingClientRect();
// 				worker.postMessage({ cmd: 'Resize', width: bounds.width, height: bounds.height });
// 			});

// 			setInterval(() => {
// 				// TODO 
// 				worker.postMessage({ cmd: 'Ping' });
// 			}, 1000);
// 		}
// 	});
// });

export default function Workbench() {

	const [state, setState] = useState<EditorState>({
		panes: [
			{
				key: 'main',
				top: 0,
				right: 80,
				bottom: 100,
				left: 0,
				elem: <PaneContext.Consumer>{pane => <View><Viewport /></View>}</PaneContext.Consumer>
			},
			{
				key: 'outline',
				top: 0,
				right: 100,
				bottom: 40,
				left: 80,
				elem: <PaneContext.Consumer>{pane => <View><Outline /></View>}</PaneContext.Consumer>
			},
			{
				key: 'properties',
				top: 40,
				right: 100,
				bottom: 100,
				left: 80,
				elem: <PaneContext.Consumer>{pane => <View><Properties /></View>}</PaneContext.Consumer>
			}
		]
	});

	return (
		<EditorContext.Provider value={state}>
			<div className="workbench">
				<canvas id="mainCanvas" />
				<Layout panes={state.panes} onChange={panes => setState({ ...state, panes })} />
			</div>
		</EditorContext.Provider>
	)
}