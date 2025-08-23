<script>
	import PageWrapper from '$lib/components/layout/page-wrapper.svelte';
	import { Button } from '$lib/components/ui/button';
	import { formatElapsedTime } from '$lib/services/datetime';
	import { ArrowLeft, CircleCheckBig } from '@lucide/svelte';
	import { onMount } from 'svelte';

	let startDateTime = new Date();
	let elapsedTime = $state(formatElapsedTime(startDateTime, startDateTime));

	onMount(() => {
		const interval = setInterval(() => {
			elapsedTime = formatElapsedTime(startDateTime, new Date());
		}, 1000);

		return () => clearInterval(interval);
	});
</script>

<header class="flex w-full flex-row items-center border-b p-2">
	<a href="/main/home" class="flex h-auto flex-col items-center text-sm">
		<Button variant="ghost" size="icon">
			<ArrowLeft class="size-6" />
		</Button>
	</a>
	<div class="flex flex-1 justify-center">
		<p class="flex items-center">{elapsedTime}</p>
	</div>
	<Button variant="ghost" size="icon">
		<CircleCheckBig class="size-6" />
	</Button>
</header>

<PageWrapper>salut</PageWrapper>
