import { createContext } from 'react';
import type { PaneProps } from '../containers/Layout';

export interface EditorState {
	panes: PaneProps[],
}

export const EditorContext = createContext<EditorState>({
	panes: []
});