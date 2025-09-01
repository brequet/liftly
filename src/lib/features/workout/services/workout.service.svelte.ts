import { commands, type WorkoutSession } from '$lib/generated/bindings';
import { loggingService } from '$lib/services';
import { formatElapsedTime } from '$lib/utils';

class WorkoutService {
	activeWorkout = $state<WorkoutSession | null>(null);
	now = $state(new Date());

	isWorkoutActive = $derived(this.activeWorkout !== null);

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

	async fetchActiveWorkout() {
		try {
			const result = await commands.getActiveWorkout();
			if (result.status === 'ok') {
				this.activeWorkout = result.data;
			} else {
				loggingService.logApiError('Failed to fetch active workout', result.error);
				this.activeWorkout = null;
			}
		} catch (error) {
			loggingService.logUnexpectedError('Fetching active workout', error);
			this.activeWorkout = null;
		}
	}

	async startWorkout() {
		try {
			const result = await commands.createWorkout();
			if (result.status === 'ok') {
				this.activeWorkout = result.data;
			} else {
				loggingService.logApiError('Failed to start workout', result.error);
			}
		} catch (error) {
			loggingService.logUnexpectedError('Starting a workout', error);
		}
	}

	async endWorkout() {
		try {
			const result = await commands.endWorkout();
			if (result.status === 'ok') {
				this.activeWorkout = null;
			} else {
				loggingService.logApiError('Failed to end workout', result.error);
			}
		} catch (error) {
			loggingService.logUnexpectedError('Ending the workout', error);
		}
	}

	async addSet(exerciseId: number, reps: number, weight: number) {
		if (!this.activeWorkout) return;

		try {
			const result = await commands.addSetToActiveWorkout(exerciseId, reps, weight);
			if (result.status === 'ok') {
				const newSet = result.data;
				this.activeWorkout.sets.push(newSet);
			} else {
				loggingService.logApiError('Failed to add set', result.error);
			}
		} catch (error) {
			loggingService.logUnexpectedError('Adding set to workout', error);
		}
	}

	async updateSet(setId: number, exerciseId: number, reps: number, weight: number) {
		if (!this.activeWorkout) return;

		try {
			const result = await commands.updateSetFromActiveWorkout(setId, exerciseId, reps, weight);
			if (result.status === 'ok') {
				const updatedSet = result.data;
				const index = this.activeWorkout.sets.findIndex((s) => s.id === updatedSet.id);
				if (index !== -1) {
					this.activeWorkout.sets[index] = updatedSet;
				}
			} else {
				loggingService.logApiError('Failed to update set', result.error);
			}
		} catch (error) {
			loggingService.logUnexpectedError('Updating set in workout', error);
		}
	}

	async removeSet(setId: number) {
		if (!this.activeWorkout) return;

		try {
			const result = await commands.removeSetFromActiveWorkout(setId);
			if (result.status === 'ok') {
				this.activeWorkout.sets = this.activeWorkout.sets.filter((s) => s.id !== setId);
			} else {
				loggingService.logApiError('Failed to remove set', result.error);
			}
		} catch (error) {
			loggingService.logUnexpectedError('Removing set from workout', error);
		}
	}
}

export const workoutService = new WorkoutService();
