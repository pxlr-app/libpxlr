import { createContext } from 'react';
import type { PaneProps } from '../containers/Layout';
import type { Command, Viewport } from '../editor';

type OnUpdate = () => void;

type WorkerResponse = 
	{
		cmd: 'Ready'
	};

export class Editor {
	public panes: PaneProps[] = [];
	private worker?: Worker;
	private canvas?: HTMLCanvasElement;
	private ready: boolean = false;
	private commandBuffer: Command[] = [];
	public onUpdate?: OnUpdate;


	constructor() {}

	private sendCommand(command: Command) {
		if (this.ready && this.worker) {
			this.worker.postMessage(command);
		} else {
			this.commandBuffer.push(command);
		}
	}

	public init(canvas: HTMLCanvasElement) {
		if (this.canvas) {
			throw new Error(`This editor has already a Canvas`);
		}

		const worker = new Worker(
			new URL('../worker.js', import.meta.url),
			{ type: 'module' }
		);

		worker.addEventListener('message', (msg: MessageEvent<WorkerResponse>) => {
			console.log('Main', msg.data);
	
			if (msg.data.cmd === 'Ready') {
				const offscreen_canvas = canvas.transferControlToOffscreen();
				const bounds = canvas.getBoundingClientRect();
				offscreen_canvas.width = bounds.width;
				offscreen_canvas.height = bounds.height;
				worker.postMessage({ cmd: 'Init', offscreen_canvas }, [offscreen_canvas]);
				worker.postMessage({ cmd: 'Resize', width: bounds.width, height: bounds.height });
				
				let cmd: Command | undefined;
				while ((cmd = this.commandBuffer.pop()) !== undefined) {
					worker.postMessage(cmd);
				}
				this.ready = true;
			}
		});

		// TODO on uninit remove event listener
		window.addEventListener('resize', e => {
			const bounds = canvas.getBoundingClientRect();
			this.sendCommand({ cmd: 'Resize', width: bounds.width, height: bounds.height });
		});

		// TODO on uninit remove timer
		setInterval(() => {
			// TODO 
			worker.postMessage({ cmd: 'Ping' });
		}, 1000);

		this.canvas = canvas;
		this.worker = worker;
	}

	public setPanes(panes: PaneProps[], needUpdate = true) {
		this.panes = panes;
		window.dispatchEvent(new Event('resize'));
		needUpdate && this.onUpdate && this.onUpdate();
	}

	public addViewport(viewport: Viewport) {
		this.sendCommand({ cmd: 'AddViewport', viewport });
	}

	public removeViewport(key: string) {
		this.sendCommand({ cmd: 'RemoveViewport', key });
	}

	public updateViewport(viewport: Viewport) {
		this.sendCommand({ cmd: 'UpdateViewport', viewport });
	}
}

export const EditorContext = createContext<Editor>(new Editor());