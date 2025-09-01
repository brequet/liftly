import { info } from '@tauri-apps/plugin-log';
import { drawerService } from './drawer.service.svelte';

class NativeBridgeService {
	constructor() {
		this.initialize();
	}

	private initialize() {
		if (typeof window === 'undefined') {
			return;
		}

		info('[FRONTEND] NativeBridgeService initialized. Setting up key event handler.');

		/**
		 * This function is called from the native Android code (MainActivity.kt).
		 * @param keyName - The name of the key that was pressed (e.g., 'back').
		 * @returns {boolean} - `false` to prevent native action, `true` to allow it.
		 */
		window.onNativeKeyEvent = (keyName: string): boolean => {
			info(`[FRONTEND] onNativeKeyEvent received: ${keyName}`);

			if (keyName === 'back') {
				const wasDrawerClosed = drawerService.closeTopDrawer();

				if (wasDrawerClosed) {
					info('[FRONTEND] Back key handled by DrawerService. Preventing native navigation.');
					// **Crucially, we return `false` to prevent the default native back navigation.**
					return false;
				}
			}

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
			delete window.onNativeKeyEvent;
		}
	}
}

export const nativeBridgeService = new NativeBridgeService();
