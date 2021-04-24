const production = !process.env.NODE_ENV;

module.exports = {
	preprocess: require('svelte-sequential-preprocessor')([
		require('svelte-preprocess').typescript(),
		require('svelte-windicss-preprocess-exp').preprocess({})
	])
};