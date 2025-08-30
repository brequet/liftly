<script lang="ts">
	import { onVisible } from '$lib/actions/on-visible.action';
	import ExerciseRowItem from '$lib/components/exercise-row-item.svelte';
	import { Separator } from '$lib/components/ui/separator/';
	import { commands, type ExerciseLight } from '$lib/generated/bindings';
	import { LoaderCircle } from '@lucide/svelte';
	import { watch } from 'runed';
	import { Input } from './ui/input';
	import { ScrollArea } from './ui/scroll-area';

	let filter = $state('');
	let items = $state<ExerciseLight[]>([]);
	let page = $state(1);
	let hasMore = $state(true);
	let isFetchingMore = $state(false);

	const PAGE_SIZE = 20;

	watch(
		() => filter,
		() => {
			hasMore = true;
			loadExercises(filter, 1, false);
		}
	);

	async function loadExercises(query: string, pageToLoad: number, append: boolean) {
		if (!hasMore) return;

		if (append) {
			isFetchingMore = true;
		}

		try {
			const result = await commands.searchExercises(query, {
				page: pageToLoad,
				pageSize: PAGE_SIZE
			});

			if (result.status === 'ok') {
				const paginatedData = result.data;
				if (append) {
					items = [...items, ...paginatedData.items];
				} else {
					items = paginatedData.items;
				}
				hasMore = paginatedData.hasMore;
				page = pageToLoad;
			} else {
				console.error('Failed to search exercises:', JSON.stringify(result.error));
				items = [];
				hasMore = false;
			}
		} catch (error) {
			console.error('An unexpected error occurred during search:', error);
			items = [];
			hasMore = false;
		} finally {
			isFetchingMore = false;
		}
	}

	function loadMore() {
		if (isFetchingMore || !hasMore) return;
		loadExercises(filter, page + 1, true);
	}
</script>

<div class="flex h-full flex-col">
	<div class="p-4">
		<Input placeholder="Search exercises..." bind:value={filter} />
	</div>

	<div class="flex-1 overflow-hidden">
		<ScrollArea class="h-full px-4">
			{#if items.length > 0}
				{#each items as exercise (exercise.id)}
					<ExerciseRowItem {exercise} />
					<Separator />
				{/each}

				{#if isFetchingMore}
					<div class="flex items-center justify-center p-4">
						<LoaderCircle class="size-5 animate-spin text-muted-foreground" />
						<span class="ml-2 text-sm text-muted-foreground">Loading more...</span>
					</div>
				{/if}

				<!-- Placeholder to trigger loading more -->
				{#if hasMore && !isFetchingMore}
					<div class="h-10 w-full" use:onVisible={{ callback: loadMore }}></div>
				{/if}
			{:else}
				<p class="p-8 text-center text-muted-foreground">No exercises found.</p>
			{/if}
		</ScrollArea>
	</div>
</div>
