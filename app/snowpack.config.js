module.exports = {
	mount: {
		public: '/',
		src: '/_dist_',
	},
	plugins: [
		'@snowpack/plugin-sass',
		'@snowpack/plugin-react-refresh',
		'@snowpack/plugin-dotenv',
		'@snowpack/plugin-typescript',
		'@snowpack/plugin-webpack'
	],
	install: [
		/* ... */
	],
	installOptions: {
		/* ... */
	},
	devOptions: {
		/* ... */
	},
	buildOptions: {
		/* ... */
	},
	proxy: {
		/* ... */
	},
	alias: {
		/* ... */
	},
};
