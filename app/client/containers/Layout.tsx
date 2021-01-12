import React, { createContext, useEffect, useRef, useMemo } from 'react';
import { nanoid } from 'nanoid';
import './Layout.scss';

export interface PaneProps {
	key: string,
	top: number,
	right: number,
	bottom: number,
	left: number,
	minWidth?: number,
	minHeight?: number,
	elem: React.ReactElement,
	props?: any,
}

export const PaneContext = createContext<PaneProps>({
	key: '',
	top: 0,
	right: 0,
	bottom: 0,
	left: 0,
	elem: <></>
});

export interface LayoutProps {
	panes: PaneProps[],
	onChange?: (panes: PaneProps[]) => void,
	onDragging?: (panes: PaneProps[]) => void,
}

type Axe = 'horizontal' | 'vertical';

interface Internal {
	bounds: DOMRect,
	dragging: boolean,
	dragLeft?: string,
	dragRight?: string,
	dragCorner?: Corner,
	dragBounds: DOMRect,
	dragSiblings?: boolean,
	divId?: number,
	divAxe?: Axe,
	lastPointerEvent?: PointerEvent,
	trottledPointerEvent?: PointerEvent,
}

export default function({ panes, onChange, onDragging }: React.PropsWithChildren<LayoutProps>) {
	const layoutRef = useRef<HTMLDivElement>(null);
	const internal = useRef<Internal>({
		bounds: new DOMRect(),
		dragging: false,
		dragBounds: new DOMRect(),
	});
	const layout = useMemo<Layout>(
		() => new Layout(panes),
		[panes]
	);

	useEffect(() => {
		internal.current.bounds = layoutRef.current?.getBoundingClientRect()!;

		function onResize(_: UIEvent) {
			internal.current!.bounds = layoutRef.current?.getBoundingClientRect()!;
		}

		function onLeave(e: PointerEvent) {
			if (internal.current?.dragging) {
				internal.current = {
					...internal.current!,
					dragging: false,
					dragLeft: undefined,
					dragRight: undefined,
					dragCorner: undefined,
					dragSiblings: undefined,
					divId: undefined,
					divAxe: undefined,
				};

				const newPanes = panes.reduce((panes, pane) => {
					const layoutPane = layout.panes.find(p => p.id === pane.key);
					// Remove missing pane or collapsed pane
					if (layoutPane && layoutPane.width > 0 && layoutPane.height > 0) {
						pane.top = layoutPane.top;
						pane.right = layoutPane.right;
						pane.bottom = layoutPane.bottom;
						pane.left = layoutPane.left;
						panes.push(pane);
					}
					return panes;
				}, [] as PaneProps[]);


				onChange && onChange(newPanes);
			}
		}

		function onMove(e: PointerEvent) {
			if (internal.current?.dragging === true) {
				const { bounds, dragLeft, dragRight, dragBounds, dragCorner, dragSiblings, divAxe, lastPointerEvent, trottledPointerEvent } = internal.current;

				if (e.clientX === lastPointerEvent?.clientX && e.clientY === lastPointerEvent?.clientY) {
					return;
				}
				internal.current.lastPointerEvent = e;
				internal.current.trottledPointerEvent = trottledPointerEvent ?? e;

				const { x: oX, y: oY, width, height } = bounds;
				const [x, y] = [e.clientX - oX, e.clientY - oY];
				const [pX, pY] = [x / width * 100, y / height * 100];
				const [cX, cY] = [
					Math.max(Math.min(pX, dragBounds.right), dragBounds.left),
					Math.max(Math.min(pY, dragBounds.bottom), dragBounds.top),
				];

				if (dragCorner !== undefined) {
					const { trottledPointerEvent } = internal.current;
					const deltaX = e.clientX - trottledPointerEvent.clientX;
					const deltaY = e.clientY - trottledPointerEvent.clientY;

					if (deltaX * deltaX + deltaY * deltaY >= 20 * 20) {
						internal.current.trottledPointerEvent = e;

						const axe = Math.abs(deltaX) > Math.abs(deltaY) ? 'vertical' : 'horizontal';

						if (axe !== divAxe) {
							internal.current.divAxe = axe;

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

							onChange && onChange(newPanes);
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
							for (const sibling of edge.siblings) {
								sibling.p = p;
								sibling.updateDOM();
							}
						}

						onDragging && onDragging(panes.map(pane => {
							const newPane = { ...pane };
							const layoutPane = layout.panes.find(p => p.id === pane.key);
							if (layoutPane) {
								newPane.top = layoutPane.top;
								newPane.right = layoutPane.right;
								newPane.bottom = layoutPane.bottom;
								newPane.left = layoutPane.left;
							}
							return newPane
						}));
					}
				}
			}
		}

		window.addEventListener('resize', onResize);
		document.addEventListener('pointerup', onLeave);
		document.addEventListener('pointermove', onMove);
		return () => {
			window.removeEventListener('resize', onResize);
			document.removeEventListener('pointerup', onLeave);
			document.removeEventListener('pointermove', onMove);
		}
	}, [panes, onChange]);

	const onEdgeDown = (id: number) => (e: React.PointerEvent) => {
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

		for (const sibiling of [edge].concat(edge.siblings)) {
			minX = Math.max(minX, sibiling.left.left);
			maxX = Math.min(maxX, sibiling.right.right);
			minY = Math.max(minY, sibiling.left.top);
			maxY = Math.min(maxY, sibiling.right.bottom);
		}

		internal.current = {
			...internal.current!,
			dragging: true,
			dragLeft: edge.left.id,
			dragRight: edge.right.id,
			dragBounds: new DOMRect(minX, minY, maxX - minX, maxY - minY),
			dragSiblings: !(breakable && e.ctrlKey)
		};
	};

	const onDividerDown = (id: number, corner: Corner) => (e: React.PointerEvent) => {
		e.preventDefault();
		e.stopPropagation();

		const pane = layout.panes[id];

		const paneProps: PaneProps = panes.find(p => p.key === pane.id)!;
		const newPaneProps: PaneProps = { ...paneProps };
		newPaneProps.key = nanoid();

		let dragLeft = pane.id;
		let dragRight = newPaneProps.key;

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

		internal.current = {
			...internal.current!,
			dragging: true,
			dragLeft,
			dragRight,
			dragCorner: corner,
			dragBounds: new DOMRect(pane.left, pane.top, pane.right - pane.left, pane.bottom - pane.top),
			divAxe: 'vertical'
		};

		onChange && onChange(panes.concat([newPaneProps]));
	};

	return (<div className="layout" ref={layoutRef}>
		<div className="layout-edge-container">
			{layout.edges.map((edge, id) => {
		let styles: React.CSSProperties = {};
		if (edge.axe === 'horizontal') {
			const left = Math.max(edge.left.left, edge.right.left);
			const right = Math.min(edge.left.right, edge.right.right);
			styles.top = edge.p.toFixed(6) + '%';
			styles.left = left.toFixed(6) + '%';
			styles.width = (right - left).toFixed(6) + '%';
			styles.height = 'var(--edge-size)';
		} else {
			const top = Math.max(edge.left.top, edge.right.top);
			const bottom = Math.min(edge.left.bottom, edge.right.bottom);
			styles.top = top.toFixed(6) + '%';
			styles.left = edge.p.toFixed(6) + '%';
			styles.width = 'var(--edge-size)';
			styles.height = (bottom - top).toFixed(6) + '%';
		}

		return <div
			ref={edge.ref}
			key={`edge-${edge.left.id}-${edge.right.id}`}
			className={`layout-handle-edge layout-handle-edge--${edge.axe}`}
			style={styles}
			onPointerDown={onEdgeDown(id)}
		/>;
	})}
		</div>
		<div className="layout-divider-container">
			{layout.panes.map((pane, id) => <div
		ref={pane.dividerRef}
		key={`divider-${pane.id}`}
		className="layout-handle"
		style={{
			top: `${pane.top.toFixed(6)}%`,
			right: `${(100 - pane.right).toFixed(6)}%`,
			bottom: `${(100 - pane.bottom).toFixed(6)}%`,
			left: `${(pane.left).toFixed(6)}%`,
			borderWidth: pane.links.map((link, dir) => link ? (dir === 1 || dir === 2 ? `var(--border-size)` : 0) : `var(--border-size)`).join(' '),
		}}
	>
		<div key="top-left" className={`layout-handle-divider layout-handle-divider--top-left`} onPointerDown={onDividerDown(id, 'top-left')} />
		<div key="top-right" className={`layout-handle-divider layout-handle-divider--top-right`} onPointerDown={onDividerDown(id, 'top-right')} />
		<div key="bottom-left" className={`layout-handle-divider layout-handle-divider--bottom-left`} onPointerDown={onDividerDown(id, 'bottom-left')} />
		<div key="bottom-right" className={`layout-handle-divider layout-handle-divider--bottom-right`} onPointerDown={onDividerDown(id, 'bottom-right')} />
	</div>)}
		</div>
		<div className="layout-view-container">
			{layout.panes.map((pane, id) => <div
				ref={pane.paneRef}
				key={`pane-${pane.id}`}
				className="layout-view-container-view"
				style={{
					top: `${pane.top.toFixed(6)}%`,
					right: `${(100 - pane.right).toFixed(6)}%`,
					bottom: `${(100 - pane.bottom).toFixed(6)}%`,
					left: `${pane.left.toFixed(6)}%`,
					// borderWidth: pane.links.map((link, dir) => link.length ? (dir === 1 || dir === 2 ? `var(--border-size)` : 0) : `var(--border-size)`).join(' '),
					['--pane-top-neighbor' as any]: pane.links[0].length ? 1 : 0,
					['--pane-right-neighbor' as any]: pane.links[1].length ? 1 : 0,
					['--pane-bottom-neighbor' as any]: pane.links[2].length ? 1 : 0,
					['--pane-left-neighbor' as any]: pane.links[3].length ? 1 : 0,
				}}
			>
				<PaneContext.Provider value={pane.props}>
					{pane.props.elem}
				</PaneContext.Provider>
			</div>)}
		</div>
	</div>);
}

