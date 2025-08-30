<script lang="ts">
	import type { ExerciseLight } from '$lib/generated/bindings';
	import { Dumbbell } from '@lucide/svelte';
	import { exerciseImageService } from '../services';

	interface Props {
		exercise: ExerciseLight;
	}

	let { exercise }: Props = $props();

	const imageSrc = $derived(exerciseImageService.getImagePath(exercise.predefined_id));
</script>

<div class="flex items-center justify-between p-4">
	<div class="flex items-center gap-4">
		<div class="flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-md">
			{#if imageSrc}
				<img
					src={imageSrc}
					alt="Illustration for {exercise.title}"
					class="h-full w-full object-contain"
				/>
			{:else}
				<!-- Fallback icon when no image is found in the manifest -->
				<Dumbbell class="text-gray-400" />
			{/if}
		</div>

		<p class="font-semibold">{exercise.title}</p>
	</div>
</div>
