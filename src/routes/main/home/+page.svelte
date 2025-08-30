<script lang="ts">
	import { goto } from '$app/navigation';
	import { PageWrapper } from '$lib/components/layout';
	import { Button } from '$lib/components/ui/button';
	import { Card } from '$lib/components/ui/card';
	import { workoutService } from '$lib/features/workout';
	import { Plus } from '@lucide/svelte';

	function startWorkout() {
		workoutService.startWorkout();
		goto('/active-workout');
	}
</script>

<PageWrapper title="Good morning!" subtitle="Ready to crush today's workout?">
	<Button size="lg" class="w-full" disabled={workoutService.isWorkoutActive} onclick={startWorkout}>
		<Plus class="mr-2 size-5" />
		Start New Workout
	</Button>

	<section class="flex flex-col gap-4">
		<h2 class="text-lg font-semibold">Recent Activity (fake)</h2>

		<!-- Placeholder Card 1 -->
		<Card class="p-4">
			<div class="flex items-center justify-between">
				<div>
					<h3 class="font-semibold">Push Day</h3>
					<p class="text-sm text-muted-foreground">Yesterday</p>
				</div>
				<span class="text-sm font-medium">5 Exercises</span>
			</div>
		</Card>

		<!-- Placeholder Card 2 -->
		<Card class="p-4">
			<div class="flex items-center justify-between">
				<div>
					<h3 class="font-semibold">Leg Day</h3>
					<p class="text-sm text-muted-foreground">3 days ago</p>
				</div>
				<span class="text-sm font-medium">6 Exercises</span>
			</div>
		</Card>
	</section>
</PageWrapper>
