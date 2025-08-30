import { commands, type Workout } from '$lib/generated/bindings';
import { formatElapsedTime } from './datetime.utils';

class WorkoutService {
	activeWorkout = $state<Workout | null>(null);
	now = $state(new Date());

	elapsedTime = $derived.by(() => {
		if (!this.activeWorkout || this.now < new Date(this.activeWorkout.start_datetime)) {
			return '00:00:00';
		}
		const startDateTime = new Date(this.activeWorkout.start_datetime);
		return formatElapsedTime(startDateTime, this.now);
	});

	constructor() {
		this.fetchActiveWorkout();

		$effect.root(() => {
			$effect(() => {
				if (this.activeWorkout) {
					const timerId = setInterval(() => {
						this.now = new Date();
					}, 1000);

					return () => {
						clearInterval(timerId);
					};
				}
			});
		});
	}

	/**
	 * A reactive boolean indicating if a workout is currently active.
	 */
	get isWorkoutActive() {
		// This remains a simple getter, which is fine.
		// For more complex derivations, $derived is preferred.
		return this.activeWorkout !== null;
	}

	/**
	 * Fetches the active workout from the backend and updates the state.
	 */
	async fetchActiveWorkout() {
		try {
			const workout = await commands.getActiveWorkout();
			this.activeWorkout = workout.status === 'ok' ? workout.data : null;
			if (workout.status !== 'ok') {
				console.error('Failed to fetch active workout:', workout.error);
			}
		} catch (error) {
			this.activeWorkout = null;
			console.error('An unexpected error occurred while fetching the active workout:', error);
		}
	}

	/**
	 * Starts a new workout session.
	 */
	async startWorkout() {
		try {
			const workout = await commands.createWorkout();
			if (workout.status === 'ok') {
				this.activeWorkout = workout.data;
			} else {
				console.error('Failed to start workout:', workout.error);
			}
		} catch (error) {
			console.error('An unexpected error occurred while starting a workout:', error);
		}
	}

	/**
	 * Ends the current workout session.
	 */
	async endWorkout() {
		try {
			const workout = await commands.endWorkout();
			if (workout.status === 'ok') {
				this.activeWorkout = null;
			} else {
				console.error('Failed to end workout:', workout.error.data);
			}
		} catch (error) {
			console.error('An unexpected error occurred while ending the workout:', error);
		}
	}
}

/**
 * A singleton instance of the WorkoutService for use throughout the application.
 */
export const workoutService = new WorkoutService();
