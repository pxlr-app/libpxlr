export interface ViewportBounds {
	top: number,
	right: number,
	bottom: number,
	left: number,
}

export type ViewportOptions =
	{
		type: 'Blank'
	};

export interface Viewport {
	key: string,
	bounds: ViewportBounds,
	options: ViewportOptions,
}

export type Command = 
	  { cmd: 'Init', offscreen_canvas: OffscreenCanvas }
	| { cmd: 'Ping' }
	| { cmd: 'AddViewport', viewport: Viewport }
	| { cmd: 'RemoveViewport', key: string }
	| { cmd: 'UpdateViewport', viewport: Viewport }
	| { cmd: 'Resize', width: number, height: number }
	| { cmd: 'Draw' }
	;