import { watch } from 'runed';

type SearchFunction<T> = (
	query: string,
	pagination: { page: number; pageSize: number }
) => Promise<{
	status: 'ok' | 'error';
	data?: { items: T[]; hasMore: boolean };
	error?: unknown;
}>;

interface UsePaginatedSearchOptions {
	pageSize?: number;
	debounce?: number;
}

/**
 * A composable for managing state for a debounced, paginated search.
 * @param searchFn The async function that fetches data.
 * @param options Configuration like pageSize and debounce delay.
 */
export function usePaginatedSearch<T>(
	searchFn: SearchFunction<T>,
	options: UsePaginatedSearchOptions = {}
) {
	const { pageSize = 20, debounce = 0 } = options;

	let filter = $state('');
	let items = $state<T[]>([]);
	let page = $state(1);
	let hasMore = $state(true);
	let isFetching = $state(false);
	let isFetchingMore = $state(false);
	let error = $state<string | null>(null);

	const isLoading = $derived(isFetching || isFetchingMore);

	watch(
		() => filter,
		() => {
			const currentFilter = filter;
			isFetching = true;
			hasMore = true;

			const handler = setTimeout(() => {
				load(currentFilter, 1, false);
			}, debounce);

			return () => clearTimeout(handler);
		}
	);

	async function load(query: string, pageToLoad: number, append: boolean) {
		if (append) {
			if (isFetchingMore || !hasMore) return;
			isFetchingMore = true;
		}
		error = null;

		try {
			const result = await searchFn(query, { page: pageToLoad, pageSize });

			if (result.status === 'ok' && result.data) {
				items = append ? [...items, ...result.data.items] : result.data.items;
				hasMore = result.data.hasMore;
				page = pageToLoad;
			} else {
				throw new Error(JSON.stringify(result.error) || 'Search failed');
			}
		} catch (e: unknown) {
			error = e instanceof Error ? e.message : 'An unexpected error occurred';
			items = [];
			hasMore = false;
		} finally {
			isFetching = false;
			isFetchingMore = false;
		}
	}

	function loadMore() {
		load(filter, page + 1, true);
	}

	return {
		get items() {
			return items;
		},
		get isLoading() {
			return isLoading;
		},
		get isFetchingMore() {
			return isFetchingMore;
		},
		get hasMore() {
			return hasMore;
		},
		get error() {
			return error;
		},
		get filter() {
			return filter;
		},
		set filter(value: string) {
			filter = value;
		},
		loadMore
	};
}
