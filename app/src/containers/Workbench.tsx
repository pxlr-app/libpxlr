import React, { useState, useEffect } from 'react';
import Pane from './Pane';
import Splitview from './Splitview';

export default function Workbench() {

	return (
		// <Pane defaultPane={<div>Hey</div>} />
		// <Splitview axe="horizontal" defaultView={<div>View</div>} />
		<Splitview defaultView={<div>Content</div>} />
	)
}