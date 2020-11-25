import React from 'react';
import Splitview from './Splitview';
import Gridview from './Gridview';

export default function Workbench() {
	return (
		<div style={{ flexDirection: 'row', flex: '1' }}>
			<div style={{ height: '50vh', width: '100%', background: 'rgba(230, 230, 230, 0.5)', display: 'flex' }}>
				<Splitview defaultView={<div>Splitview</div>} />
			</div>
			<div style={{ height: '50vh', width: '100%', display: 'flex' }}>
				<Gridview />
			</div>
		</div>
	)
}