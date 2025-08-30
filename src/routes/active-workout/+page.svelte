<script lang="ts">
	import { goto } from '$app/navigation';
	import { PageWrapper } from '$lib/components/layout';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import * as Drawer from '$lib/components/ui/drawer';
	import { ExerciseSearch } from '$lib/features/exercise';
	import { workoutService } from '$lib/features/workout';
	import { ArrowLeft, CircleCheckBig, Dumbbell } from '@lucide/svelte';

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
			<ExerciseSearch />
		</Drawer.Content>
	</Drawer.Root>

	<div
		class="flex flex-1 items-center justify-center rounded-lg border-2 border-dashed border-border"
	>
		<p class="text-muted-foreground">Selected exercises will appear here.</p>
	</div>
</PageWrapper>
