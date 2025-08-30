import manifest from '$lib/generated/exercise-images.json';

class ExerciseImageService {
	private imageMap = $state<Record<string, string[]>>(manifest);

	/**
	 * Returns the primary image for a given exercise ID.
	 * It prioritizes 'tension' over 'relaxation' or takes the first available.
	 * @param predefinedId The exercise's predefined ID.
	 * @returns The image URL or null if not found.
	 */
	getImagePath(predefinedId: string | null): string | null {
		if (!predefinedId) {
			return null;
		}

		const images = this.imageMap[predefinedId];
		if (!images || images.length === 0) {
			return null;
		}

		const tensionImage = images.find((img) => img.includes('-tension.svg'));
		return tensionImage || images[0];
	}
}

export const exerciseImageService = new ExerciseImageService();
