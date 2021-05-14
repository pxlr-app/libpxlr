import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react-refresh';

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [
		react(),
	],
	resolve: {
		alias: {
			libpxlr: require('path').resolve(__dirname, '../libpxlr/pkg/')
		}
	},
})
