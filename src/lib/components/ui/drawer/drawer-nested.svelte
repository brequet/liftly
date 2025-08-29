<script lang="ts">
	import { drawerService } from '$lib/services/drawer.service.svelte';
	import { Drawer as DrawerPrimitive } from 'vaul-svelte';

	// Generate a stable, unique ID for this specific drawer instance.
	const id = crypto.randomUUID();

	let {
		shouldScaleBackground = true,
		open = $bindable(false),
		activeSnapPoint = $bindable(null),
		...restProps
	}: DrawerPrimitive.RootProps = $props();

	// This function reference is stable.
	function close() {
		open = false;
	}

	$effect(() => {
		if (open) {
			drawerService.register(id, close);
		} else {
			drawerService.unregister(id);
		}

		return () => {
			drawerService.unregister(id);
		};
	});
</script>

<DrawerPrimitive.NestedRoot {shouldScaleBackground} bind:open bind:activeSnapPoint {...restProps} />
