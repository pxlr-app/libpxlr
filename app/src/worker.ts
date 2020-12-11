// importScripts('/pxlr/pxlr.js', { type: "module" });
window = self;

self.onmessage = function (msg: MessageEvent<any>) {
	console.log('Worker', msg.data);
}

setInterval(() => {
	postMessage('PING');
}, 1800);

import pxlr from './pkg/app';
console.log('pxlr', pxlr);