<script lang="ts">
	import { commands, type MyResponse } from '$lib/bindings';
	import { Button } from '$lib/components/ui/button';
	import Database from '@tauri-apps/plugin-sql';
	import { onMount } from 'svelte';

	let clickCount = $state(0);

	let response: MyResponse | undefined = $state(undefined);
	async function callHello(): Promise<void> {
		const res = await commands.anotherCommand({ a: `World-${clickCount}` });
		if (res.status === 'ok') {
			response = res.data;
		} else {
			response = { message: `Error: ${res.error}` };
		}
	}

	let dbRes = $state('');
	let dbRes2: string[] = $state([]);

	onMount(async () => {
		const db = await Database.load('sqlite:liftly.db');
		dbRes = await db.select("SELECT name FROM sqlite_master WHERE type='table'");

		const tablesRes = await commands.getDbTables();
		if (tablesRes.status === 'ok') {
			dbRes2 = tablesRes.data;
		}
	});
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>

<Button onclick={() => (clickCount += 1)}>Click me</Button>
<p class="bg-red-500 font-bold">Button clicked {clickCount} times</p>

<Button onclick={callHello} class="my-4">Call Hello</Button>
<p class="custom-p bg-red-500">{response?.message}</p>
{#if response}
	<p class="rounded-md bg-destructive p-2 text-blue-500">{response.message}</p>
{/if}

<pre class="bg-amber-200">
	{JSON.stringify(dbRes)}
</pre>

<h3>tables</h3>
<ul>
	{#each dbRes2 as table}
		<li>{table}</li>
	{/each}
</ul>

<style>
	.custom-p {
		color: oklch(37.446% 0.13301 270.507);
	}
</style>
