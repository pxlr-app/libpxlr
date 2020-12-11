import { useEffect, useRef } from 'react';

export default function(callback: (delta: number) => void, deps?: React.DependencyList) {
	const requestRef = useRef<number>(-1);
	const previousTimeRef = useRef<number>(-1);
	
	const animate = (time: number) => {
		if (previousTimeRef.current != undefined) {
			const deltaTime = time - previousTimeRef.current;
			callback(deltaTime);
		}
		previousTimeRef.current = time;
		requestRef.current = requestAnimationFrame(animate);
	}
	
	useEffect(() => {
		requestRef.current = requestAnimationFrame(animate);
		return () => cancelAnimationFrame(requestRef.current);
	}, deps);
}