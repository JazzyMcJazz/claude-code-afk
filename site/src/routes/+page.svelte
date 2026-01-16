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
				Claude Code AFK
			</h1>
			<p class="mb-8 text-xl text-gray-600 dark:text-gray-300">
				Get push notifications to your phone when Claude Code needs your attention
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
					Receive instant notifications on your phone when Claude Code requires input
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
					Minimal Rust CLI with no async runtime overhead
				</p>
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
						<h4 class="mb-1 font-semibold">Configure Claude Code</h4>
						<p class="text-gray-600 dark:text-gray-400">
							Add the notification hook to your Claude Code configuration
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

	<!-- Architecture -->
	<section class="container mx-auto px-4 py-16">
		<div class="mx-auto max-w-4xl">
			<h2 class="mb-8 text-center text-3xl font-bold">Architecture</h2>
			<div class="overflow-x-auto rounded-xl bg-gray-50 p-8 dark:bg-gray-800">
				<div class="flex min-w-max items-center justify-center gap-4">
					<div class="text-center">
						<div class="rounded-lg bg-white px-4 py-3 shadow dark:bg-gray-700">Claude Code</div>
					</div>
					<div class="text-2xl">→</div>
					<div class="text-center">
						<div class="rounded-lg bg-white px-4 py-3 shadow dark:bg-gray-700">CLI Tool</div>
						<div class="mt-1 text-xs text-gray-500">Rust</div>
					</div>
					<div class="text-2xl">→</div>
					<div class="text-center">
						<div class="rounded-lg bg-white px-4 py-3 shadow dark:bg-gray-700">Backend</div>
						<div class="mt-1 text-xs text-gray-500">SvelteKit</div>
					</div>
					<div class="text-2xl">→</div>
					<div class="text-center">
						<div class="rounded-lg bg-white px-4 py-3 shadow dark:bg-gray-700">Your Phone</div>
						<div class="mt-1 text-xs text-gray-500">Web Push</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Quick Start -->
	<section id="quick-start" class="container mx-auto px-4 py-16">
		<div class="mx-auto max-w-3xl">
			<h2 class="mb-8 text-center text-3xl font-bold">Quick Start</h2>
			<div class="rounded-xl bg-gray-900 p-6 text-gray-100">
				<div class="space-y-4 font-mono text-sm">
					<div>
						<div class="text-gray-400"># Install the CLI</div>
						<div class="text-green-400">cargo build --release</div>
					</div>
					<div>
						<div class="text-gray-400"># Pair your device</div>
						<div class="text-green-400">claude-afk setup</div>
					</div>
					<div>
						<div class="text-gray-400"># Test notification</div>
						<div class="text-green-400">
							echo '&#123;"message":"Hello!"&#125;' | claude-afk notify
						</div>
					</div>
				</div>
			</div>
			<div class="mt-4 text-center">
				<a
					href="https://github.com/JazzyMcJazz/claude-code-afk"
					target="_blank"
					rel="noopener noreferrer"
					class="text-blue-600 hover:underline dark:text-blue-400"
				>
					View full documentation →
				</a>
			</div>
		</div>
	</section>

	<!-- Footer -->
	<footer
		class="container mx-auto border-t border-gray-200 px-4 py-8 text-center text-gray-500 dark:border-gray-700 dark:text-gray-400"
	>
		<p>Built with SvelteKit, Rust, and Web Push API</p>
	</footer>
</div>
