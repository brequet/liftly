<script lang="ts">
	import type { ExerciseLight } from '$lib/bindings';
	import { Dumbbell } from '@lucide/svelte';

	interface Props {
		exercise: ExerciseLight;
	}

	let { exercise }: Props = $props();

	// --- Image Loading Logic ---

	// $state creates a reactive state variable for our image source.
	let imageSrc = $state<string | null>(null);
	let isLoading = $state(true);

	// $effect runs this code block whenever its dependencies (exercise.predefined_id) change.
	// This is the ideal place for side effects like fetching data or, in this case, checking for images.
	$effect(() => {
		// 1. Reset state for the new exercise
		isLoading = true;
		imageSrc = null;
		const predefinedId = exercise.predefined_id;

		// If there's no ID, we can stop early.
		if (!predefinedId) {
			isLoading = false;
			return;
		}

		// 2. Define potential image URLs based on the static path
		const tensionSrc = `/exercises/${predefinedId}-tension.svg`;
		const relaxationSrc = `/exercises/${predefinedId}-relaxation.svg`;

		// 3. Helper function to check if an image URL is valid
		const checkImage = (src: string, onFail: () => void) => {
			const img = new Image();
			img.onload = () => {
				// Success! Set the reactive state.
				imageSrc = src;
				isLoading = false;
			};
			img.onerror = () => {
				// Failure. Trigger the fallback.
				onFail();
			};
			img.src = src;
		};

		// 4. Start the check, beginning with the 'tension' image.
		checkImage(tensionSrc, () => {
			// If tension fails, check for 'relaxation'.
			checkImage(relaxationSrc, () => {
				// If relaxation also fails, we have no image.
				imageSrc = null;
				isLoading = false;
			});
		});
	});
</script>

<div class="flex items-center justify-between p-4">
	<div class="flex items-center gap-4">
		<!-- 
          This container ensures the layout doesn't shift. 
          It has a fixed size and acts as a placeholder.
        -->
		<div class="flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-md">
			{#if isLoading}
				<!-- Skeleton loader for a better user experience -->
				<div class="h-full w-full animate-pulse rounded-md bg-gray-300"></div>
			{:else if imageSrc}
				<!-- Image is successfully loaded -->
				<img
					src={imageSrc}
					alt="Illustration for {exercise.title}"
					class="h-full w-full object-contain"
				/>
			{:else}
				<!-- 
                  Fallback placeholder when no image is found.
                  Using an icon maintains visual consistency.
                -->
				<Dumbbell class="text-gray-400" />
			{/if}
		</div>

		<p class="font-semibold">{exercise.title}</p>
	</div>

	<!-- Any elements on the right (like a button) would go here -->
</div>
