<script lang="ts">
	import PageWrapper from '$lib/components/layout/page-wrapper.svelte';
	import { getName, getVersion } from '@tauri-apps/api/app';
	import { onMount } from 'svelte';

	interface AppData {
		name: string;
		version: string;
	}

	let appData: AppData = $state({ name: '', version: '' });

	onMount(async () => {
		const name = await getName();
		const version = await getVersion();

		appData = {
			name,
			version
		};
	});
</script>

<PageWrapper title="Profile">
	<p class="align text-center text-muted-foreground">{appData.name} - {appData.version}</p>
</PageWrapper>
