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

	// This function reference is stable and can be safely passed to the service.
	function close() {
		open = false;
	}

	$effect(() => {
		if (open) {
			drawerService.register(id, close);
		} else {
			// This is called when `open` becomes false.
			drawerService.unregister(id);
		}

		// The return function from $effect provides automatic cleanup.
		// This ensures the drawer is unregistered if the component is ever destroyed.
		return () => {
			drawerService.unregister(id);
		};
	});
</script>

<DrawerPrimitive.Root {shouldScaleBackground} bind:open bind:activeSnapPoint {...restProps} />
