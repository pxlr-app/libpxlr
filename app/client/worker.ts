// @ts-ignore
import init, { process_command, Editor } from '/editor/editor.js';

type Command = 
	  {
		cmd: 'Init',
		offscreen_canvas: OffscreenCanvas
	}
	| {
		cmd: 'Resize',
		width: number,
		height: number,
	}
	| {
		cmd: 'RegisterViewport',
		id: string,
		top: number,
		right: number,
		bottom: number,
		left: number,
	}
	| {
		cmd: 'UpdateViewport',
		id: string,
		top: number,
		right: number,
		bottom: number,
		left: number,
	}
	| {
		cmd: 'UnregisterViewport',
		id: string,
	};

(async () => {
	await init();

	self.postMessage({ cmd: 'Ready' });

	let editor: number;

	self.addEventListener('message', (msg: MessageEvent<Command>) => {
		if (msg.data.cmd === 'Init') {
			const canvas = msg.data.offscreen_canvas;
			editor = Editor.init(canvas, canvas.width, canvas.height);
		} else if (msg.data.cmd === 'Resize') {
			Editor.resize(editor, msg.data.width, msg.data.height);
		} else if (msg.data.cmd === 'RegisterViewport') {
			const { id, top, right, bottom, left} = msg.data;
			Editor.add_viewport(editor, id, top, right, bottom, left);
		} else if (msg.data.cmd === 'UpdateViewport') {
			const { id, top, right, bottom, left} = msg.data;
			Editor.update_viewport(editor, id, top, right, bottom, left);
		} else if (msg.data.cmd === 'UnregisterViewport') {
			Editor.remove_viewport(editor, msg.data.id);
		} else {
			process_command(editor, JSON.stringify(msg.data));
		}
	});

	function draw(time: number) {
		if (editor) {
			process_command(editor, JSON.stringify({ cmd: 'Draw' }));
		}
		requestAnimationFrame(draw);
	  }
	  requestAnimationFrame(draw);
})();