class DrawerService {
	// Use a Map to store drawers by a unique ID. This is the core fix.
	private openDrawers = $state(new Map<string, () => void>());

	isDrawerOpen = $derived(this.openDrawers.size > 0);

	/**
	 * Called by a drawer component when it opens.
	 * It adds its ID and close function to the map.
	 */
	register(id: string, close: () => void) {
		this.openDrawers.set(id, close);
		console.log(`DrawerService: Registered drawer ${id}. Total open: ${this.openDrawers.size}`);
	}

	/**
	 * Called by a drawer component when it closes or is destroyed.
	 * It removes its ID from the map.
	 */
	unregister(id: string) {
		if (this.openDrawers.has(id)) {
			this.openDrawers.delete(id);
			console.log(`DrawerService: Unregistered drawer ${id}. Total open: ${this.openDrawers.size}`);
		}
	}

	/**
	 * Called by the global back handler.
	 * It closes the most recently opened drawer.
	 * @returns {boolean} - `true` if a drawer was closed, `false` otherwise.
	 */
	closeTopDrawer(): boolean {
		if (this.openDrawers.size === 0) {
			console.log('DrawerService: No drawers to close.');
			return false;
		}

		// Get the last key added to the map (most recent drawer)
		const lastId = Array.from(this.openDrawers.keys()).pop();
		if (lastId) {
			console.log(`DrawerService: Closing top drawer ${lastId}.`);
			const closeFn = this.openDrawers.get(lastId);
			closeFn?.();
			// The drawer's own $effect will then call unregister()
			return true;
		}
		return false;
	}
}

export const drawerService = new DrawerService();
