<script lang="ts">
	interface Props {
		exercise_predefined_id: number | null;
	}

	let { exercise_predefined_id }: Props = $props();

	/*
	 * TODO: images MAY exist in the static/exercises/ directory
	 * The images are .svg with this pattern: `${exercise_predefined_id}-(relaxation|tension).svg`
	 */
</script>

{#if exercise_predefined_id}
	<img
		src={`/exercises/${exercise_predefined_id}`}
		alt="Exercise Icon"
		class="h-24 w-24 object-contain"
	/>
{:else}
	<div class="flex h-24 w-24 items-center justify-center rounded-md bg-gray-200">
		<span class="text-sm text-gray-500">No Icon</span>
	</div>
{/if}
