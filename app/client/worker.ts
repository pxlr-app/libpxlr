// @ts-ignore
import init, { Editor } from '/editor/editor.js';
import type { Command } from './editor';

(async () => {
	await init();

	self.postMessage({ cmd: 'Ready' });

	let editor: number;

	self.addEventListener('message', (msg: MessageEvent<Command>) => {
		if (msg.data.cmd === 'Init') {
			editor = Editor.init();
			Editor.send_command_with_canvas(editor, JSON.stringify({ cmd: 'Init' }), msg.data.offscreen_canvas);
		} else {
			console.debug('send_command', msg.data);
			Editor.send_command(editor, JSON.stringify(msg.data));
		}
	});

	function draw(time: number) {
		if (editor) {
			Editor.send_command(editor, JSON.stringify({ cmd: 'Draw' }));
		}
		requestAnimationFrame(draw);
	}
	requestAnimationFrame(draw);
})();