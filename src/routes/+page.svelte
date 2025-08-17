<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import type { HelloResponse } from '$lib/taurpc';
	import taurpc from '$lib/taurpc';

	let clickCount = $state(0);

	let response: HelloResponse | undefined = $state(undefined);
	async function callHello(): Promise<void> {
		response = await taurpc.hello({ name: `World-${clickCount}` });
	}
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>

<Button onclick={() => (clickCount += 1)}>Click me</Button>
<p>Button clicked {clickCount} times</p>

<Button onclick={callHello}>Call Hello</Button>
<p>{response?.message}</p>
