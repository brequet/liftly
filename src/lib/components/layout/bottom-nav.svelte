<script lang="ts">
	import { page } from '$app/state';
	import { Button } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
	import { Dumbbell, House, User } from '@lucide/svelte';

	const navLinks = [
		{ href: '/main/home', label: 'Home', icon: House },
		{ href: '/main/workouts', label: 'Workouts', icon: Dumbbell },
		{ href: '/main/profile', label: 'Profile', icon: User }
	];
</script>

<nav
	class="flex justify-around border-t bg-card p-2 pb-[calc(env(safe-area-inset-bottom,0rem)+0.5rem)]"
>
	{#each navLinks as link}
		{@const isSelected = page.url.pathname === link.href}
		<a href={link.href} class="flex h-auto flex-col items-center gap-1 p-2 text-sm">
			<Button
				variant="ghost"
				size="icon"
				class={cn(
					'flex h-auto flex-col gap-1 p-2 text-muted-foreground',
					isSelected && 'font-extrabold text-primary'
				)}
			>
				<svelte:component this={link.icon} class="size-6" />
				<span class="text-xs">{link.label}</span>
			</Button>
		</a>
	{/each}
</nav>
