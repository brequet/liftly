<script>
	import { goto } from '$app/navigation';
	import PageWrapper from '$lib/components/layout/page-wrapper.svelte';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import * as Drawer from '$lib/components/ui/drawer';
	import Input from '$lib/components/ui/input/input.svelte';
	import { workoutService } from '$lib/services/workout.service.svelte';
	import { ArrowLeft, CircleCheckBig } from '@lucide/svelte';

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
		<Drawer.Trigger class={buttonVariants({ variant: 'default' })}>Add Exercise</Drawer.Trigger>
		<Drawer.Content>
			<Drawer.Header>
				<Input placeholder="Bench press..." />
			</Drawer.Header>

			<Drawer.Footer>
				<Button>Submit</Button>
				<Drawer.Close>Cancel</Drawer.Close>
			</Drawer.Footer>
		</Drawer.Content>
	</Drawer.Root>
</PageWrapper>
