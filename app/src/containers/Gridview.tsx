import React, { useState, useEffect, useRef } from 'react';
import './Gridview.scss';

const EPSILON = 0.1;

type Neightbor = number[] | null;
type Corner = 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right';

interface View {
	top: number,
	right: number,
	bottom: number,
	left: number,
	minWidth?: number,
	minHeight?: number,
}

interface ViewState extends View {
	neightbors: [Neightbor, Neightbor, Neightbor, Neightbor],
	ref: React.RefObject<HTMLDivElement>,
	elem: React.ReactElement,
	props: any,
}

interface GridviewInternal {
	bounds: DOMRect,
	views: ViewState[],
	dragging: boolean,
	subdividing: boolean,
	limits: DOMRect,
	axe: 'horizontal' | 'vertical',
	left: Set<number>,
	right: Set<number>,
	corner?: Corner,
	subdividingOriginal?: number,
	subdividingOther?: number,
	lastPointerEvent?: PointerEvent,
	trottledPointerEvent?: PointerEvent,
}

interface GridviewState {
	views: ViewState[],
}

export default function () {
	const gridviewRef = useRef<HTMLDivElement>(null);
	const internal = useRef<GridviewInternal>({
		bounds: new DOMRect(),
		views: [],
		dragging: false,
		subdividing: false,
		limits: new DOMRect(),
		axe: 'horizontal',
		left: new Set(),
		right: new Set()
	});
	const viewsProps: View[] = [{
		top: 0,
		right: 50,
		bottom: 66.6666,
		left: 0,
	}, {
		top: 0,
		right: 0,
		bottom: 50,
		left: 50,
	}, {
		top: 33.3333,
		bottom: 33.3333,
		right: 75,
		left: 0,
	}, {
		top: 66.6666,
		right: 50,
		bottom: 0,
		left: 0,
	}, {
		top: 50,
		right: 0,
		bottom: 0,
		left: 50,
	}, {
		top: 33.3333,
		bottom: 33.3333,
		right: 50,
		left: 25,
	}];
	const [state, setState] = useState<GridviewState>({
		views: viewsProps.map<ViewState>((view, id) => ({
			...view,
			neightbors: computeNeighbors(viewsProps, id),
			ref: React.createRef<HTMLDivElement>(),
			elem: <div>{id}</div>,
			props: {},
		}))
	});

	useEffect(() => {
		if (gridviewRef.current) {
			internal.current.bounds = gridviewRef.current.getBoundingClientRect();
		}

		function onResize(e: UIEvent) {
			if (gridviewRef.current) {
				internal.current.bounds = gridviewRef.current.getBoundingClientRect();
			}
		}

		function onLeave(e: PointerEvent) {
			if (internal.current.dragging) {
				const { x, y, width, height } = internal.current.bounds;
				const newViews = internal.current.views.map((view, id) => {
					const bounds = view.ref.current?.getBoundingClientRect();
					if (bounds) {
						view.top = (bounds.top - y) / height * 100;
						view.right = 100 - ((bounds.right - x) / width * 100);
						view.bottom = 100 - ((bounds.bottom - y) / height * 100);
						view.left = (bounds.left - x) / width * 100;
					}
					return { ...view };
				});
				setState({
					...state,
					views: newViews.map((view, id) => ({
						...view,
						neightbors: computeNeighbors(newViews, id)
					}))
				});

				internal.current = {
					bounds: internal.current.bounds,
					views: [],
					dragging: false,
					subdividing: false,
					limits: new DOMRect(),
					axe: 'horizontal',
					left: new Set(),
					right: new Set()
				};
			}
		}

		function onMove(e: PointerEvent) {
			const { dragging } = internal.current;
			if (dragging) {
				const { subdividing, lastPointerEvent, trottledPointerEvent } = internal.current;
				if (e.clientX === lastPointerEvent?.clientX && e.clientY === lastPointerEvent?.clientY) {
					return;
				}
				internal.current.lastPointerEvent = e;
				internal.current.trottledPointerEvent = trottledPointerEvent ?? e;

				if (subdividing !== false) {
					const { trottledPointerEvent } = internal.current;
					const deltaX = e.clientX - trottledPointerEvent.clientX;
					const deltaY = e.clientY - trottledPointerEvent.clientY;

					if (deltaX * deltaX + deltaY * deltaY >= 20 * 20) {
						const { corner, limits, subdividingOriginal, subdividingOther } = internal.current;
						internal.current.trottledPointerEvent = e;

						const axe = Math.abs(deltaX) > Math.abs(deltaY) ? 'horizontal' : 'vertical';

						if (axe !== internal.current.axe) {
							const view = internal.current.views[subdividingOriginal!];
							const otherView = internal.current.views[subdividingOther!];
							// view.top = otherView.top = limits.top;
							// view.right = otherView.right = 100 - limits.right;
							// view.bottom = otherView.bottom = limits.bottom;
							// view.left = otherView.left = limits.left;

							// view.ref.current!.style.top = otherView.ref.current!.style.top = limits.top + '%';
							// view.ref.current!.style.right = otherView.ref.current!.style.right = (100 - limits.right) + '%';
							// view.ref.current!.style.bottom = otherView.ref.current!.style.bottom = limits.bottom + '%';
							// view.ref.current!.style.left = otherView.ref.current!.style.left = limits.left + '%';

							console.log('subdividing', subdividingOriginal, subdividingOther, corner, axe, limits);
							if (axe === 'vertical') {
								// internal.current.views[subdividingOriginal!].neightbors[3] = internal.current.views[subdividingOther!].neightbors[3];
								// internal.current.views[subdividingOther!].neightbors[3] = internal.current.views[subdividingOriginal!].neightbors[1];
							} else {

							}

							// internal.current.axe = axe;

							// setState({
							// 	...state,
							// 	views: internal.current.views
							// });
						}
					}
				}

				const { bounds, axe, left, right, views } = internal.current;
				const { x: oX, y: oY, width, height } = bounds;
				const [x, y] = [e.clientX - oX, e.clientY - oY];
				const [pX, pY] = [x / width * 100, y / height * 100];
				const [cX, cY] = [
					Math.max(Math.min(pX, internal.current.limits.right), internal.current.limits.left),
					Math.max(Math.min(pY, internal.current.limits.bottom), internal.current.limits.top),
				];

				const C = axe === 'horizontal' ? cX : cY;
				const L = axe === 'horizontal' ? 'left' : 'top';
				const R = axe === 'horizontal' ? 'right' : 'bottom';

				for (let id of left) {
					const view = views[id];
					if (view.ref.current) {
						view.ref.current.style[R] = (100 - C) + '%';
					}
					// TODO W == 0, collapse and resize siblings
				}
				for (let id of right) {
					const view = views[id];
					if (view.ref.current) {
						view.ref.current.style[L] = (C) + '%';
					}
					// TODO W == 0, collapse and resize siblings
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
	}, []);

	const onSplitDown = (id: number, dir: number) => (e: React.PointerEvent) => {
		e.preventDefault();
		e.stopPropagation();

		const [left, right] = computeSplit(state.views, id, dir);

		internal.current.views = state.views;
		internal.current.dragging = true;
		internal.current.axe = dir % 2 ? 'horizontal' : 'vertical';
		internal.current.left = left;
		internal.current.right = right;
		internal.current.limits = computeLimits(state.views, id, dir);
	}

	const onSubdividerDown = (id:number, corner: Corner) => (e: React.PointerEvent) => {
		e.preventDefault();
		e.stopPropagation();
		const view = state.views[id];
		const otherId = state.views.length;
		const dir = corner === 'top-left' || corner === 'bottom-left' ? 3 : 1;

		const otherView: ViewState = {
			top: view.top,
			right: view.right,
			bottom: view.bottom,
			left: view.left,
			neightbors: view.neightbors.map(n => n?.concat() ?? null) as ViewState['neightbors'],
			ref: React.createRef<HTMLDivElement>(),
			elem: <div>{otherId}</div>,
			props: {}
		}
		
		if (corner === 'top-left' || corner === 'bottom-left') {
			otherView.neightbors[1] = [id];
			view.neightbors[3] = [otherId];
		} else {
			otherView.neightbors[3] = [id];
			view.neightbors[1] = [otherId];
		}

		const views = state.views.concat([otherView]);
		const [left, right] = computeSplit(views, id, dir);

		internal.current.views = views;
		internal.current.dragging = true;
		internal.current.subdividing = true;
		internal.current.axe = dir % 2 ? 'horizontal' : 'vertical';
		internal.current.left = left;
		internal.current.right = right;
		internal.current.limits = new DOMRect(view.left, view.top, 100 - view.right - view.left, 100 - view.bottom - view.top);
		internal.current.corner = corner;
		internal.current.subdividingOriginal = id;
		internal.current.subdividingOther = otherId;

		setState({ ...state, views });
	}

	const splits = state.views.map((view, id) => view.neightbors.map((neighbors, dir) => {
		if (neighbors === null || dir === 0 || dir === 3) {
			return undefined;
		}
		const axe = dir % 2 ? 'horizontal' : 'vertical';
		const styles: React.CSSProperties = {};
		if (dir === 0) {
			styles.top = `${view.top}%`;
			styles.left = `${view.left}%`;
			styles.width = `${(100 - view.right) - view.left}%`;
			styles.height = 'var(--split-size)';
		} else if (dir === 1) {
			styles.top = `${view.top}%`;
			styles.left = `${100 - view.right}%`;
			styles.width = 'var(--split-size)';
			styles.height = `${(100 - view.bottom) - view.top}%`;
		} else if (dir === 2) {
			styles.top = `${view.top + (100 - view.bottom) - view.top}%`;
			styles.left = `${view.left}%`;
			styles.width = `${(100 - view.right) - view.left}%`;
			styles.height = 'var(--split-size)';
		} else if (dir === 3) {
			styles.top = `${view.top}%`;
			styles.left = `${view.left}%`;
			styles.width = 'var(--split-size)';
			styles.height = `${(100 - view.bottom) - view.top}%`;
		}
		return <div
			key={`split-${id}-${dir}`}
			data-key={`split-${id}-${dir}`}
			className={`gridview-handle-split gridview-handle-split--${axe}`}
			style={styles}
			onPointerDown={onSplitDown(id, dir)}
		/>;
	}));

	const subdividers = state.views.map((view, id) => <div key={`subdivider-${id}`} className="gridview-handle" style={{
		top: `${view.top}%`,
		right: `${view.right}%`,
		bottom: `${view.bottom}%`,
		left: `${view.left}%`,
	}}>
		<div key="top-left" className={`gridview-handle-subdivider gridview-handle-subdivider--top-left`} onPointerDown={onSubdividerDown(id, 'top-left')} />
		<div key="top-right" className={`gridview-handle-subdivider gridview-handle-subdivider--top-right`} onPointerDown={onSubdividerDown(id, 'top-right')} />
		<div key="bottom-left" className={`gridview-handle-subdivider gridview-handle-subdivider--bottom-left`} onPointerDown={onSubdividerDown(id, 'bottom-left')} />
		<div key="bottom-right" className={`gridview-handle-subdivider gridview-handle-subdivider--bottom-right`} onPointerDown={onSubdividerDown(id, 'bottom-right')} />
	</div>);

	const views = state.views.map((view, id) => <div
		ref={view.ref}
		key={`view-${id}`}
		className="gridview-view-container-view"
		style={{
			top: `${view.top}%`,
			right: `${view.right}%`,
			bottom: `${view.bottom}%`,
			left: `${view.left}%`,
			borderWidth: view.neightbors.map((neighbors, dir) => neighbors === null ? `var(--border-size)` : (dir === 1 || dir === 2 ? `var(--border-size)` : 0)).join(' ')
		}}
	>
		{view.elem}
	</div>);

	return (
		<div className="gridview" ref={gridviewRef}>
			{splits.length > 0 && <div className="gridview-split-container">
				{splits}
			</div>}
			<div className="gridview-subdivider-container">
				{subdividers}
			</div>
			<div className="gridview-view-container">
				{views}
			</div>
		</div>
	);
}

function computeLimits(views: ViewState[], id: number, dir: number): DOMRect {
	const rect = { top: -1, right: 0, bottom: -1, left: 0 };
	const [left, right] = computeSplit(views, id, dir);

	{
		const op = dir === 0 || dir === 3 ? 'min' : 'max';
		const side = dir === 0 ? 'bottom' : (dir === 1 ? 'left' : (dir === 2 ? 'top' : 'right'));
		for (const neighbor of left) {
			rect[dir % 2 ? 'left' : 'top'] = Math[op](rect[side], views[neighbor][side]);
		}
	}
	{
		const op = dir === 0 || dir === 3 ? 'min' : 'max';
		const side = dir === 0 ? 'top' : (dir === 1 ? 'right' : (dir === 2 ? 'bottom' : 'left'));
		for (const neighbor of right) {
			rect[dir % 2 ? 'right' : 'bottom'] = Math[op](rect[side], views[neighbor][side]);
		}
	}

	return new DOMRect(rect.left, rect.top, 100 - rect.right - rect.left, 100 - rect.bottom - rect.top);
}

function computeSplit(views: ViewState[], id: number, dir: number): [Set<number>, Set<number>] {
	const left = new Set<number>();
	const right = new Set<number>();
	const visited = new Set<number>();
	const visit: [number, number][] = [[id, dir]];
	for (let next = visit.pop(); next !== undefined; next = visit.pop()) {
		const [i, dir] = next;
		if (dir === 0 || dir === 3) {
			right.add(i);
		} else {
			left.add(i);
		}
		visited.add(i);
		
		const neighbors = views[i].neightbors[dir];
		if (neighbors) {
			for (let neighbor of neighbors) {
				if (!visited.has(neighbor)) {
					visit.push([neighbor, (dir + 2) % 4]);
				}
			}
		}
	}

	return [left, right];
}

function computeNeighbors(views: View[], id: number): [Neightbor, Neightbor, Neightbor, Neightbor] {
	if (id >= views.length) {
		return [null, null, null, null];
	}

	const view = views[id];

	const vMinX = view.left;
	const vMinY = view.top;
	const vMaxX = 100 - view.right;
	const vMaxY = 100 - view.bottom;

	const neighbors: [Neightbor, Neightbor, Neightbor, Neightbor] = [
		view.top === 0 ? null : [],
		view.right === 0 ? null : [],
		view.bottom === 0 ? null : [],
		view.left === 0 ? null : []
	];

	for (let v = 0, l = views.length; v < l; ++v) {
		if (v !== id) {
			const other = views[v];
			const oMinX = other.left;
			const oMinY = other.top;
			const oMaxX = 100 - other.right;
			const oMaxY = 100 - other.bottom;

			// TOP
			if (Math.abs(vMinY - oMaxY) < EPSILON && segmentIntersect(Math.min(vMinX, vMaxX), Math.max(vMinX, vMaxX), Math.min(oMinX, oMaxX), Math.max(oMinX, oMaxX))) {
				neighbors[0]!.push(v);
			}
			// RIGHT
			if (Math.abs(vMaxX - oMinX) < EPSILON && segmentIntersect(Math.min(vMinY, vMaxY), Math.max(vMinY, vMaxY), Math.min(oMinY, oMaxY), Math.max(oMinY, oMaxY))) {
				neighbors[1]!.push(v);
			}
			// BOTTOM
			if (Math.abs(vMaxY - oMinY) < EPSILON && segmentIntersect(Math.min(vMinX, vMaxX), Math.max(vMinX, vMaxX), Math.min(oMinX, oMaxX), Math.max(oMinX, oMaxX))) {
				neighbors[2]!.push(v);
			}
			// LEFT
			if (Math.abs(vMinX - oMaxX) < EPSILON && segmentIntersect(Math.min(vMinY, vMaxY), Math.max(vMinY, vMaxY), Math.min(oMinY, oMaxY), Math.max(oMinY, oMaxY))) {
				neighbors[3]!.push(v);
			}
		}
	}

	return neighbors;
}

function segmentIntersect(x1: number, x2: number, y1: number, y2: number): boolean {
	return x2 >= y1 && y2 >= x1;
}