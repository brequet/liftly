<script lang="ts">
	import { workoutService } from '../services';
</script>

{#if workoutService.isWorkoutActive}
	<a
		href="/active-workout"
		class="flex flex-col justify-around border-t bg-secondary py-2 text-center text-secondary-foreground"
	>
		<h3 class="font-semibold underline">Ongoing workout</h3>
		<p class="font-mono">{workoutService.elapsedTime}</p>
	</a>
{/if}
