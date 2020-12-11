import React from 'react';
import ReactDOM from 'react-dom';
import App from './containers/App';

const worker = new Worker(
	new URL('./worker.js', import.meta.url),
	{ type: 'module' }
);
worker.addEventListener('message', (msg) => {
	console.log('Main', msg.data);
});

setInterval(() => {
	worker.postMessage('PONG');
}, 2000);

ReactDOM.render(
	<React.StrictMode>
		<App />
	</React.StrictMode>,
	document.getElementById('root'),
);
	
// Hot Module Replacement (HMR) - Remove this snippet to remove HMR.
// Learn more: https://www.snowpack.dev/#hot-module-replacement
if (import.meta.hot) {
	import.meta.hot.accept();
}