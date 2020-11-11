import React, { useState, useEffect } from 'react';
import { ThemeProvider } from 'styled-components';
import * as styled from './App.styled';
import Workbench from './Workbench';

export default function App() {
	return (
		<ThemeProvider theme={{
			colors: {
				primary: 'red'
			}
		}}>
			<styled.Global />
			<Workbench />
		</ThemeProvider>
	);
}