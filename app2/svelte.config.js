const autoPreprocess = require('svelte-preprocess');
const production = !process.env.NODE_ENV;

module.exports = {
	preprocess: autoPreprocess({
		sourceMap: !production,
		defaults: {
			script: 'typescript',
		},
		postcss: {
			plugins: [
				require("tailwindcss"),
				require("autoprefixer"),
				require("postcss-nesting")
			],
		}
	}),
};