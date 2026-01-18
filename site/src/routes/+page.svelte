<script lang="ts">
</script>

<div
	class="min-h-screen bg-white text-gray-900 transition-colors dark:bg-gray-900 dark:text-gray-100"
>
	<!-- Hero Section -->
	<header class="container mx-auto px-4 py-16 text-center">
		<div class="mx-auto max-w-3xl">
			<h1
				class="mb-4 bg-linear-to-r from-blue-600 to-purple-600 bg-clip-text text-5xl font-bold text-transparent dark:from-blue-400 dark:to-purple-400"
			>
				Claude AFK
			</h1>
			<p class="mb-8 text-xl text-gray-600 dark:text-gray-300">
				Step away from your desk while Claude codes. Get notified on your phone when it needs you.
			</p>
			<div class="flex flex-wrap justify-center gap-4">
				<a
					href="#quick-start"
					class="rounded-lg bg-blue-600 px-6 py-3 font-semibold text-white transition-colors hover:bg-blue-700"
				>
					Get Started
				</a>
				<a
					href="https://github.com/JazzyMcJazz/claude-code-afk"
					target="_blank"
					rel="noopener noreferrer"
					class="rounded-lg border-2 border-gray-300 px-6 py-3 font-semibold transition-colors hover:border-blue-600 dark:border-gray-600 dark:hover:border-blue-400"
				>
					View on GitHub
				</a>
			</div>
		</div>
	</header>

	<!-- Features Section -->
	<section class="container mx-auto px-4 py-16">
		<div class="mx-auto grid max-w-5xl gap-8 md:grid-cols-3">
			<div class="rounded-xl bg-gray-50 p-6 dark:bg-gray-800">
				<div class="mb-4 text-3xl">📱</div>
				<h3 class="mb-2 text-xl font-semibold">Push Notifications</h3>
				<p class="text-gray-600 dark:text-gray-400">
					Receive instant notifications on your phone when Claude Code requires input - no app required
				</p>
			</div>
			<div class="rounded-xl bg-gray-50 p-6 dark:bg-gray-800">
				<div class="mb-4 text-3xl">🔒</div>
				<h3 class="mb-2 text-xl font-semibold">Secure Pairing</h3>
				<p class="text-gray-600 dark:text-gray-400">
					Simple QR code pairing with secure device tokens and Web Push API
				</p>
			</div>
			<div class="rounded-xl bg-gray-50 p-6 dark:bg-gray-800">
				<div class="mb-4 text-3xl">⚡</div>
				<h3 class="mb-2 text-xl font-semibold">Lightweight</h3>
				<p class="text-gray-600 dark:text-gray-400">
					Fast and efficient CLI that starts instantly without slowing down your workflow
				</p>
			</div>
		</div>
	</section>

	<!-- When to Use -->
	<section class="container mx-auto px-4 py-16">
		<div class="mx-auto max-w-4xl">
			<h2 class="mb-4 text-center text-3xl font-bold">Perfect For</h2>
			<p class="mb-8 text-center text-gray-600 dark:text-gray-400">
				Get notified when Claude requests tool permissions or asks you a question
			</p>
			<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
				<div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
					<p class="font-medium">Running long builds or tests</p>
				</div>
				<div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
					<p class="font-medium">Letting Claude refactor while you take a break</p>
				</div>
				<div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
					<p class="font-medium">Working on multiple projects</p>
				</div>
				<div class="rounded-lg border border-gray-200 p-4 dark:border-gray-700">
					<p class="font-medium">Staying productive away from your desk</p>
				</div>
			</div>
		</div>
	</section>

	<!-- How It Works -->
	<section class="container mx-auto bg-gray-50 px-4 py-16 dark:bg-gray-800">
		<div class="mx-auto max-w-4xl">
			<h2 class="mb-12 text-center text-3xl font-bold">How It Works</h2>
			<div class="space-y-8">
				<div class="flex gap-4">
					<div
						class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-blue-600 font-bold text-white"
					>
						1
					</div>
					<div>
						<h4 class="mb-1 font-semibold">Pair Your Device</h4>
						<p class="text-gray-600 dark:text-gray-400">
							Run the CLI setup command and scan the QR code with your phone
						</p>
					</div>
				</div>
				<div class="flex gap-4">
					<div
						class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-blue-600 font-bold text-white"
					>
						2
					</div>
					<div>
						<h4 class="mb-1 font-semibold">Grant Permissions</h4>
						<p class="text-gray-600 dark:text-gray-400">
							Enable push notifications when prompted in your browser
						</p>
					</div>
				</div>
				<div class="flex gap-4">
					<div
						class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-blue-600 font-bold text-white"
					>
						3
					</div>
					<div>
						<h4 class="mb-1 font-semibold">Install Hooks</h4>
						<p class="text-gray-600 dark:text-gray-400">
							Run a single command to automatically configure Claude Code hooks
						</p>
					</div>
				</div>
				<div class="flex gap-4">
					<div
						class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-blue-600 font-bold text-white"
					>
						4
					</div>
					<div>
						<h4 class="mb-1 font-semibold">Stay Notified</h4>
						<p class="text-gray-600 dark:text-gray-400">
							Get push notifications whenever Claude needs your attention
						</p>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Notification Interactions -->
	<section class="container mx-auto px-4 py-16">
		<div class="mx-auto max-w-4xl">
			<h2 class="mb-8 text-center text-3xl font-bold">Responding to Notifications</h2>
			<p class="mb-8 text-center text-gray-600 dark:text-gray-400">
				When Claude Code requests permission, you can respond directly from your phone
			</p>
			<div class="grid gap-6 md:grid-cols-2">
				<div class="rounded-xl bg-gray-50 p-6 dark:bg-gray-800">
					<div class="mb-4 text-3xl">👆</div>
					<h3 class="mb-2 text-xl font-semibold">Tap to Allow</h3>
					<p class="text-gray-600 dark:text-gray-400">
						Tapping the notification body automatically sends "allow" to Claude Code, granting
						the requested permission
					</p>
				</div>
				<div class="rounded-xl bg-gray-50 p-6 dark:bg-gray-800">
					<div class="mb-4 text-3xl">🔘</div>
					<h3 class="mb-2 text-xl font-semibold">Action Buttons</h3>
					<p class="text-gray-600 dark:text-gray-400">
						Use the "Allow" action button for quick responses, or "Dismiss" to clear the
						notification without responding
					</p>
				</div>
			</div>

			<!-- Limitations -->
			<div class="mt-8 rounded-xl border border-amber-200 bg-amber-50 p-6 dark:border-amber-800 dark:bg-amber-950">
				<h3 class="mb-3 flex items-center gap-2 text-lg font-semibold text-amber-800 dark:text-amber-200">
					<span>⚠️</span>
					<span>Web Notification Limitations</span>
				</h3>
				<ul class="space-y-2 text-amber-700 dark:text-amber-300">
					<li class="flex gap-2">
						<span class="shrink-0">•</span>
						<span>Action buttons may not appear on all devices or browsers - some platforms only support basic notifications</span>
					</li>
					<li class="flex gap-2">
						<span class="shrink-0">•</span>
						<span>iOS requires adding this site to your home screen for notifications to work</span>
					</li>
					<li class="flex gap-2">
						<span class="shrink-0">•</span>
						<span>Notifications may be delayed by battery optimization settings on mobile devices</span>
					</li>
					<li class="flex gap-2">
						<span class="shrink-0">•</span>
						<span>The browser or PWA must remain installed - clearing browser data will break notifications</span>
					</li>
				</ul>
			</div>
		</div>
	</section>

	<!-- Quick Start -->
	<section id="quick-start" class="container mx-auto px-4 py-16">
		<div class="mx-auto max-w-3xl">
			<h2 class="mb-8 text-center text-3xl font-bold">Quick Start</h2>

			<!-- Prerequisites -->
			<div class="mb-6 rounded-lg border border-blue-200 bg-blue-50 p-4 dark:border-blue-800 dark:bg-blue-950">
				<p class="text-blue-800 dark:text-blue-200">
					<span class="font-semibold">Prerequisite:</span> Rust must be installed.
					<a
						href="https://rustup.rs"
						target="_blank"
						rel="noopener noreferrer"
						class="underline hover:no-underline"
					>
						Install Rust
					</a>
				</p>
			</div>

			<div class="rounded-xl bg-gray-900 p-6 text-gray-100">
				<code class="block rounded-lg bg-black/40 p-4">
					<span class="block text-gray-400"># Install the CLI</span>
					<span class="block text-green-400">cargo install claude-afk</span>
					<br />
					<span class="block text-gray-400"># Pair your device</span>
					<span class="block text-green-400">claude-afk pair</span>
					<br />
					<span class="block text-gray-400"># Install Claude Code hooks</span>
					<span class="block text-green-400">claude-afk install-hooks</span>
					<br />
					<span class="block text-gray-400"># Enable notifications</span>
					<span class="block text-green-400">claude-afk afk</span>
					<br />
					<span class="block text-gray-400"># Disable notifications when you are back</span>
					<span class="block text-green-400">claude-afk back</span>
				</code>
			</div>
		</div>
	</section>

	<!-- Self-Hosting & Compatibility -->
	<section class="container mx-auto bg-gray-50 px-4 py-16 dark:bg-gray-800">
		<div class="mx-auto max-w-4xl">
			<div class="grid gap-8 md:grid-cols-2">
				<div>
					<h3 class="mb-4 text-xl font-bold">Self-Hosting</h3>
					<p class="mb-4 text-gray-600 dark:text-gray-400">
						Want to run your own backend? Claude AFK can be self-hosted using Docker.
					</p>
					<a
						href="https://github.com/JazzyMcJazz/claude-code-afk#self-hosting"
						target="_blank"
						rel="noopener noreferrer"
						class="font-medium text-blue-600 hover:underline dark:text-blue-400"
					>
						View self-hosting guide
					</a>
				</div>
				<div>
					<h3 class="mb-4 text-xl font-bold">Platform Support</h3>
					<ul class="space-y-2 text-gray-600 dark:text-gray-400">
						<li>Android: Chrome, Firefox, Edge</li>
						<li>iOS: Safari (add to home screen)</li>
						<li>Desktop: All major browsers</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Footer -->
	<footer
		class="container mx-auto border-t border-gray-200 px-4 py-8 dark:border-gray-700"
	>
		<div class="flex flex-col items-center gap-4">
			<div class="flex flex-wrap justify-center gap-6 text-sm">
				<a
					href="https://github.com/JazzyMcJazz/claude-code-afk"
					target="_blank"
					rel="noopener noreferrer"
					class="text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200"
				>
					GitHub
				</a>
				<a
					href="https://github.com/JazzyMcJazz/claude-code-afk#readme"
					target="_blank"
					rel="noopener noreferrer"
					class="text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200"
				>
					Documentation
				</a>
				<a
					href="https://github.com/JazzyMcJazz/claude-code-afk/issues"
					target="_blank"
					rel="noopener noreferrer"
					class="text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200"
				>
					Report an Issue
				</a>
			</div>
			<p class="text-sm text-gray-500 dark:text-gray-400">
				Open source project built with SvelteKit, Rust, and Web Push API
			</p>
		</div>
	</footer>
</div>
