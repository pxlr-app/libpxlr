<script lang="ts">
	import Layout from './Layout.svelte';
</script>

<div class="workbench">
	<canvas id="mainCanvas" />
	<Layout panes={[
		{
			key: 'main',
			top: 0,
			right: 60,
			bottom: 100,
			left: 0,
			elem: 'Viewport'
		},
		{
			key: 'outline',
			top: 0,
			right: 100,
			bottom: 40,
			left: 60,
			elem: 'Outline'
		},
		{
			key: 'properties',
			top: 40,
			right: 100,
			bottom: 100,
			left: 60,
			elem: 'Properties'
		}
	]} />
</div>

<style>
	.workbench {
		display: flex;
		flex: 1;
	}

	canvas {
		position: absolute;
		width: 100%;
		height: 100%;
		z-index: 0;
		pointer-events: none;
		background: #333;
	}
</style>