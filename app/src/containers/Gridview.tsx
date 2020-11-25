import React, { useState, useEffect, useRef } from 'react';
import './Gridview.scss';

type Neightbor = number[] | null;
type Corner = 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right';

interface View {
	x: number,
	y: number,
	width: number,
	height: number,
	minWidth?: number,
	minHeight?: number,
	ref: React.RefObject<HTMLDivElement>,
	neightbors: [Neightbor, Neightbor, Neightbor, Neightbor],
	elem: React.ReactElement,
	props: any,
}

interface GridviewInternal {
	views: View[],
	dragging: boolean,
	axe: 'horizontal' | 'vertical',
	left: number[],
	right: number[],
	bounds: DOMRect
}

interface GridviewState {
	views: View[],
}

export default function () {
	const gridviewRef = useRef<HTMLDivElement>(null);
	const internal = useRef<GridviewInternal>({
		views: [],
		dragging: false,
		axe: 'horizontal',
		left: [],
		right: [],
		bounds: new DOMRect()
	});
	const [state, setState] = useState<GridviewState>({
		views: [{
			x: 0,
			y: 0,
			width: 50,
			height: 33.3333,
			ref: useRef<HTMLDivElement>(null),
			neightbors: [null, [1], [2], null],
			elem: <div>0</div>,
			props: {},
		}, {
			x: 50,
			y: 0,
			width: 50,
			height: 50,
			ref: useRef<HTMLDivElement>(null),
			neightbors: [null, null, [4], [0, 2]],
			elem: <div>1</div>,
			props: {},
		}, {
			x: 0,
			y: 33.3333,
			width: 50,
			height: 33.3333,
			ref: useRef<HTMLDivElement>(null),
			neightbors: [[0], [1, 4], [3], null],
			elem: <div>2</div>,
			props: {},
		}, {
			x: 0,
			y: 66.6666,
			width: 50,
			height: 33.3333,
			ref: useRef<HTMLDivElement>(null),
			neightbors: [[2], [4], null, null],
			elem: <div>3</div>,
			props: {},
		}, {
			x: 50,
			y: 50,
			width: 50,
			height: 50,
			ref: useRef<HTMLDivElement>(null),
			neightbors: [[1], null, null, [2, 3]],
			elem: <div>4</div>,
			props: {},
		}]
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
				setState({
					...state,
					views: internal.current.views.map((view, id) => {
						const bounds = view.ref.current?.getBoundingClientRect();
						if (bounds) {
							view.x = (bounds.x - x) / width * 100;
							view.y = (bounds.y - y) / height * 100;
							view.width = bounds.width / width * 100;
							view.height = bounds.height / height * 100;
						}
						return { ...view };
					})
				});

				internal.current = {
					views: [],
					dragging: false,
					axe: 'horizontal',
					left: [],
					right: [],
					bounds: internal.current.bounds
				};
			}
		}

		function onMove(e: PointerEvent) {
			const { dragging } = internal.current;
			if (dragging) {
				const { bounds, axe, left, right, views } = internal.current;
				const { x: oX, y: oY, width, height } = bounds;
				const [x, y] = [e.clientX - oX, e.clientY - oY];
				const [pX, pY] = [x / width * 100, y / height * 100];

				const P = axe === 'horizontal' ? pX : pY;
				const X = axe === 'horizontal' ? 'x' : 'y';
				const W = axe === 'horizontal' ? 'width' : 'height';
				const L = axe === 'horizontal' ? 'left' : 'top';

				for (let id of left) {
					const view = views[id];
					if (view.ref.current) {
						view.ref.current.style[W] = Math.max(0, P - view[X]) + '%';
					}
				}
				for (let id of right) {
					const view = views[id];
					if (view.ref.current) {
						view.ref.current.style[L] = P + '%';
						view.ref.current.style[W] = view[X] + view[W] - P + '%';
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
	}, []);

	const onSplitDown = (id: number, dir: number) => (e: React.PointerEvent) => {
		e.preventDefault();
		e.stopPropagation();

		const left = new Set<number>();
		const right = new Set<number>();
		const visited = new Set<number>();
		const visit = [id];
		let swap = dir;
		for (let i = visit.pop(); i !== undefined; i = visit.pop(), swap = (swap + 2) % 4) {
			if (swap === 0 || swap === 3) {
				right.add(i);
			} else {
				left.add(i);
			}
			visited.add(i);
			
			const neighbors = state.views[i].neightbors[swap];
			if (neighbors) {
				for (let neighbor of neighbors) {
					if (!visited.has(neighbor)) {
						visit.push(neighbor);
					}
				}
			}
		}

		internal.current.views = state.views.map(view => ({ ...view }));
		internal.current.dragging = true;
		internal.current.axe = dir % 2 ? 'horizontal' : 'vertical';
		internal.current.left = Array.from(left);
		internal.current.right = Array.from(right);
	}

	const onSubdividerDown = (corner: Corner) => (e: React.PointerEvent) => {
		e.preventDefault();
		e.stopPropagation();
		// internal.current.views = state.views;
		// internal.current.dragging = true;
	}

	const splits = state.views.map((view, id) => view.neightbors.map((neighbors, dir) => {
		if (neighbors === null || dir === 0 || dir === 3) {
			return undefined;
		}
		const axe = dir % 2 ? 'horizontal' : 'vertical';
		const styles: React.CSSProperties = {};
		if (dir === 0) {
			styles.top = `${view.y}%`;
			styles.left = `${view.x}%`;
			styles.width = `${view.width}%`;
			styles.height = 'var(--split-size)';
		} else if (dir === 1) {
			styles.top = `${view.y}%`;
			styles.left = `${view.x + view.width}%`;
			styles.width = 'var(--split-size)';
			styles.height = `${view.height}%`;
		} else if (dir === 2) {
			styles.top = `${view.y + view.height}%`;
			styles.left = `${view.x}%`;
			styles.width = `${view.width}%`;
			styles.height = 'var(--split-size)';
		} else if (dir === 3) {
			styles.top = `${view.y}%`;
			styles.left = `${view.x}%`;
			styles.width = 'var(--split-size)';
			styles.height = `${view.height}%`;
		}
		return <div
			key={`split-${id}-${dir}`}
			className={`gridview-handle-split gridview-handle-split--${axe}`}
			style={styles}
			onPointerDown={onSplitDown(id, dir)}
		/>;
	}));

	const subdividers = state.views.map((view, id) => <div key={`subdivider-${id}`} className="gridview-handle" style={{
		left: `${view.x}%`,
		top: `${view.y}%`,
		width: `${view.width}%`,
		height: `${view.height}%`,
	}}>
		<div key="top-left" className={`gridview-handle-subdivider gridview-handle-subdivider--top-left`} onPointerDown={onSubdividerDown('top-left')} />
		<div key="top-right" className={`gridview-handle-subdivider gridview-handle-subdivider--top-right`} onPointerDown={onSubdividerDown('top-right')} />
		<div key="bottom-left" className={`gridview-handle-subdivider gridview-handle-subdivider--bottom-left`} onPointerDown={onSubdividerDown('bottom-left')} />
		<div key="bottom-right" className={`gridview-handle-subdivider gridview-handle-subdivider--bottom-right`} onPointerDown={onSubdividerDown('bottom-right')} />
	</div>);

	const views = state.views.map((view, id) => <div
		ref={view.ref}
		key={`view-${id}`}
		className="gridview-view-container-view"
		style={{
			left: `${view.x}%`,
			top: `${view.y}%`,
			width: `${view.width}%`,
			height: `${view.height}%`,
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