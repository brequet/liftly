import type { ApiError } from '$lib/generated/bindings';
import { error as logError } from '@tauri-apps/plugin-log';

class LoggingService {
	/**
	 * Logs an ApiError with a descriptive message based on its type.
	 * This provides consistent and readable error logging across the app.
	 * @param context A string describing the action that failed (e.g., "Failed to end workout").
	 * @param apiError The error object received from the backend.
	 */
	public logApiError(context: string, apiError: ApiError): void {
		let message = `[API Error] ${context}: ${apiError.type}`;

		switch (apiError.type) {
			case 'Database':
			case 'Internal':
				message += ` - Details: ${apiError.data}`;
				break;
			case 'SetNotInWorkout':
				message += ` - Details: ${JSON.stringify(apiError.data)}`;
				break;
			case 'WorkoutSetNotFound':
				message += ` - Details: ${JSON.stringify(apiError.data)}`;
				break;
			case 'WorkoutAlreadyInProgress':
			case 'NoActiveWorkout':
				break;
		}

		logError(message);
	}

	/**
	 * Logs a generic, unexpected error.
	 * @param context A string describing the action that failed.
	 * @param error The caught error object.
	 */
	public logUnexpectedError(context: string, error: unknown): void {
		const errorMessage = error instanceof Error ? error.message : String(error);
		logError(`[Unexpected Error] ${context}: ${errorMessage}`);
		if (error instanceof Error && error.stack) {
			console.error(error.stack);
		}
	}
}

export const loggingService = new LoggingService();
