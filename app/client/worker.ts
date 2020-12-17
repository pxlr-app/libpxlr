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
	};

(async () => {
	await init();

	self.postMessage({ cmd: 'Ready' });

	let editor: number;

	self.addEventListener('message', (msg: MessageEvent<Command>) => {
		if (msg.data.cmd === 'Init') {
			const canvas = msg.data.offscreen_canvas;
			editor = Editor.load(canvas, canvas.width, canvas.height);
		} else if (msg.data.cmd === 'Resize') {
			Editor.resize(editor, msg.data.width, msg.data.height);
		} else {
			process_command(editor, JSON.stringify(msg.data));
		}
	});
})();