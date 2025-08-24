<script>
	import { goto } from '$app/navigation';
	import PageWrapper from '$lib/components/layout/page-wrapper.svelte';
	import { Button } from '$lib/components/ui/button';
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
	<pre class="bg-blue-50">{JSON.stringify(workoutService.activeWorkout)}</pre>
</PageWrapper>
