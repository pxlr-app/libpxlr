import type { SvelteComponent } from "svelte";

export interface PaneProps {
	key: string,
	top: number,
	right: number,
	bottom: number,
	left: number,
	minWidth?: number,
	minHeight?: number,
	elem: typeof SvelteComponent,
	props?: Record<string, any>,
}

const { abs, min, max } = Math;
export class Layout {

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
						// if (pos === 0 || pos === 100) {
						// 	debugger;
						// }
						const start = dir % 2 ? Math.max(panesProps[a].top, panesProps[b].top) : Math.max(panesProps[a].left, panesProps[b].left);
						const end = dir % 2 ? Math.min(panesProps[a].bottom, panesProps[b].bottom) : Math.min(panesProps[a].right, panesProps[b].right);
						edges.set(key, new Edge(axe, start, end, pos, null!, null!, []));
					}
				}
			}
			return edges;
		}, new Map<string, Edge>());

		for (const [_, edge] of edges) {
			for (const [_, other] of edges) {
				if (
					edge !== other &&
					edge.axe === other.axe &&
					Math.abs(edge.p - other.p) < 0.1 &&
					(
						Math.abs(edge.s - other.e) < 0.1 ||
						Math.abs(edge.e - other.s) < 0.1
					)
				) {
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

export type Axe = 'horizontal' | 'vertical';
export type Links = [Edge[], Edge[], Edge[], Edge[]];
export type Corner = 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right';

export class Edge {
	public ref: HTMLDivElement | undefined;

	constructor(
		public axe: Axe,
		public s: number,
		public e: number,
		public p: number,
		public left: Pane,
		public right: Pane,
		public siblings: Edge[],
	) { }

	public get id(): String {
		return `${this.left.id}-${this.right.id}`;
	}

	public allSiblings(siblings: Set<Edge> = new Set()) {
		for (const sibling of this.siblings) {
			if (!siblings.has(sibling)) {
				siblings.add(sibling);
				sibling.allSiblings(siblings);
			}
		}
		return Array.from(siblings);
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
		if (this.ref) {
			if (this.axe === 'horizontal') {
				this.ref.style.top = this.p.toFixed(6) + '%';
				this.ref.style.left = this.s.toFixed(6) + '%';
				this.ref.style.width = (this.e - this.s).toFixed(6) + '%';
				this.ref.style.height = 'var(--edge-size)';
			} else {
				this.ref.style.top = this.s.toFixed(6) + '%';
				this.ref.style.left = this.p.toFixed(6) + '%';
				this.ref.style.width = 'var(--edge-size)';
				this.ref.style.height = (this.e - this.s).toFixed(6) + '%';
			}
		}
		for (const sibling of this.allSiblings()) {
			sibling.left.updateDOM();
			sibling.right.updateDOM();
		}
	}
}

export class Pane {
	public paneRef: HTMLDivElement | undefined;
	public dividerRef: HTMLDivElement | undefined;

	constructor(
		public id: string,
		public links: Links,
		public props: PaneProps,
	) { }

	public dispose() {
		this.paneRef = undefined!;
		this.dividerRef = undefined!;
		this.links = undefined!;
		this.props = undefined!;
	}

	public updateDOM() {
		for (const ref of [this.paneRef, this.dividerRef]) {
			if (ref) {
				ref.style.top = `${this.top}%`;
				ref.style.right = `${100 - this.right}%`;
				ref.style.bottom = `${100 - this.bottom}%`;
				ref.style.left = `${this.left}%`;
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