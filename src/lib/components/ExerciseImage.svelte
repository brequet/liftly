<script lang="ts">
	let { name, alt = 'Exercise Icon' }: { name: string; alt?: string } = $props();

	const iconPath = $derived(`/exercises/${name}`);
</script>

{#if name}
	<img src={iconPath} {alt} class="h-24 w-24 object-contain" />
{:else}
	<div class="flex h-24 w-24 items-center justify-center rounded-md bg-gray-200">
		<span class="text-sm text-gray-500">No Icon</span>
	</div>
{/if}
