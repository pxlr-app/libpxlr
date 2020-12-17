import React, { useState } from 'react';
import './View.scss';

export default function View({ children }: React.PropsWithChildren<{}>) {
	return (<div className="view">{children}</div>);
}