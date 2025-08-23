# Flattened Codebase

Total files: 46

## Table of Contents

1. [.\README.md](#file-1)
2. [.\components.json](#file-2)
3. [.\eslint.config.js](#file-3)
4. [.\package.json](#file-4)
5. [.\src\app.css](#file-5)
6. [.\src\app.d.ts](#file-6)
7. [.\src\app.html](#file-7)
8. [.\src\lib\bindings.ts](#file-8)
9. [.\src\lib\components\layout\bottom-nav.svelte](#file-9)
10. [.\src\lib\components\layout\page-wrapper.svelte](#file-10)
11. [.\src\lib\components\ui\button\button.svelte](#file-11)
12. [.\src\lib\components\ui\button\index.ts](#file-12)
13. [.\src\lib\components\ui\card\card-action.svelte](#file-13)
14. [.\src\lib\components\ui\card\card-content.svelte](#file-14)
15. [.\src\lib\components\ui\card\card-description.svelte](#file-15)
16. [.\src\lib\components\ui\card\card-footer.svelte](#file-16)
17. [.\src\lib\components\ui\card\card-header.svelte](#file-17)
18. [.\src\lib\components\ui\card\card-title.svelte](#file-18)
19. [.\src\lib\components\ui\card\card.svelte](#file-19)
20. [.\src\lib\components\ui\card\index.ts](#file-20)
21. [.\src\lib\index.ts](#file-21)
22. [.\src\lib\services\datetime.ts](#file-22)
23. [.\src\lib\utils.ts](#file-23)
24. [.\src\routes\+layout.svelte](#file-24)
25. [.\src\routes\+layout.ts](#file-25)
26. [.\src\routes\+page.ts](#file-26)
27. [.\src\routes\active-workout\+page.svelte](#file-27)
28. [.\src\routes\main\+layout.svelte](#file-28)
29. [.\src\routes\main\home\+page.svelte](#file-29)
30. [.\src\routes\main\profile\+page.svelte](#file-30)
31. [.\src\routes\main\workouts\+page.svelte](#file-31)
32. [.\src-tauri\Cargo.toml](#file-32)
33. [.\src-tauri\build.rs](#file-33)
34. [.\src-tauri\capabilities\default.json](#file-34)
35. [.\src-tauri\src\bin\export_bindings.rs](#file-35)
36. [.\src-tauri\src\commands\api1.rs](#file-36)
37. [.\src-tauri\src\commands\api2.rs](#file-37)
38. [.\src-tauri\src\commands\mod.rs](#file-38)
39. [.\src-tauri\src\db\connection.rs](#file-39)
40. [.\src-tauri\src\db\mod.rs](#file-40)
41. [.\src-tauri\src\lib.rs](#file-41)
42. [.\src-tauri\src\main.rs](#file-42)
43. [.\src-tauri\tauri.conf.json](#file-43)
44. [.\svelte.config.js](#file-44)
45. [.\tsconfig.json](#file-45)
46. [.\vite.config.ts](#file-46)

## File 1: .\README.md

```md
# sv

Everything you need to build a Svelte project, powered by [`sv`](https://github.com/sveltejs/cli).

## Creating a project

If you're seeing this, you've probably already done this step. Congrats!

```sh
# create a new project in the current directory
npx sv create

# create a new project in my-app
npx sv create my-app
```

## Developing

Once you've created a project and installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

```sh
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Building

To create a production version of your app:

```sh
npm run build
```

You can preview the production build with `npm run preview`.

> To deploy your app, you may need to install an [adapter](https://svelte.dev/docs/kit/adapters) for your target environment.
```

## File 2: .\components.json

```json
{
	"$schema": "https://shadcn-svelte.com/schema.json",
	"tailwind": {
		"css": "src\\app.css",
		"baseColor": "slate"
	},
	"aliases": {
		"components": "$lib/components",
		"utils": "$lib/utils",
		"ui": "$lib/components/ui",
		"hooks": "$lib/hooks",
		"lib": "$lib"
	},
	"typescript": true,
	"registry": "https://shadcn-svelte.com/registry"
}
```

## File 3: .\eslint.config.js

```js
import prettier from 'eslint-config-prettier';
import { includeIgnoreFile } from '@eslint/compat';
import js from '@eslint/js';
import svelte from 'eslint-plugin-svelte';
import globals from 'globals';
import { fileURLToPath } from 'node:url';
import ts from 'typescript-eslint';
import svelteConfig from './svelte.config.js';

const gitignorePath = fileURLToPath(new URL('./.gitignore', import.meta.url));

export default ts.config(
	includeIgnoreFile(gitignorePath),
	js.configs.recommended,
	...ts.configs.recommended,
	...svelte.configs.recommended,
	prettier,
	...svelte.configs.prettier,
	{
		languageOptions: {
			globals: { ...globals.browser, ...globals.node }
		},
		rules: {
			// typescript-eslint strongly recommend that you do not use the no-undef lint rule on TypeScript projects.
			// see: https://typescript-eslint.io/troubleshooting/faqs/eslint/#i-get-errors-from-the-no-undef-rule-about-global-variables-not-being-defined-even-though-there-are-no-typescript-errors
			'no-undef': 'off'
		}
	},
	{
		files: ['**/*.svelte', '**/*.svelte.ts', '**/*.svelte.js'],
		languageOptions: {
			parserOptions: {
				projectService: true,
				extraFileExtensions: ['.svelte'],
				parser: ts.parser,
				svelteConfig
			}
		}
	}
);
```

## File 4: .\package.json

```json
{
	"name": "liftly",
	"private": true,
	"version": "0.0.1",
	"type": "module",
	"scripts": {
		"dev": "vite dev",
		"build": "vite build",
		"preview": "vite preview",
		"prepare": "svelte-kit sync || echo ''",
		"check": "svelte-kit sync && svelte-check --tsconfig ./tsconfig.json",
		"check:watch": "svelte-kit sync && svelte-check --tsconfig ./tsconfig.json --watch",
		"format": "prettier --write .",
		"lint": "prettier --check . && eslint .",
		"tauri": "tauri",
		"export-bindings": "cargo run --bin export-bindings --manifest-path ./src-tauri/Cargo.toml",
		"android:clear-pkg": "adb shell pm clear fr.requet.liftly"
	},
	"devDependencies": {
		"@eslint/compat": "^1.2.5",
		"@eslint/js": "^9.18.0",
		"@lucide/svelte": "^0.539.0",
		"@sveltejs/adapter-static": "^3.0.9",
		"@sveltejs/kit": "^2.22.0",
		"@sveltejs/vite-plugin-svelte": "^6.0.0",
		"@tailwindcss/vite": "^4.0.0",
		"@tauri-apps/api": "^2.8.0",
		"@tauri-apps/cli": "^2.7.1",
		"@tauri-apps/plugin-opener": "^2.5.0",
		"@tauri-apps/plugin-sql": "^2.3.0",
		"clsx": "^2.1.1",
		"eslint": "^9.18.0",
		"eslint-config-prettier": "^10.0.1",
		"eslint-plugin-svelte": "^3.0.0",
		"globals": "^16.0.0",
		"prettier": "^3.4.2",
		"prettier-plugin-svelte": "^3.3.3",
		"prettier-plugin-tailwindcss": "^0.6.11",
		"svelte": "^5.0.0",
		"svelte-check": "^4.0.0",
		"tailwind-merge": "^3.3.1",
		"tailwind-variants": "^1.0.0",
		"tailwindcss": "^4.0.0",
		"tw-animate-css": "^1.3.7",
		"typescript": "^5.0.0",
		"typescript-eslint": "^8.20.0",
		"vite": "^7.0.4"
	},
	"pnpm": {
		"onlyBuiltDependencies": [
			"esbuild"
		]
	}
}
```

## File 5: .\src\app.css

```css
@import 'tailwindcss';

@import 'tw-animate-css';

@custom-variant dark (&:is(.dark *));

:root {
	--radius: 0.625rem;
	--background: oklch(1 0 0);
	--foreground: oklch(0.129 0.042 264.695);
	--card: oklch(1 0 0);
	--card-foreground: oklch(0.129 0.042 264.695);
	--popover: oklch(1 0 0);
	--popover-foreground: oklch(0.129 0.042 264.695);
	--primary: oklch(0.208 0.042 265.755);
	--primary-foreground: oklch(0.984 0.003 247.858);
	--secondary: oklch(0.968 0.007 247.896);
	--secondary-foreground: oklch(0.208 0.042 265.755);
	--muted: oklch(0.968 0.007 247.896);
	--muted-foreground: oklch(0.554 0.046 257.417);
	--accent: oklch(0.968 0.007 247.896);
	--accent-foreground: oklch(0.208 0.042 265.755);
	--destructive: oklch(0.577 0.245 27.325);
	--border: oklch(0.929 0.013 255.508);
	--input: oklch(0.929 0.013 255.508);
	--ring: oklch(0.704 0.04 256.788);
	--chart-1: oklch(0.646 0.222 41.116);
	--chart-2: oklch(0.6 0.118 184.704);
	--chart-3: oklch(0.398 0.07 227.392);
	--chart-4: oklch(0.828 0.189 84.429);
	--chart-5: oklch(0.769 0.188 70.08);
	--sidebar: oklch(0.984 0.003 247.858);
	--sidebar-foreground: oklch(0.129 0.042 264.695);
	--sidebar-primary: oklch(0.208 0.042 265.755);
	--sidebar-primary-foreground: oklch(0.984 0.003 247.858);
	--sidebar-accent: oklch(0.968 0.007 247.896);
	--sidebar-accent-foreground: oklch(0.208 0.042 265.755);
	--sidebar-border: oklch(0.929 0.013 255.508);
	--sidebar-ring: oklch(0.704 0.04 256.788);
}

.dark {
	--background: oklch(0.129 0.042 264.695);
	--foreground: oklch(0.984 0.003 247.858);
	--card: oklch(0.208 0.042 265.755);
	--card-foreground: oklch(0.984 0.003 247.858);
	--popover: oklch(0.208 0.042 265.755);
	--popover-foreground: oklch(0.984 0.003 247.858);
	--primary: oklch(0.929 0.013 255.508);
	--primary-foreground: oklch(0.208 0.042 265.755);
	--secondary: oklch(0.279 0.041 260.031);
	--secondary-foreground: oklch(0.984 0.003 247.858);
	--muted: oklch(0.279 0.041 260.031);
	--muted-foreground: oklch(0.704 0.04 256.788);
	--accent: oklch(0.279 0.041 260.031);
	--accent-foreground: oklch(0.984 0.003 247.858);
	--destructive: oklch(0.704 0.191 22.216);
	--border: oklch(1 0 0 / 10%);
	--input: oklch(1 0 0 / 15%);
	--ring: oklch(0.551 0.027 264.364);
	--chart-1: oklch(0.488 0.243 264.376);
	--chart-2: oklch(0.696 0.17 162.48);
	--chart-3: oklch(0.769 0.188 70.08);
	--chart-4: oklch(0.627 0.265 303.9);
	--chart-5: oklch(0.645 0.246 16.439);
	--sidebar: oklch(0.208 0.042 265.755);
	--sidebar-foreground: oklch(0.984 0.003 247.858);
	--sidebar-primary: oklch(0.488 0.243 264.376);
	--sidebar-primary-foreground: oklch(0.984 0.003 247.858);
	--sidebar-accent: oklch(0.279 0.041 260.031);
	--sidebar-accent-foreground: oklch(0.984 0.003 247.858);
	--sidebar-border: oklch(1 0 0 / 10%);
	--sidebar-ring: oklch(0.551 0.027 264.364);
}

@theme inline {
	--radius-sm: calc(var(--radius) - 4px);
	--radius-md: calc(var(--radius) - 2px);
	--radius-lg: var(--radius);
	--radius-xl: calc(var(--radius) + 4px);
	--color-background: var(--background);
	--color-foreground: var(--foreground);
	--color-card: var(--card);
	--color-card-foreground: var(--card-foreground);
	--color-popover: var(--popover);
	--color-popover-foreground: var(--popover-foreground);
	--color-primary: var(--primary);
	--color-primary-foreground: var(--primary-foreground);
	--color-secondary: var(--secondary);
	--color-secondary-foreground: var(--secondary-foreground);
	--color-muted: var(--muted);
	--color-muted-foreground: var(--muted-foreground);
	--color-accent: var(--accent);
	--color-accent-foreground: var(--accent-foreground);
	--color-destructive: var(--destructive);
	--color-border: var(--border);
	--color-input: var(--input);
	--color-ring: var(--ring);
	--color-chart-1: var(--chart-1);
	--color-chart-2: var(--chart-2);
	--color-chart-3: var(--chart-3);
	--color-chart-4: var(--chart-4);
	--color-chart-5: var(--chart-5);
	--color-sidebar: var(--sidebar);
	--color-sidebar-foreground: var(--sidebar-foreground);
	--color-sidebar-primary: var(--sidebar-primary);
	--color-sidebar-primary-foreground: var(--sidebar-primary-foreground);
	--color-sidebar-accent: var(--sidebar-accent);
	--color-sidebar-accent-foreground: var(--sidebar-accent-foreground);
	--color-sidebar-border: var(--sidebar-border);
	--color-sidebar-ring: var(--sidebar-ring);
}

@layer base {
	* {
		@apply border-border outline-ring/50;
	}
	html,
	body {
		@apply h-full w-full;
	}
	body {
		@apply bg-background text-foreground;
	}
}
```

## File 6: .\src\app.d.ts

```ts
// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
```

## File 7: .\src\app.html

```html
<!doctype html>
<html lang="en">
	<head>
		<meta charset="utf-8" />
		<meta name="viewport" content="width=device-width, initial-scale=1" />
		%sveltekit.head%
	</head>
	<body data-sveltekit-preload-data="hover">
		<div style="display: contents">%sveltekit.body%</div>
	</body>
</html>
```

## File 8: .\src\lib\bindings.ts

```ts

// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

/** user-defined commands **/


export const commands = {
async helloWorld(myName: string) : Promise<string> {
    return await TAURI_INVOKE("hello_world", { myName });
},
async goodbyeWorld() : Promise<string> {
    return await TAURI_INVOKE("goodbye_world");
},
async getDbTables() : Promise<Result<string[], string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("get_db_tables") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async anotherCommand(data: MyStruct) : Promise<Result<MyResponse, string>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("another_command", { data }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async printLog() : Promise<void> {
    await TAURI_INVOKE("print_log");
}
}

/** user-defined events **/



/** user-defined constants **/



/** user-defined types **/

export type MyResponse = { message: string }
export type MyStruct = { a: string }

/** tauri-specta globals **/

import {
	invoke as TAURI_INVOKE,
	Channel as TAURI_CHANNEL,
} from "@tauri-apps/api/core";
import * as TAURI_API_EVENT from "@tauri-apps/api/event";
import { type WebviewWindow as __WebviewWindow__ } from "@tauri-apps/api/webviewWindow";

type __EventObj__<T> = {
	listen: (
		cb: TAURI_API_EVENT.EventCallback<T>,
	) => ReturnType<typeof TAURI_API_EVENT.listen<T>>;
	once: (
		cb: TAURI_API_EVENT.EventCallback<T>,
	) => ReturnType<typeof TAURI_API_EVENT.once<T>>;
	emit: null extends T
		? (payload?: T) => ReturnType<typeof TAURI_API_EVENT.emit>
		: (payload: T) => ReturnType<typeof TAURI_API_EVENT.emit>;
};

export type Result<T, E> =
	| { status: "ok"; data: T }
	| { status: "error"; error: E };

function __makeEvents__<T extends Record<string, any>>(
	mappings: Record<keyof T, string>,
) {
	return new Proxy(
		{} as unknown as {
			[K in keyof T]: __EventObj__<T[K]> & {
				(handle: __WebviewWindow__): __EventObj__<T[K]>;
			};
		},
		{
			get: (_, event) => {
				const name = mappings[event as keyof T];

				return new Proxy((() => {}) as any, {
					apply: (_, __, [window]: [__WebviewWindow__]) => ({
						listen: (arg: any) => window.listen(name, arg),
						once: (arg: any) => window.once(name, arg),
						emit: (arg: any) => window.emit(name, arg),
					}),
					get: (_, command: keyof __EventObj__<any>) => {
						switch (command) {
							case "listen":
								return (arg: any) => TAURI_API_EVENT.listen(name, arg);
							case "once":
								return (arg: any) => TAURI_API_EVENT.once(name, arg);
							case "emit":
								return (arg: any) => TAURI_API_EVENT.emit(name, arg);
						}
					},
				});
			},
		},
	);
}
```

## File 9: .\src\lib\components\layout\bottom-nav.svelte

```svelte
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
```

## File 10: .\src\lib\components\layout\page-wrapper.svelte

```svelte
<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		title?: string;
		subtitle?: string;
		children: Snippet<[]>;
	}

	let { title, subtitle, children }: Props = $props();
</script>

<div class="flex flex-1 flex-col gap-8 px-2">
	{#if title}
		<header>
			<h1 class="text-2xl font-bold">{title}</h1>
			{#if subtitle}
				<p class="text-muted-foreground">{subtitle}</p>
			{/if}
		</header>
	{/if}

	{@render children()}
</div>
```

## File 11: .\src\lib\components\ui\button\button.svelte

```svelte
<script lang="ts" module>
	import { cn, type WithElementRef } from "$lib/utils.js";
	import type { HTMLAnchorAttributes, HTMLButtonAttributes } from "svelte/elements";
	import { type VariantProps, tv } from "tailwind-variants";

	export const buttonVariants = tv({
		base: "focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive inline-flex shrink-0 items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium outline-none transition-all focus-visible:ring-[3px] disabled:pointer-events-none disabled:opacity-50 aria-disabled:pointer-events-none aria-disabled:opacity-50 [&_svg:not([class*='size-'])]:size-4 [&_svg]:pointer-events-none [&_svg]:shrink-0",
		variants: {
			variant: {
				default: "bg-primary text-primary-foreground shadow-xs hover:bg-primary/90",
				destructive:
					"bg-destructive shadow-xs hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60 text-white",
				outline:
					"bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/50 border",
				secondary: "bg-secondary text-secondary-foreground shadow-xs hover:bg-secondary/80",
				ghost: "hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50",
				link: "text-primary underline-offset-4 hover:underline",
			},
			size: {
				default: "h-9 px-4 py-2 has-[>svg]:px-3",
				sm: "h-8 gap-1.5 rounded-md px-3 has-[>svg]:px-2.5",
				lg: "h-10 rounded-md px-6 has-[>svg]:px-4",
				icon: "size-9",
			},
		},
		defaultVariants: {
			variant: "default",
			size: "default",
		},
	});

	export type ButtonVariant = VariantProps<typeof buttonVariants>["variant"];
	export type ButtonSize = VariantProps<typeof buttonVariants>["size"];

	export type ButtonProps = WithElementRef<HTMLButtonAttributes> &
		WithElementRef<HTMLAnchorAttributes> & {
			variant?: ButtonVariant;
			size?: ButtonSize;
		};
</script>

<script lang="ts">
	let {
		class: className,
		variant = "default",
		size = "default",
		ref = $bindable(null),
		href = undefined,
		type = "button",
		disabled,
		children,
		...restProps
	}: ButtonProps = $props();
</script>

{#if href}
	<a
		bind:this={ref}
		data-slot="button"
		class={cn(buttonVariants({ variant, size }), className)}
		href={disabled ? undefined : href}
		aria-disabled={disabled}
		role={disabled ? "link" : undefined}
		tabindex={disabled ? -1 : undefined}
		{...restProps}
	>
		{@render children?.()}
	</a>
{:else}
	<button
		bind:this={ref}
		data-slot="button"
		class={cn(buttonVariants({ variant, size }), className)}
		{type}
		{disabled}
		{...restProps}
	>
		{@render children?.()}
	</button>
{/if}
```

## File 12: .\src\lib\components\ui\button\index.ts

```ts
import Root, {
	type ButtonProps,
	type ButtonSize,
	type ButtonVariant,
	buttonVariants,
} from "./button.svelte";

export {
	Root,
	type ButtonProps as Props,
	//
	Root as Button,
	buttonVariants,
	type ButtonProps,
	type ButtonSize,
	type ButtonVariant,
};
```

## File 13: .\src\lib\components\ui\card\card-action.svelte

```svelte
<script lang="ts">
	import { cn, type WithElementRef } from "$lib/utils.js";
	import type { HTMLAttributes } from "svelte/elements";

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props();
</script>

<div
	bind:this={ref}
	data-slot="card-action"
	class={cn("col-start-2 row-span-2 row-start-1 self-start justify-self-end", className)}
	{...restProps}
>
	{@render children?.()}
</div>
```

## File 14: .\src\lib\components\ui\card\card-content.svelte

```svelte
<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements";
	import { cn, type WithElementRef } from "$lib/utils.js";

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props();
</script>

<div bind:this={ref} data-slot="card-content" class={cn("px-6", className)} {...restProps}>
	{@render children?.()}
</div>
```

## File 15: .\src\lib\components\ui\card\card-description.svelte

```svelte
<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements";
	import { cn, type WithElementRef } from "$lib/utils.js";

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLParagraphElement>> = $props();
</script>

<p
	bind:this={ref}
	data-slot="card-description"
	class={cn("text-muted-foreground text-sm", className)}
	{...restProps}
>
	{@render children?.()}
</p>
```

## File 16: .\src\lib\components\ui\card\card-footer.svelte

```svelte
<script lang="ts">
	import { cn, type WithElementRef } from "$lib/utils.js";
	import type { HTMLAttributes } from "svelte/elements";

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props();
</script>

<div
	bind:this={ref}
	data-slot="card-footer"
	class={cn("[.border-t]:pt-6 flex items-center px-6", className)}
	{...restProps}
>
	{@render children?.()}
</div>
```

## File 17: .\src\lib\components\ui\card\card-header.svelte

```svelte
<script lang="ts">
	import { cn, type WithElementRef } from "$lib/utils.js";
	import type { HTMLAttributes } from "svelte/elements";

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props();
</script>

<div
	bind:this={ref}
	data-slot="card-header"
	class={cn(
		"@container/card-header has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-6 grid auto-rows-min grid-rows-[auto_auto] items-start gap-1.5 px-6",
		className
	)}
	{...restProps}
>
	{@render children?.()}
</div>
```

## File 18: .\src\lib\components\ui\card\card-title.svelte

```svelte
<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements";
	import { cn, type WithElementRef } from "$lib/utils.js";

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props();
</script>

<div
	bind:this={ref}
	data-slot="card-title"
	class={cn("font-semibold leading-none", className)}
	{...restProps}
>
	{@render children?.()}
</div>
```

## File 19: .\src\lib\components\ui\card\card.svelte

```svelte
<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements";
	import { cn, type WithElementRef } from "$lib/utils.js";

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLAttributes<HTMLDivElement>> = $props();
</script>

<div
	bind:this={ref}
	data-slot="card"
	class={cn(
		"bg-card text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm",
		className
	)}
	{...restProps}
>
	{@render children?.()}
</div>
```

## File 20: .\src\lib\components\ui\card\index.ts

```ts
import Root from "./card.svelte";
import Content from "./card-content.svelte";
import Description from "./card-description.svelte";
import Footer from "./card-footer.svelte";
import Header from "./card-header.svelte";
import Title from "./card-title.svelte";
import Action from "./card-action.svelte";

export {
	Root,
	Content,
	Description,
	Footer,
	Header,
	Title,
	Action,
	//
	Root as Card,
	Content as CardContent,
	Description as CardDescription,
	Footer as CardFooter,
	Header as CardHeader,
	Title as CardTitle,
	Action as CardAction,
};
```

## File 21: .\src\lib\index.ts

```ts
// place files you want to import through the `$lib` alias in this folder.
```

## File 22: .\src\lib\services\datetime.ts

```ts
export const formatElapsedTime = (start: Date, end: Date): string => {
	const diff = end.getTime() - start.getTime();
	return formatTimeDiff(diff);
};

const formatTimeDiff = (diff: number, useNanoSeconds: boolean = false): string => {
	const hours = Math.floor(diff / 3600000);
	const minutes = Math.floor((diff % 3600000) / 60000);
	const seconds = Math.floor((diff % 60000) / 1000);
	const milliseconds = diff % 1000;

	const formattedResult = `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;

	if (useNanoSeconds) {
		return `${formattedResult}.${milliseconds.toString().padStart(3, '0')}`;
	}

	return formattedResult;
};
```

## File 23: .\src\lib\utils.ts

```ts
import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChild<T> = T extends { child?: any } ? Omit<T, "child"> : T;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChildren<T> = T extends { children?: any } ? Omit<T, "children"> : T;
export type WithoutChildrenOrChild<T> = WithoutChildren<WithoutChild<T>>;
export type WithElementRef<T, U extends HTMLElement = HTMLElement> = T & { ref?: U | null };
```

## File 24: .\src\routes\+layout.svelte

```svelte
<script lang="ts">
	import '../app.css';

	let { children } = $props();
</script>

<div class="flex h-full flex-col">
	<main class="flex-1 overflow-y-auto pt-12">
		{@render children?.()}
	</main>
</div>
```

## File 25: .\src\routes\+layout.ts

```ts
export const ssr = false;
```

## File 26: .\src\routes\+page.ts

```ts
import { redirect } from '@sveltejs/kit';

export function load() {
	throw redirect(307, '/main/home');
}
```

## File 27: .\src\routes\active-workout\+page.svelte

```svelte
<script>
	import PageWrapper from '$lib/components/layout/page-wrapper.svelte';
	import { Button } from '$lib/components/ui/button';
	import { formatElapsedTime } from '$lib/services/datetime';
	import { ArrowLeft, CircleCheckBig } from '@lucide/svelte';
	import { onMount } from 'svelte';

	let startDateTime = new Date();
	let elapsedTime = $state(formatElapsedTime(startDateTime, startDateTime));

	onMount(() => {
		const interval = setInterval(() => {
			elapsedTime = formatElapsedTime(startDateTime, new Date());
		}, 1000);

		return () => clearInterval(interval);
	});
</script>

<header class="flex w-full flex-row items-center border-b p-2">
	<a href="/main/home" class="flex h-auto flex-col items-center text-sm">
		<Button variant="ghost" size="icon">
			<ArrowLeft class="size-6" />
		</Button>
	</a>
	<div class="flex flex-1 justify-center">
		<p class="flex items-center">{elapsedTime}</p>
	</div>
	<Button variant="ghost" size="icon">
		<CircleCheckBig class="size-6" />
	</Button>
</header>

<PageWrapper>salut</PageWrapper>
```

## File 28: .\src\routes\main\+layout.svelte

```svelte
<script lang="ts">
	import BottomNav from '$lib/components/layout/bottom-nav.svelte';

	let { children } = $props();
</script>

<div class="flex h-full flex-col justify-between">
	{@render children?.()}
	<BottomNav />
</div>
```

## File 29: .\src\routes\main\home\+page.svelte

```svelte
<script lang="ts">
	import PageWrapper from '$lib/components/layout/page-wrapper.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Card } from '$lib/components/ui/card';
	import { Plus } from '@lucide/svelte';
</script>

<PageWrapper title="Good morning!" subtitle="Ready to crush today's workout?">
	<a href="/active-workout">
		<Button size="lg" class="w-full">
			<Plus class="mr-2 size-5" />
			Start New Workout
		</Button>
	</a>

	<section class="flex flex-col gap-4">
		<h2 class="text-lg font-semibold">Recent Activity (fake)</h2>

		<!-- Placeholder Card 1 -->
		<Card class="p-4">
			<div class="flex items-center justify-between">
				<div>
					<h3 class="font-semibold">Push Day</h3>
					<p class="text-sm text-muted-foreground">Yesterday</p>
				</div>
				<span class="text-sm font-medium">5 Exercises</span>
			</div>
		</Card>

		<!-- Placeholder Card 2 -->
		<Card class="p-4">
			<div class="flex items-center justify-between">
				<div>
					<h3 class="font-semibold">Leg Day</h3>
					<p class="text-sm text-muted-foreground">3 days ago</p>
				</div>
				<span class="text-sm font-medium">6 Exercises</span>
			</div>
		</Card>
	</section>
</PageWrapper>
```

## File 30: .\src\routes\main\profile\+page.svelte

```svelte
<script lang="ts">
	import PageWrapper from '$lib/components/layout/page-wrapper.svelte';
	import { getName, getVersion } from '@tauri-apps/api/app';
	import { onMount } from 'svelte';

	interface AppData {
		name: string;
		version: string;
	}

	let appData: AppData = $state({ name: '', version: '' });

	onMount(async () => {
		const name = await getName();
		const version = await getVersion();

		appData = {
			name,
			version
		};
	});
</script>

<PageWrapper title="Profile">
	<p class="align text-center text-muted-foreground">{appData.name} - {appData.version}</p>
</PageWrapper>
```

## File 31: .\src\routes\main\workouts\+page.svelte

```svelte
<script>
	import PageWrapper from '$lib/components/layout/page-wrapper.svelte';
</script>

<PageWrapper title="This is workouts page">-</PageWrapper>
```

## File 32: .\src-tauri\Cargo.toml

```toml
[package]
name = "liftly"
version = "0.1.0"
description = "Liftly App"
authors = ["brequet"]
license = ""
repository = ""
edition = "2021"
rust-version = "1.77.2"
default-run = "liftly"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[lib]
name = "app_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[[bin]]
name = "export-bindings"
path = "src/bin/export_bindings.rs"

[build-dependencies]
tauri-build = { version = "2.3.1", features = [] }

[dependencies]
serde_json = "1.0"
serde = { version = "1.0", features = ["derive"] }
log = "0.4"
tauri = { version = "2.7.0", features = [] }
tauri-plugin-log = "2"
specta = "=2.0.0-rc.22"
tauri-specta = { version = "=2.0.0-rc.21", features = ["derive", "typescript"] }
specta-typescript = "0.0.9"
tauri-plugin-sql = { version = "2", features = ["sqlite"] }
sqlx = { version = "0.8.6", features = ["sqlite"] }
once_cell = "1.21.3"
```

## File 33: .\src-tauri\build.rs

```rs
fn main() {
    tauri_build::build();
}
```

## File 34: .\src-tauri\capabilities\default.json

```json
{
	"$schema": "../gen/schemas/desktop-schema.json",
	"identifier": "default",
	"description": "enables the default permissions",
	"windows": ["main"],
	"permissions": ["core:default", "sql:default", "sql:allow-execute"]
}
```

## File 35: .\src-tauri\src\bin\export_bindings.rs

```rs
use specta_typescript::Typescript;

fn main() {
    let output_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "./src/lib/bindings.ts".to_string());

    let builder = app_lib::commands::specta_builder();

    println!("Generating TypeScript bindings to: {}", output_path);

    builder
        .export(Typescript::default(), &output_path)
        .expect("Failed to export typescript bindings");

    println!("✅ TypeScript bindings generated successfully!");
}
```

## File 36: .\src-tauri\src\commands\api1.rs

```rs
use tauri::State;

use crate::db::connection::DbPool;

#[tauri::command]
#[specta::specta]
pub fn hello_world(my_name: String) -> String {
    format!("Hello, {my_name}! You've been greeted from Rust!")
}

#[tauri::command]
#[specta::specta]
pub fn goodbye_world() -> String {
    "Goodbye!".to_string()
}

#[tauri::command]
#[specta::specta]
pub async fn get_db_tables(pool: State<'_, DbPool>) -> Result<Vec<String>, String> {
    let query = "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'";

    let table_names = sqlx::query_as::<_, (String,)>(query)
        .fetch_all(&pool.0)
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(|(name,)| name)
        .collect();

    Ok(table_names)
}
```

## File 37: .\src-tauri\src\commands\api2.rs

```rs
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Type)]
pub struct MyStruct {
    a: String,
}

#[derive(Serialize, Deserialize, Type)]
pub struct MyResponse {
    message: String,
}

#[tauri::command]
#[specta::specta]
pub fn another_command(data: MyStruct) -> Result<MyResponse, String> {
    Ok(MyResponse {
        message: format!("Received: {}", data.a),
    })
}

#[tauri::command]
#[specta::specta]
pub fn print_log() {
    println!("Log message");
}
```

## File 38: .\src-tauri\src\commands\mod.rs

```rs
use tauri_specta::{collect_commands, Builder, Commands};

mod api1;
mod api2;

macro_rules! combine_commands {
    ( $( $module:ident : [ $( $command:ident ),* ] ),* ) => {
        collect_commands![
            $( $( $module::$command ),* ),*
        ]
    };
}

fn get_all_commands() -> Commands<tauri::Wry> {
    combine_commands!(
        api1: [
            hello_world,
            goodbye_world,
            get_db_tables
        ],
        api2: [
            another_command,
            print_log
        ]
    )
}

pub fn specta_builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new().commands(get_all_commands())
}
```

## File 39: .\src-tauri\src\db\connection.rs

```rs
use once_cell::sync::Lazy;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tauri_plugin_sql::{Migration, MigrationKind};

const DB_NAME: &str = "liftly.db";
static DB_URL: Lazy<String> = Lazy::new(|| format!("sqlite:{}", DB_NAME));

pub struct DbPool(pub Pool<Sqlite>);

fn get_app_data_dir(app: &AppHandle) -> PathBuf {
    app.path()
        .app_data_dir()
        .expect("Failed to get app data directory")
}

pub async fn init_db_pool(app: &AppHandle) -> Result<DbPool, sqlx::Error> {
    let app_data_dir = get_app_data_dir(app);
    let db_path = app_data_dir.join(DB_NAME);

    if !db_path.exists() {
        std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data directory");
        std::fs::File::create(&db_path).expect("Failed to create database file");
    }

    log::info!("Database path: {:?}", db_path);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_path.to_str().unwrap())
        .await?;
    Ok(DbPool(pool))
}

pub fn get_db_plugin_config() -> (String, Vec<Migration>) {
    (DB_URL.to_string(), get_migrations())
}

fn get_migrations() -> Vec<Migration> {
    vec![Migration {
        version: 1,
        description: "create_users_table",
        sql: "CREATE TABLE IF NOT EXISTS users (
                  id    INTEGER PRIMARY KEY AUTOINCREMENT,
                  name  TEXT NOT NULL,
                  email TEXT UNIQUE NOT NULL
              )",
        kind: MigrationKind::Up,
    }]
}
```

## File 40: .\src-tauri\src\db\mod.rs

```rs
pub mod connection;
```

## File 41: .\src-tauri\src\lib.rs

```rs
use tauri::Manager;

pub mod db;

pub mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = commands::specta_builder();

    let (db_plugin_url, migrations) = db::connection::get_db_plugin_config();

    let mut log_builder = tauri_plugin_log::Builder::new();

    if cfg!(debug_assertions) {
        log_builder = log_builder.level(log::LevelFilter::Info);
    }

    tauri::Builder::default()
        .plugin(log_builder.build())
        .plugin(
            tauri_plugin_sql::Builder::new()
                .add_migrations(&db_plugin_url, migrations)
                .build(),
        )
        .setup(|app| {
            let pool = tauri::async_runtime::block_on(db::connection::init_db_pool(app.handle()))?;
            app.manage(pool);
            // builder.mount_events(app); //TODO for events
            Ok(())
        })
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## File 42: .\src-tauri\src\main.rs

```rs
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    app_lib::run()
}
```

## File 43: .\src-tauri\tauri.conf.json

```json
{
	"$schema": "../node_modules/@tauri-apps/cli/config.schema.json",
	"productName": "liftly",
	"version": "0.1.0",
	"identifier": "fr.requet.liftly",
	"build": {
		"frontendDist": "../build",
		"devUrl": "http://localhost:1420",
		"beforeDevCommand": "pnpm dev",
		"beforeBuildCommand": "pnpm build"
	},
	"app": {
		"windows": [
			{
				"title": "liftly",
				"width": 800,
				"height": 600,
				"resizable": true,
				"fullscreen": false
			}
		],
		"security": {
			"csp": null
		}
	},
	"bundle": {
		"active": true,
		"targets": "all",
		"icon": [
			"icons/32x32.png",
			"icons/128x128.png",
			"icons/128x128@2x.png",
			"icons/icon.icns",
			"icons/icon.ico"
		]
	}
}
```

## File 44: .\svelte.config.js

```js
import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter({
			fallback: 'index.html'
		})
	}
};

export default config;
```

## File 45: .\tsconfig.json

```json
{
	"extends": "./.svelte-kit/tsconfig.json",
	"compilerOptions": {
		"allowJs": true,
		"checkJs": true,
		"esModuleInterop": true,
		"forceConsistentCasingInFileNames": true,
		"resolveJsonModule": true,
		"skipLibCheck": true,
		"sourceMap": true,
		"strict": true,
		"moduleResolution": "bundler"
	}
	// Path aliases are handled by https://svelte.dev/docs/kit/configuration#alias
	// except $lib which is handled by https://svelte.dev/docs/kit/configuration#files
	//
	// To make changes to top-level options such as include and exclude, we recommend extending
	// the generated config; see https://svelte.dev/docs/kit/configuration#typescript
}
```

## File 46: .\vite.config.ts

```ts
import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],

	// Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
	//
	// 1. prevent Vite from obscuring rust errors
	clearScreen: false,
	// 2. tauri expects a fixed port, fail if that port is not available
	server: {
		port: 1420,
		strictPort: true,
		host: host || false,
		hmr: host
			? {
					protocol: 'ws',
					host,
					port: 1421
				}
			: undefined,
		watch: {
			// 3. tell Vite to ignore watching `src-tauri`
			ignored: ['**/src-tauri/**']
		}
	}
});
```

