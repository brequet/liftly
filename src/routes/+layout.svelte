<script lang="ts">
	import '$lib/services/native-bridge.service';
	import '../app.css';

	let { children } = $props();

	// The previous onMount logic has been moved to the NativeBridgeService.
</script>

<div class="flex h-full flex-col">
	<main class="flex-1 overflow-y-auto">
		{@render children?.()}
	</main>
</div>
