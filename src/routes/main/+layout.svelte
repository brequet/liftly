<script lang="ts">
	import { ActiveWorkoutBanner } from '$lib/features/workout';
	import BottomNav from './_components/bottom-nav.svelte';

	let { children } = $props();
</script>

<div class="flex h-full flex-col justify-between">
	{@render children?.()}

	<ActiveWorkoutBanner />
	<BottomNav />
</div>
