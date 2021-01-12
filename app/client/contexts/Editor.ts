import { createContext } from 'react';
import type { PaneProps } from '../containers/Layout';

type OnUpdate = () => void;

type Command = 
	{
		cmd: 'Ready'
	};

export class Editor {
	public panes: PaneProps[] = [];
	public readonly viewports: ReadonlySet<PaneProps['key']> = new Set();
	private worker?: Worker;
	private canvas?: HTMLCanvasElement;
	public onUpdate?: OnUpdate;

	constructor() {}

	public loadFromLocalStorage() {}

	public init(canvas: HTMLCanvasElement) {
		if (this.canvas) {
			throw new Error(`This editor has already a Canvas`);
		}

		const worker = new Worker(
			new URL('../worker.js', import.meta.url),
			{ type: 'module' }
		);

		worker.addEventListener('message', (msg: MessageEvent<Command>) => {
			console.log('Main', msg.data);
	
			if (msg.data.cmd === 'Ready') {
				const offscreen_canvas = canvas.transferControlToOffscreen();
				const bounds = canvas.getBoundingClientRect();
				offscreen_canvas.width = bounds.width;
				offscreen_canvas.height = bounds.height;
				worker.postMessage({ cmd: 'Init', offscreen_canvas }, [offscreen_canvas]);

				this.viewports.forEach(id => {
					const pane = this.panes.find(p => p.key === id);
					if (pane) {
						worker.postMessage({ cmd: 'RegisterViewport', id, top: pane.top, right: pane.right, bottom: pane.bottom, left: pane.left });
					}
				});
			}
		});

		window.addEventListener('resize', e => {
			const bounds = canvas.getBoundingClientRect();
			canvas.width = bounds.width;
			canvas.height = bounds.height;
			worker.postMessage({ cmd: 'Resize', width: bounds.width, height: bounds.height });
		});

		setInterval(() => {
			// TODO 
			worker.postMessage({ cmd: 'Ping' });
		}, 1000);

		this.canvas = canvas;
		this.worker = worker;
	}

	public setPanes(panes: PaneProps[], needUpdate = true) {
		this.panes = panes;
		this.worker && this.viewports.forEach(id => {
			const pane = this.panes.find(p => p.key === id);
			if (pane) {
				this.worker!.postMessage({ cmd: 'UpdateViewport', id, top: pane.top, right: pane.right, bottom: pane.bottom, left: pane.left });
			}
		});
		needUpdate && this.onUpdate && this.onUpdate();
	}

	public registerViewport(paneKey: PaneProps['key'], needUpdate = true) {
		const pane = this.panes.find(p => p.key === paneKey);
		if (pane) {
			(this.viewports as Set<PaneProps['key']>).add(paneKey);
			this.worker && this.worker!.postMessage({ cmd: 'RegisterViewport', id: paneKey, top: pane.top, right: pane.right, bottom: pane.bottom, left: pane.left });
		}
		needUpdate && this.onUpdate && this.onUpdate();
	}

	public unregisterViewport(paneKey: PaneProps['key'], needUpdate = true) {
		(this.viewports as Set<PaneProps['key']>).delete(paneKey);
		this.worker && this.worker!.postMessage({ cmd: 'UnregisterViewport', id: paneKey });
		needUpdate && this.onUpdate && this.onUpdate();
	}
}

export const EditorContext = createContext<Editor>(new Editor());