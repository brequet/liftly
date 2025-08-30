import type { Action } from 'svelte/action';

interface Options {
	callback: () => void;
	root?: Element | null;
	threshold?: number;
	rootMargin?: string;
}

export const onVisible: Action<HTMLElement, Options> = (node, options) => {
	const { callback, root = null, threshold = 0.1, rootMargin = '0px' } = options;

	const observer = new IntersectionObserver(
		(entries) => {
			const entry = entries[0];
			if (entry.isIntersecting) {
				callback();
			}
		},
		{ root, threshold, rootMargin }
	);

	observer.observe(node);

	return {
		destroy() {
			observer.unobserve(node);
		}
	};
};
