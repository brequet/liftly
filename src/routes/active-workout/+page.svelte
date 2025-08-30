<script lang="ts">
	import { goto } from '$app/navigation';
	import { onVisible } from '$lib/actions/on-visible.action';
	import ExerciseRowItem from '$lib/components/exercise-row-item.svelte';
	import PageWrapper from '$lib/components/layout/page-wrapper.svelte';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import * as Drawer from '$lib/components/ui/drawer';
	import { Input } from '$lib/components/ui/input/';
	import { ScrollArea } from '$lib/components/ui/scroll-area/';
	import { Separator } from '$lib/components/ui/separator/';
	import { commands, type ExerciseLight } from '$lib/generated/bindings';
	import { workoutService } from '$lib/services/workout.service.svelte';
	import { ArrowLeft, CircleCheckBig, Dumbbell, Loader2 } from '@lucide/svelte';

	// State for search and pagination
	let filter = $state('');
	let items = $state<ExerciseLight[]>([]);
	let page = $state(1);
	let hasMore = $state(true);
	let isFetchingMore = $state(false);

	const PAGE_SIZE = 20;

	$effect(() => {
		loadExercises(filter, 1, false);
	});

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
				items = []; // Clear items on error
				hasMore = false;
			}
		} catch (error) {
			console.error('An unexpected error occurred during search:', error);
			items = []; // Clear items on error
			hasMore = false;
		} finally {
			isFetchingMore = false;
		}
	}

	function loadMore() {
		if (isFetchingMore || !hasMore) return;
		loadExercises(filter, page + 1, true);
	}

	function goBackToHome() {
		goto('/main/home', { replaceState: true });
	}

	function endWorkout() {
		workoutService.endWorkout();
		goBackToHome();
	}
</script>

<header class="flex w-full flex-row items-center border-b p-2">
	<Button variant="ghost" size="icon" onclick={goBackToHome}>
		<ArrowLeft class="size-6" />
	</Button>
	<div class="flex flex-1 justify-center">
		<p class="flex items-center font-mono text-lg">{workoutService.elapsedTime}</p>
	</div>
	<Button variant="ghost" size="icon" onclick={endWorkout}>
		<CircleCheckBig class="size-6" />
	</Button>
</header>

<PageWrapper>
	<Drawer.Root>
		<Drawer.Trigger class={buttonVariants({ variant: 'default' })}>
			<Dumbbell />
			Add Exercise
		</Drawer.Trigger>
		<Drawer.Content>
			<Drawer.Header>
				<Input placeholder="Search exercises..." bind:value={filter} />
			</Drawer.Header>
			<ScrollArea class="flex-1 overflow-hidden p-4 pt-0">
				<div>
					{#if items.length > 0}
						{#each items as exercise (exercise.id)}
							<ExerciseRowItem {exercise} />
							<Separator />
						{/each}

						{#if isFetchingMore}
							<div class="flex items-center justify-center p-4">
								<Loader2 class="size-5 animate-spin text-muted-foreground" />
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
				</div>
			</ScrollArea>
		</Drawer.Content>
	</Drawer.Root>

	<div
		class="flex flex-1 items-center justify-center rounded-lg border-2 border-dashed border-border"
	>
		<p class="text-muted-foreground">Selected exercises will appear here.</p>
	</div>
</PageWrapper>
