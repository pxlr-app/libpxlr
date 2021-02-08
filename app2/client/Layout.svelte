<script lang="ts">
	import type { Corner, Edge, PaneProps } from './helpers/Layout';
	import { Layout } from './helpers/Layout';
	import { onMount } from 'svelte';
	import { each } from 'svelte/internal';

	

	export let panes: PaneProps[];

	let layoutRef: HTMLDivElement | undefined;
	let bounds = new DOMRect();
	let dragging = false;
	let dragBounds = new DOMRect();
	let layout = new Layout(panes);

	console.log('Layout', layout);

	onMount(() => {
		onResize();
	});

	function onResize() {
		bounds = layoutRef!.getBoundingClientRect();
	}

	function onLeave() {
		if (dragging) {

		}
	}

	function onMove() {
		if (dragging) {

		}
	}

	function onResizerDown(id: number) {
		return (e: PointerEvent) => {
			e.preventDefault();
			e.stopPropagation();
		}
	}

	function onDividerDown(id: number, corner: Corner) {
		return (e: PointerEvent) => {
			e.preventDefault();
			e.stopPropagation();
		}
	}
</script>

<svelte:window on:resize={onResize} />
<svelte:body on:pointerup={onLeave} on:pointermove={onMove} />

<div bind:this={layoutRef} class="layout">
	<div class="container">
		{#each layout.edges as edge, id (edge.id)}
			<div
				bind:this={edge.ref}
				on:pointerdown={onResizerDown(id)}
				class={`resizer resizer--${edge.axe}`}
				style={edge.axe === 'horizontal'
						? `top: ${edge.p.toFixed(6)}%; left: ${edge.s.toFixed(6)}%; width: ${(edge.e - edge.s).toFixed(6)}%; height: var(--edge-size);`
						: `top: ${edge.s.toFixed(6)}%; left: ${edge.p.toFixed(6)}%; width: var(--edge-size); height: ${(edge.e - edge.s).toFixed(6)}%;`}
			/>
		{/each}
	</div>
	<div class="container">
		{#each layout.panes as pane, id (pane.id)}
			<div
				bind:this={pane.dividerRef}
				class="divider"
				style={`top: ${pane.top.toFixed(6)}%; right: ${(100 - pane.right).toFixed(6)}%; bottom: ${(100 - pane.bottom).toFixed(6)}%; left: ${(pane.left).toFixed(6)}%; border-width: ${pane.links.map((link, dir) => link ? (dir === 1 || dir === 2 ? `var(--border-size)` : 0) : `var(--border-size)`).join(' ')};`}
			>
				<div on:pointerdown={onDividerDown(id, 'top-left')} class="divider--top-left" />
				<div on:pointerdown={onDividerDown(id, 'top-right')} class="divider--top-right" />
				<div on:pointerdown={onDividerDown(id, 'bottom-left')} class="divider--bottom-left" />
				<div on:pointerdown={onDividerDown(id, 'bottom-right')} class="divider--bottom-right" />
			</div>
		{/each}
	</div>
	<div class="container">
		{#each layout.panes as pane (pane.id)}
			<div
				bind:this={pane.paneRef}
				class="view"
				style={`top: ${pane.top.toFixed(6)}%; right: ${(100 - pane.right).toFixed(6)}%; bottom: ${(100 - pane.bottom).toFixed(6)}%; left: ${pane.left.toFixed(6)}%; --pane-top-neighbor: ${pane.links[0].length ? 1 : 0}; --pane-right-neighbor: ${pane.links[1].length ? 1 : 0}; --pane-bottom-neighbor: ${pane.links[2].length ? 1 : 0}; --pane-left-neighbor: ${pane.links[3].length ? 1 : 0};`}
			>
				View
			</div>
		{/each}
	</div>
</div>

<style>
	.layout {
		position: relative;
		display: flex;
		flex: 1;
		--edge-size: 10px;
		--subdivide-size: 10px;
		--border-size: 1px;
	}

	.container {
		position: absolute;
		width: 100%;
		height: 100%;
		pointer-events: none;
	}

	.view {
		--pane-top-neighbor: 0;
		--pane-right-neighbor: 0;
		--pane-bottom-neighbor: 0;
		--pane-left-neighbor: 0;
		position: absolute;
		overflow: hidden;
		pointer-events: auto;
	}

	.resizer {
		position: absolute;
		z-index: 35;
		touch-action: none;
		pointer-events: auto;
	}

	.resizer--horizontal {
		cursor: var(--cursor-ns-resize);
		top: 0;
		height: 100%;
		width: var(--edge-size);
		transform: translateY(calc(var(--edge-size) / -2));
	}

	body.key--alt .resizer--horizontal {
		cursor: var(--cursor-swap-horizontal);
	}

	.resizer--vertical {
		cursor: var(--cursor-ew-resize);
		left: 0;
		width: 100%;
		height: var(--edge-size);
		transform: translateX(calc(var(--edge-size) / -2));
	}
	
	body.key--alt .resizer--vertical {
		cursor: var(--cursor-swap-vertical);
	}

	.divider {
		position: absolute;
		pointer-events: none;
	}

	.divider--top-left,
	.divider--top-right,
	.divider--bottom-left,
	.divider--bottom-right {
		position: absolute;
		z-index: 35;
		touch-action: none;
		pointer-events: auto;
		width: var(--subdivide-size);
		height: var(--subdivide-size);
	}

	.divider--top-left {
		top: 0;
		left: 0;
		cursor: var(--cursor-nw-resize);
	}

	.divider--top-right {
		top: 0;
		right: 0;
		cursor: var(--cursor-ne-resize);
	}

	.divider--bottom-left {
		bottom: 0;
		left: 0;
		cursor: var(--cursor-sw-resize);
	}

	.divider--bottom-right {
		bottom: 0;
		right: 0;
		cursor: var(--cursor-se-resize);
	}
</style>