import { info } from '@tauri-apps/plugin-log';
import { drawerService } from './drawer.service.svelte';

class NativeBridgeService {
	constructor() {
		this.initialize();
	}

	private initialize() {
		// Guard against running in a non-browser environment
		if (typeof window === 'undefined') {
			return;
		}

		info('[FRONTEND] NativeBridgeService initialized. Setting up key event handler.');

		/**
		 * This function is called from the native Android code (MainActivity.kt).
		 * @param keyName - The name of the key that was pressed (e.g., 'back').
		 * @returns {boolean} - `false` to prevent native action, `true` to allow it.
		 */
		(window as any).onNativeKeyEvent = (keyName: string): boolean => {
			info(`[FRONTEND] onNativeKeyEvent received: ${keyName}`);

			// We only care about the 'back' key for now
			if (keyName === 'back') {
				// Ask the drawer service to close the top-most drawer
				const wasDrawerClosed = drawerService.closeTopDrawer();

				if (wasDrawerClosed) {
					info('[FRONTEND] Back key handled by DrawerService. Preventing native navigation.');
					// **Crucially, we return `false` to prevent the default native back navigation.**
					return false;
				}
			}

			// For any other key, or if no drawer was open, return `true`
			// to allow the default native behavior (e.g., navigating back a page).
			info('[FRONTEND] Event not handled by frontend. Allowing native action.');
			return true;
		};
	}

	/**
	 * Cleans up the global event handler.
	 */
	cleanup() {
		if (typeof window !== 'undefined') {
			info('[FRONTEND] Cleaning up native key event handler.');
			delete (window as any).onNativeKeyEvent;
		}
	}
}

// Export a singleton instance. The act of importing this module will instantiate the service.
export const nativeBridgeService = new NativeBridgeService();
