<script lang="ts">
</script>

<div class="min-h-screen bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 transition-colors">
    <!-- Hero Section -->
    <header class="container mx-auto px-4 py-16 text-center">
        <div class="max-w-3xl mx-auto">
            <h1 class="text-5xl font-bold mb-4 bg-linear-to-r from-blue-600 to-purple-600 dark:from-blue-400 dark:to-purple-400 bg-clip-text text-transparent">
                Claude Code AFK
            </h1>
            <p class="text-xl text-gray-600 dark:text-gray-300 mb-8">
                Get push notifications to your phone when Claude Code needs your attention
            </p>
            <div class="flex gap-4 justify-center flex-wrap">
                <a href="#quick-start" class="px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-semibold transition-colors">
                    Get Started
                </a>
                <a href="https://github.com/JazzyMcJazz/claude-code-afk" target="_blank" rel="noopener noreferrer" class="px-6 py-3 border-2 border-gray-300 dark:border-gray-600 hover:border-blue-600 dark:hover:border-blue-400 rounded-lg font-semibold transition-colors">
                    View on GitHub
                </a>
            </div>
        </div>
    </header>

    <!-- Features Section -->
    <section class="container mx-auto px-4 py-16">
        <div class="max-w-5xl mx-auto grid md:grid-cols-3 gap-8">
            <div class="p-6 bg-gray-50 dark:bg-gray-800 rounded-xl">
                <div class="text-3xl mb-4">📱</div>
                <h3 class="text-xl font-semibold mb-2">Push Notifications</h3>
                <p class="text-gray-600 dark:text-gray-400">
                    Receive instant notifications on your phone when Claude Code requires input
                </p>
            </div>
            <div class="p-6 bg-gray-50 dark:bg-gray-800 rounded-xl">
                <div class="text-3xl mb-4">🔒</div>
                <h3 class="text-xl font-semibold mb-2">Secure Pairing</h3>
                <p class="text-gray-600 dark:text-gray-400">
                    Simple QR code pairing with secure device tokens and Web Push API
                </p>
            </div>
            <div class="p-6 bg-gray-50 dark:bg-gray-800 rounded-xl">
                <div class="text-3xl mb-4">⚡</div>
                <h3 class="text-xl font-semibold mb-2">Lightweight</h3>
                <p class="text-gray-600 dark:text-gray-400">
                    Minimal Rust CLI with no async runtime overhead
                </p>
            </div>
        </div>
    </section>

    <!-- How It Works -->
    <section class="container mx-auto px-4 py-16 bg-gray-50 dark:bg-gray-800">
        <div class="max-w-4xl mx-auto">
            <h2 class="text-3xl font-bold text-center mb-12">How It Works</h2>
            <div class="space-y-8">
                <div class="flex gap-4">
                    <div class="shrink-0 w-8 h-8 bg-blue-600 text-white rounded-full flex items-center justify-center font-bold">1</div>
                    <div>
                        <h4 class="font-semibold mb-1">Pair Your Device</h4>
                        <p class="text-gray-600 dark:text-gray-400">Run the CLI setup command and scan the QR code with your phone</p>
                    </div>
                </div>
                <div class="flex gap-4">
                    <div class="shrink-0 w-8 h-8 bg-blue-600 text-white rounded-full flex items-center justify-center font-bold">2</div>
                    <div>
                        <h4 class="font-semibold mb-1">Grant Permissions</h4>
                        <p class="text-gray-600 dark:text-gray-400">Enable push notifications when prompted in your browser</p>
                    </div>
                </div>
                <div class="flex gap-4">
                    <div class="shrink-0 w-8 h-8 bg-blue-600 text-white rounded-full flex items-center justify-center font-bold">3</div>
                    <div>
                        <h4 class="font-semibold mb-1">Configure Claude Code</h4>
                        <p class="text-gray-600 dark:text-gray-400">Add the notification hook to your Claude Code configuration</p>
                    </div>
                </div>
                <div class="flex gap-4">
                    <div class="shrink-0 w-8 h-8 bg-blue-600 text-white rounded-full flex items-center justify-center font-bold">4</div>
                    <div>
                        <h4 class="font-semibold mb-1">Stay Notified</h4>
                        <p class="text-gray-600 dark:text-gray-400">Get push notifications whenever Claude needs your attention</p>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <!-- Architecture -->
    <section class="container mx-auto px-4 py-16">
        <div class="max-w-4xl mx-auto">
            <h2 class="text-3xl font-bold text-center mb-8">Architecture</h2>
            <div class="bg-gray-50 dark:bg-gray-800 p-8 rounded-xl overflow-x-auto">
                <div class="flex items-center justify-center gap-4 min-w-max">
                    <div class="text-center">
                        <div class="px-4 py-3 bg-white dark:bg-gray-700 rounded-lg shadow">Claude Code</div>
                    </div>
                    <div class="text-2xl">→</div>
                    <div class="text-center">
                        <div class="px-4 py-3 bg-white dark:bg-gray-700 rounded-lg shadow">CLI Tool</div>
                        <div class="text-xs text-gray-500 mt-1">Rust</div>
                    </div>
                    <div class="text-2xl">→</div>
                    <div class="text-center">
                        <div class="px-4 py-3 bg-white dark:bg-gray-700 rounded-lg shadow">Backend</div>
                        <div class="text-xs text-gray-500 mt-1">SvelteKit</div>
                    </div>
                    <div class="text-2xl">→</div>
                    <div class="text-center">
                        <div class="px-4 py-3 bg-white dark:bg-gray-700 rounded-lg shadow">Your Phone</div>
                        <div class="text-xs text-gray-500 mt-1">Web Push</div>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <!-- Quick Start -->
    <section id="quick-start" class="container mx-auto px-4 py-16">
        <div class="max-w-3xl mx-auto">
            <h2 class="text-3xl font-bold text-center mb-8">Quick Start</h2>
            <div class="bg-gray-900 text-gray-100 p-6 rounded-xl">
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
                        <div class="text-green-400">echo '&#123;"message":"Hello!"&#125;' | claude-afk notify</div>
                    </div>
                </div>
            </div>
            <div class="mt-4 text-center">
                <a href="https://github.com/JazzyMcJazz/claude-code-afk" target="_blank" rel="noopener noreferrer" class="text-blue-600 dark:text-blue-400 hover:underline">
                    View full documentation →
                </a>
            </div>
        </div>
    </section>

    <!-- Footer -->
    <footer class="container mx-auto px-4 py-8 text-center text-gray-500 dark:text-gray-400 border-t border-gray-200 dark:border-gray-700">
        <p>Built with SvelteKit, Rust, and Web Push API</p>
    </footer>
</div>