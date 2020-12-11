import React, { useState } from 'react';
import Layout from './Layout';

export default function Workbench() {

	const [panes, setPanes] = useState([
		{
			key: 'main',
			top: 0,
			right: 100,
			bottom: 100,
			left: 0,
			elem: <div>Main</div>
		}
	]);

	return (
		<Layout panes={panes} onChange={setPanes} />
	)
}