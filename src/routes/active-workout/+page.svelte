<script lang="ts">
	import { goto } from '$app/navigation';
	import { commands, type ExerciseLight } from '$lib/bindings';
	import ExerciseRowItem from '$lib/components/exercise-row-item.svelte';
	import PageWrapper from '$lib/components/layout/page-wrapper.svelte';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import * as Drawer from '$lib/components/ui/drawer';
	import { Input } from '$lib/components/ui/input/';
	import { ScrollArea } from '$lib/components/ui/scroll-area/';
	import { Separator } from '$lib/components/ui/separator/';
	import { workoutService } from '$lib/services/workout.service.svelte';
	import { ArrowLeft, CircleCheckBig, Dumbbell } from '@lucide/svelte';

	// 1. State for the input filter and the resulting exercises
	let filter = $state('');
	let exercises = $state<ExerciseLight[]>([]);
	let isLoading = $state(true);

	// 2. Use an effect to fetch data when the filter changes (with debouncing)
	$effect(() => {
		const currentFilter = filter;
		// The cleanup function runs before the effect re-runs or when the component unmounts.
		// It cancels the pending API call, preventing race conditions and unnecessary network requests.
		const handler = setTimeout(async () => {
			try {
				const result = await commands.searchExercises(currentFilter);
				if (result.status === 'ok') {
					exercises = result.data;
				} else {
					console.error('Failed to search exercises:', JSON.stringify(result.error));
					exercises = [];
				}
			} catch (error) {
				console.error('An unexpected error occurred during search:', error);
				exercises = [];
			} finally {
				isLoading = false;
			}
		}, 300); // 300ms debounce delay

		// This cleanup function is crucial for debouncing.
		return () => {
			clearTimeout(handler);
		};
	});

	function goBackToHome() {
		goto('/main/home', { replaceState: true });
	}

	function endWorkout() {
		workoutService.endWorkout();
	}
</script>

<header class="flex w-full flex-row items-center border-b p-2">
	<Button variant="ghost" size="icon" onclick={goBackToHome}>
		<ArrowLeft class="size-6" />
	</Button>
	<div class="flex flex-1 justify-center">
		<p class="flex items-center">{workoutService.elapsedTime}</p>
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
				<Input placeholder="Bench press..." bind:value={filter} />
			</Drawer.Header>
			<ScrollArea class="flex-1 overflow-hidden p-4 pt-0">
				<div class="p-4">
					{#if isLoading}
						<p>Loading...</p>
					{:else if exercises.length > 0}
						{#each exercises as exercise (exercise.id)}
							<ExerciseRowItem {exercise} />
							<Separator />
						{/each}
					{:else}
						<p>No exercises found.</p>
					{/if}
				</div>
			</ScrollArea>
		</Drawer.Content>
	</Drawer.Root>
</PageWrapper>
