import React, { useContext, useEffect, useRef } from 'react';
import { EditorContext } from '../contexts/Editor';
import type { Viewport, ViewportOptions } from '../editor';
import './Viewport.scss';

export interface ViewportProps {
	id: string,
	options: ViewportOptions,
};

export default function Viewport({ id, options }: ViewportProps) {
	const viewportRef = useRef<HTMLDivElement>(null);
	const editor = useContext(EditorContext);

	useEffect(() => {
		const { top, right, bottom, left } = viewportRef.current!.getBoundingClientRect();
		const viewport: Viewport = {
			key: id,
			bounds: { top, right, bottom, left },
			options
		};
		editor.addViewport(viewport);

		function updateViewportBounds() {
			const { top, right, bottom, left } = viewportRef.current!.getBoundingClientRect();
			viewport.bounds = { top, right, bottom, left };
			editor.updateViewport(viewport);
		}

		window.addEventListener('resize', updateViewportBounds);

		return () => {
			window.removeEventListener('resize', updateViewportBounds);
			editor.removeViewport(id);
		};
	}, [editor]);

	return (<div ref={viewportRef} id={id} className="viewport" />);
}