<script lang="ts">
	import BottomNav from '$lib/components/layout/bottom-nav.svelte';
	import { workoutService } from '$lib/services/workout.service.svelte';

	let { children } = $props();
</script>

<div class="flex h-full flex-col justify-between">
	{@render children?.()}

	{#if workoutService.isWorkoutActive}
		<a
			href="/active-workout"
			class="flex flex-col justify-around border-t bg-secondary py-2 text-center"
		>
			<h3 class="font-semibold underline">Ongoing workout</h3>
			<p class="font-mono">{workoutService.elapsedTime}</p>
		</a>
	{/if}
	<BottomNav />
</div>
