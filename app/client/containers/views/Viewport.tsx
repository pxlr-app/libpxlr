import React, { useContext } from 'react';
import { PaneContext } from '../Layout';
import BaseViewport from '../Viewport';
import type { Viewport, ViewportOptions } from '../../editor';

export interface ViewportProps {
	options: ViewportOptions,
};

export default function Viewport({ options }: ViewportProps) {
	const pane = useContext(PaneContext);

	return (<BaseViewport id={pane.key} options={options} />);
}