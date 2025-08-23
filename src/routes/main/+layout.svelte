<script lang="ts">
	import BottomNav from '$lib/components/layout/bottom-nav.svelte';

	let { children } = $props();
</script>

<div class="flex h-full flex-col justify-between">
	{@render children?.()}
	<BottomNav />
</div>
