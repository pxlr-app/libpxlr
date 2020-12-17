module.exports = {
	mount: {
		public: '/',
		client: '/_dist_',
		pkg: '/editor'
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
		hmr: false,
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
