<script lang="ts">
	import { onVisible } from '$lib/actions/on-visible.action';
	import ExerciseRowItem from '$lib/components/exercise-row-item.svelte';
	import { Separator } from '$lib/components/ui/separator/';
	import { usePaginatedSearch } from '$lib/composables/usePaginatedSearch.svelte';
	import { commands, type ExerciseLight } from '$lib/generated/bindings';
	import { LoaderCircle } from '@lucide/svelte';
	import { Input } from './ui/input';
	import { ScrollArea } from './ui/scroll-area';

	const search = usePaginatedSearch<ExerciseLight>(commands.searchExercises);
</script>

<div class="flex h-full flex-col">
	<div class="p-4">
		<Input placeholder="Search exercises..." bind:value={search.filter} />
	</div>

	<div class="flex-1 overflow-hidden">
		<ScrollArea class="h-full px-4">
			{#if search.items.length > 0}
				{#each search.items as exercise (exercise.id)}
					<ExerciseRowItem {exercise} />
					<Separator />
				{/each}

				{#if search.isFetchingMore}
					<div class="flex items-center justify-center p-4">
						<LoaderCircle class="size-5 animate-spin text-muted-foreground" />
						<span class="ml-2 text-sm text-muted-foreground">Loading more...</span>
					</div>
				{/if}

				<!-- Placeholder to trigger loading more -->
				{#if search.hasMore && !search.isFetchingMore}
					<div class="h-10 w-full" use:onVisible={{ callback: search.loadMore }}></div>
				{/if}
			{:else}
				<p class="p-8 text-center text-muted-foreground">No exercises found.</p>
			{/if}
		</ScrollArea>
	</div>
</div>
