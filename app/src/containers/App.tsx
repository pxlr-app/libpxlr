import React, { useEffect } from 'react';
import Workbench from './Workbench';
import './App.scss';

export default function App() {
	useEffect(() => {

		function onKeyDown(e: KeyboardEvent) {
			if (e.ctrlKey) {
				document.body.classList.add('key--ctrl');
			}
			if (e.altKey) {
				document.body.classList.add('key--alt');
			}
			if (e.shiftKey) {
				document.body.classList.add('key--shift');
			}
			if (e.metaKey) {
				document.body.classList.add('key--meta');
			}
			if (e.ctrlKey || e.altKey || e.shiftKey || e.metaKey) {
				e.preventDefault();
			}
		}

		function onKeyUp(e: KeyboardEvent) {
			if (!e.ctrlKey) {
				document.body.classList.remove('key--ctrl');
			}
			if (!e.altKey) {
				document.body.classList.remove('key--alt');
			}
			if (!e.shiftKey) {
				document.body.classList.remove('key--shift');
			}
			if (!e.metaKey) {
				document.body.classList.remove('key--meta');
			}
			if (!e.ctrlKey || !e.altKey || !e.shiftKey || !e.metaKey) {
				e.preventDefault();
			}
		}

		document.addEventListener('keydown', onKeyDown);
		document.addEventListener('keyup', onKeyUp);
		return () => {
			document.removeEventListener('keydown', onKeyDown);
			document.removeEventListener('keyup', onKeyUp);
		}
	}, []);

	return (
		<Workbench />
	);
}