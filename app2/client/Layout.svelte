<script lang="ts">
	import type { Corner, PaneProps, Axe } from './helpers/Layout';
	import { Layout } from './helpers/Layout';
	import { nanoid } from 'nanoid';
	import { onMount } from 'svelte';

	export let panes: PaneProps[];

	let layoutRef: HTMLDivElement | undefined;
	let bounds = new DOMRect();
	let dragging = false;
	let dragLeft: string | undefined;
	let dragRight: string | undefined;
	let dragCorner: Corner | undefined;
	let dragBounds = new DOMRect();
	let dragSiblings = false;
	let divId: number | undefined;
	let divAxe: Axe | undefined;
	let lastPointerEvent: PointerEvent | undefined;
	let trottledPointerEvent: PointerEvent | undefined;
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
			dragging = false;

			const newPanes = panes.reduce((panes, pane) => {
				const layoutPane = layout.panes.find(p => p.id === pane.key);
				if (layoutPane && layoutPane.width > 0 && layoutPane.height > 0) {
					pane.top = layoutPane.top;
					pane.right = layoutPane.right;
					pane.bottom = layoutPane.bottom;
					pane.left = layoutPane.left;
					panes.push(pane);
				}
				return panes;
			}, [] as PaneProps[]);

			console.log('onChange', newPanes);
			// TODO onChange && onChange(newPanes);
		}
	}

	function onMove(e: PointerEvent) {
		if (dragging) {
			if (e.clientX === lastPointerEvent?.clientX && e.clientY === lastPointerEvent?.clientY) {
				return;
			}
			lastPointerEvent = e;
			trottledPointerEvent = trottledPointerEvent ?? e;

			const { x: oX, y: oY, width, height } = bounds;
			const [x, y] = [e.clientX - oX, e.clientY - oY];
			const [pX, pY] = [x / width * 100, y / height * 100];
			const [cX, cY] = [
				Math.max(Math.min(pX, dragBounds.right), dragBounds.left),
				Math.max(Math.min(pY, dragBounds.bottom), dragBounds.top),
			];

			if (dragCorner !== undefined) {
				const deltaX = e.clientX - trottledPointerEvent.clientX;
				const deltaY = e.clientY - trottledPointerEvent.clientY;

				if (deltaX * deltaX + deltaY * deltaY >= 20 * 20) {
					trottledPointerEvent = e;

					const axe = Math.abs(deltaX) > Math.abs(deltaY) ? 'vertical' : 'horizontal';

					if (axe !== divAxe) {
						divAxe = axe;

						const newPanes = panes.reduce((panes, pane) => {
							const newPane = { ...pane };
							if (newPane.key === dragLeft || newPane.key === dragRight) {
								newPane.top = dragBounds.top;
								newPane.right = dragBounds.right;
								newPane.bottom = dragBounds.bottom;
								newPane.left = dragBounds.left;

								if (axe === 'horizontal') {
									newPane[newPane.key === dragLeft ? 'bottom' : 'top'] = cY;
								} else {
									newPane[newPane.key === dragLeft ? 'right' : 'left'] = cX;
								}
							}
							panes.push(newPane);
							return panes;
						}, [] as PaneProps[]);

						console.log('onChange', newPanes);
						// TODO : onChange && onChange(newPanes);
						return;
					}
				}
			}

			if (dragLeft !== undefined && dragRight !== undefined) {
				const edge = layout.edges.find(e => e.left.id === dragLeft && e.right.id === dragRight);
				if (edge) {
					const p = edge.axe === 'horizontal' ? cY : cX;
					
					edge.p = p;
					edge.updateDOM();

					if (dragSiblings) {
						for (const sibling of edge.allSiblings()) {
							sibling.p = p;
							sibling.updateDOM();
						}
					}

					const newPanes = panes.map(pane => {
						const newPane = { ...pane };
						const layoutPane = layout.panes.find(p => p.id === pane.key);
						if (layoutPane) {
							newPane.top = layoutPane.top;
							newPane.right = layoutPane.right;
							newPane.bottom = layoutPane.bottom;
							newPane.left = layoutPane.left;
						}
						return newPane
					});

					console.log('onDragging', newPanes);
					// TODO
					// onDragging && onDragging(panes.map(pane => {
					// 	const newPane = { ...pane };
					// 	const layoutPane = layout.panes.find(p => p.id === pane.key);
					// 	if (layoutPane) {
					// 		newPane.top = layoutPane.top;
					// 		newPane.right = layoutPane.right;
					// 		newPane.bottom = layoutPane.bottom;
					// 		newPane.left = layoutPane.left;
					// 	}
					// 	return newPane
					// }));
				}
			}
		}
	}

	function onResizerDown(id: number) {
		return (e: PointerEvent) => {
			e.preventDefault();
			e.stopPropagation();

			const edge = layout.edges[id];
			let minX: number = 0;
			let maxX: number = 100;
			let minY: number = 0;
			let maxY: number = 100;
			const breakable = edge.axe === 'horizontal'
				? Math.abs(edge.left.left - edge.right.left) < 0.1 && Math.abs(edge.left.right - edge.right.right) < 0.1
				: Math.abs(edge.left.top - edge.right.top) < 0.1 && Math.abs(edge.left.bottom - edge.right.bottom) < 0.1;

			for (const sibiling of [edge].concat(edge.allSiblings())) {
				minX = Math.max(minX, sibiling.left.left);
				maxX = Math.min(maxX, sibiling.right.right);
				minY = Math.max(minY, sibiling.left.top);
				maxY = Math.min(maxY, sibiling.right.bottom);
			}

			dragging = true;
			dragLeft = edge.left.id;
			dragRight = edge.right.id;
			dragBounds = new DOMRect(minX, minY, maxX - minX, maxY - minY);
			dragSiblings = !(breakable && e.ctrlKey);
		}
	}

	function onDividerDown(id: number, corner: Corner) {
		return (e: PointerEvent) => {
			e.preventDefault();
			e.stopPropagation();

			const pane = layout.panes[id];

			const paneProps: PaneProps = panes.find(p => p.key === pane.id)!;
			const newPaneProps: PaneProps = { ...paneProps };
			newPaneProps.key = nanoid();

			dragLeft = pane.id;
			dragRight = newPaneProps.key;

			if (corner === 'top-left' || corner === 'bottom-left') {
				let t = dragLeft;
				dragLeft = dragRight;
				dragRight = t;
				paneProps.left += 1;
				newPaneProps.right = paneProps.left;
			} else {
				paneProps.right -= 1;
				newPaneProps.left = paneProps.right;
			}

			dragging = true,
			dragCorner = corner;
			dragBounds = new DOMRect(pane.left, pane.top, pane.right - pane.left, pane.bottom - pane.top);
			divAxe = 'vertical';

			console.log('onChange', panes.concat([newPaneProps]));
			// TODO onChange && onChange(panes.concat([newPaneProps]));
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