const { abs, min, max } = Math;

class Layout {

	public edges: Edge[];
	public panes: Pane[];

	constructor(panesProps: PaneProps[]) {
		// Find neighbors
		const neighbors: [number[], number[], number[], number[]][] = [];
		for (let a = 0, b = panesProps.length; a < b; ++a) {
			const pane = panesProps[a];
			const neighbor: [number[], number[], number[], number[]] = [[], [], [], []];
			const pLeft = min(pane.left, pane.right);
			const pRight = max(pane.left, pane.right);
			const pTop = min(pane.top, pane.bottom);
			const pBottom = max(pane.top, pane.bottom);
			for (let c = 0, d = panesProps.length; c < d; ++c) {
				if (a !== c) {
					const other = panesProps[c];
					const oLeft = min(other.left, other.right);
					const oRight = max(other.left, other.right);
					const oTop = min(other.top, other.bottom);
					const oBottom = max(other.top, other.bottom);

					// TOP
					if (abs(pane.top - other.bottom) <= 0.1 && segmentIntersect(pLeft, pRight, oLeft, oRight)) {
						neighbor[0].push(c);
					}
					// RIGHT
					else if (abs(pane.right - other.left) <= 0.1 && segmentIntersect(pTop, pBottom, oTop, oBottom)) {
						neighbor[1].push(c);
					}
					// BOTTOM
					else if (abs(pane.bottom - other.top) <= 0.1 && segmentIntersect(pLeft, pRight, oLeft, oRight)) {
						neighbor[2].push(c);
					}
					// LEFT
					else if (abs(pane.left - other.right) <= 0.1 && segmentIntersect(pTop, pBottom, oTop, oBottom)) {
						neighbor[3].push(c);
					}
				}
			}
			neighbors.push(neighbor);
		}

		// Create edges
		const edges = neighbors.reduce((edges, neighbors, a) => {
			// RIGHT, BOTTOM only
			for (let dir = 1; dir < 3; ++dir) {
				for (const b of neighbors[dir]) {
					const axe = dir % 2 ? 'vertical' : 'horizontal';
					const pos = dir % 2 ? panesProps[a].right : panesProps[a].bottom;
					const key = `${Math.min(a, b)}-${Math.max(a, b)}`;
					if (!edges.has(key)) {
						if (pos === 0 || pos === 100) {
							debugger;
						}
						edges.set(key, new Edge(axe, pos, null!, null!, []));
					}
				}
			}
			return edges;
		}, new Map<string, Edge>());

		for (const [_, edge] of edges) {
			for (const [_, other] of edges) {
				if (edge !== other && edge.axe === other.axe && Math.abs(edge.p - other.p) < 0.1) {
					edge.siblings.push(other);
				}
			}
		}

		// Create panes
		const panes = neighbors.reduce((panes, neighbors, a) => {
			const pane = new Pane(panesProps[a].key, [[], [], [], []], panesProps[a]);
			pane.links = neighbors.map((neighbors, dir) => {
				return neighbors.reduce((links, b) => {
					const key = `${Math.min(a, b)}-${Math.max(a, b)}`;
					if (edges.has(key)) {
						const edge = edges.get(key)!;
						edge[dir === 1 || dir === 2 ? 'left' : 'right'] = pane;
						links.push(edge);
					}
					return links;
				}, [] as Edge[]);
			}) as Links;
			panes.push(pane);
			return panes;
		}, [] as Pane[]);

		this.edges = Array.from(edges.values());
		this.panes = panes;
	}

