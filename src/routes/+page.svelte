<script lang="ts">
	import { commands, type MyResponse } from '$lib/bindings';
	import { Button } from '$lib/components/ui/button';

	let clickCount = $state(0);

	let response: MyResponse | undefined = $state(undefined);
	async function callHello(): Promise<void> {
		const res = await commands.anotherCommand({a: `World-${clickCount}`});
		if (res.status === "ok") {
			response = res.data;
		} else {
			response = { message: `Error: ${res.error}` };
		}
	}
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>

<Button onclick={() => (clickCount += 1)}>Click me</Button>
<p class="font-bold bg-red-500">Button clicked {clickCount} times</p>

<Button onclick={callHello} class="my-4">Call Hello</Button>
<p class="bg-red-500 custom-p">{response?.message}</p>
{#if response}
	<p class="bg-destructive text-blue-500 p-2 rounded-md">{response.message}</p>
{/if}

<style>
	.custom-p {
		color: oklch(37.446% 0.13301 270.507);
	}
</style>