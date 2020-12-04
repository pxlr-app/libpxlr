import React, { useState } from 'react';
import Layout from './Layout';

export default function Workbench() {

	const [panes, setPanes] = useState([
		{
			key: '0',
			top: 0,
			right: 50,
			bottom: 33.3333,
			left: 0,
			elem: <div>0</div>
		}, {
			key: '1',
			top: 0,
			right: 100,
			bottom: 50,
			left: 50,
			elem: <div>1</div>
		}, {
			key: '2',
			top: 33.3333,
			bottom: 66.6666,
			right: 25,
			left: 0,
			elem: <div>2</div>
		}, {
			key: '3',
			top: 66.6666,
			right: 50,
			bottom: 100,
			left: 0,
			elem: <div>3</div>
		}, {
			key: '4',
			top: 50,
			right: 100,
			bottom: 100,
			left: 50,
			elem: <div>4</div>
		}, {
			key: '5',
			top: 33.3333,
			bottom: 66.6666,
			right: 50,
			left: 25,
			elem: <div>5</div>
		}
	]);

	return (
		<Layout panes={panes} onChange={setPanes} />
	)
}