	public dispose() {
		for (const edge of this.edges) {
			edge.dispose();
		}
		for (const pane of this.panes) {
			pane.dispose();
		}
		this.edges = undefined!;
		this.panes = undefined!;
	}
}

type Links = [Edge[], Edge[], Edge[], Edge[]];
type Corner = 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right';

class Edge {
	public ref: React.RefObject<HTMLDivElement>;

	constructor(
		public axe: Axe,
		public p: number,
		public left: Pane,
		public right: Pane,
		public siblings: Edge[],
	) {
		this.ref = React.createRef();
	}

	public dispose() {
		this.ref = undefined!;
		this.left = undefined!;
		this.right = undefined!;
		this.siblings = undefined!;
	}

	public updateDOM() {
		this.left.updateDOM();
		this.right.updateDOM();
		if (this.ref.current) {
			if (this.axe === 'horizontal') {
				const left = Math.max(this.left.left, this.right.left);
				const right = Math.min(this.left.right, this.right.right);
				this.ref.current.style.top = this.p.toFixed(6) + '%';
				this.ref.current.style.left = left.toFixed(6) + '%';
				this.ref.current.style.width = (right - left).toFixed(6) + '%';
				this.ref.current.style.height = 'var(--edge-size)';
			} else {
				const top = Math.max(this.left.top, this.right.top);
				const bottom = Math.min(this.left.bottom, this.right.bottom);
				this.ref.current.style.top = top.toFixed(6) + '%';
				this.ref.current.style.left = this.p.toFixed(6) + '%';
				this.ref.current.style.width = 'var(--edge-size)';
				this.ref.current.style.height = (bottom - top).toFixed(6) + '%';
			}
		}
		for (const sibling of this.siblings) {
			sibling.left.updateDOM();
			sibling.right.updateDOM();
		}
	}
}

