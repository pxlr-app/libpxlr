self.addEventListener('message', (msg) => {
	console.log('Worker', msg.data);
});

setInterval(() => {
	self.postMessage('PING');
}, 1800);


// (async () => {
// 	// @ts-ignore
// 	const wasm = await import('/editor/app.js');
// 	console.log(wasm);
// })();

// @ts-ignore
import wasm from '/editor/app.js';
wasm();