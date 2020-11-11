import 'styled-components';

declare module 'styled-components' {
	// eslint-disable-next-line @typescript-eslint/interface-name-prefix
	export interface DefaultTheme {
		colors: {
			primary: string
		};
	}
}
