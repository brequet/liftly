<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		title?: string;
		subtitle?: string;
		children: Snippet<[]>;
	}

	let { title, subtitle, children }: Props = $props();
</script>

<div class="flex flex-1 flex-col gap-8 px-2">
	{#if title}
		<header>
			<h1 class="text-2xl font-bold">{title}</h1>
			{#if subtitle}
				<p class="text-muted-foreground">{subtitle}</p>
			{/if}
		</header>
	{/if}

	{@render children()}
</div>
