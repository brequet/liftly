<script lang="ts">
	import { BottomNav } from '$lib/components/layout';
	import { ActiveWorkoutBanner } from '$lib/features/workout';

	let { children } = $props();
</script>

<div class="flex h-full flex-col justify-between">
	{@render children?.()}

	<ActiveWorkoutBanner />
	<BottomNav />
</div>
