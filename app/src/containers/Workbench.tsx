import React, { useState, useEffect } from 'react';
import Pane from './Pane';
import Splitview from './Splitview';

export default function Workbench() {
	return (
		// <Pane defaultPane={<div>Hey</div>} />
		<Splitview defaultView={<div>Hey</div>} />
	)
}