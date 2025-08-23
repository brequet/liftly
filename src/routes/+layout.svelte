<script lang="ts">
	import '../app.css';

	let { children } = $props();
</script>

<div class="flex h-full flex-col">
	<main class="flex-1 overflow-y-auto pt-12">
		{@render children?.()}
	</main>
</div>