class Pane {
	public paneRef: React.RefObject<HTMLDivElement>;
	public dividerRef: React.RefObject<HTMLDivElement>;

	constructor(
		public id: string,
		public links: Links,
		public props: PaneProps,
	) {
		this.paneRef = React.createRef();
		this.dividerRef = React.createRef();
	}

	public dispose() {
		this.paneRef = undefined!;
		this.dividerRef = undefined!;
		this.links = undefined!;
		this.props = undefined!;
	}

	public updateDOM() {
		for (const ref of [this.paneRef, this.dividerRef]) {
			if (ref.current) {
				ref.current.style.top = `${this.top}%`;
				ref.current.style.right = `${100 - this.right}%`;
				ref.current.style.bottom = `${100 - this.bottom}%`;
				ref.current.style.left = `${this.left}%`;
			}
		}
	}

	get top(): number {
		return this.links[0].length ? this.links[0][0].p : 0;
	}

	get right(): number {
		return this.links[1].length ? this.links[1][0].p : 100;
	}

	get bottom(): number {
		return this.links[2].length ? this.links[2][0].p : 100;
	}

	get left(): number {
		return this.links[3].length ? this.links[3][0].p : 0;
	}

	get width(): number {
		return this.right - this.left;
	}

	get height(): number {
		return this.bottom - this.top;
	}
}

function segmentIntersect(x1: number, x2: number, y1: number, y2: number): boolean {
	return x2 > y1 && y2 > x1;
}