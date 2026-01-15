<script lang="ts">
	import { urlBase64ToUint8Array } from '$lib/utils';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	let status = $state<'idle' | 'requesting' | 'subscribing' | 'completing' | 'success' | 'error'>(
		'idle'
	);
	let errorMessage = $state('');
	let notificationsSupported = $derived('serviceWorker' in navigator && 'PushManager' in window);

	async function enableNotifications() {
		try {
			status = 'requesting';

			const permission = await Notification.requestPermission();
			if (permission !== 'granted') {
				throw new Error('Notification permission denied');
			}

			status = 'subscribing';

			// Register service worker and wait for it to be ready
			const registration = await navigator.serviceWorker.register('/sw.js');
			const activeRegistration = await navigator.serviceWorker.ready;

			// Check if there's an existing subscription
			let subscription = await activeRegistration.pushManager.getSubscription();

			if (!subscription) {
				// Create new subscription
				const applicationServerKey = urlBase64ToUint8Array(data.vapidPublicKey);

				subscription = await activeRegistration.pushManager.subscribe({
					userVisibleOnly: true,
					applicationServerKey
				});
			}

			status = 'completing';

			const response = await fetch(`/api/pairing/${data.pairingToken}/complete`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ subscription })
			});

			if (!response.ok) {
				const err = await response.json();
				throw new Error(err.message || 'Failed to complete pairing');
			}

			status = 'success';
		} catch (err) {
			status = 'error';
			if (err instanceof Error) {
				// Provide user-friendly error messages
				if (err.message.includes('denied')) {
					errorMessage = 'Notification permission was denied. Please enable notifications in your browser settings and try again.';
				} else if (err.message.includes('push service')) {
					errorMessage = 'Unable to set up push notifications. This may be due to browser restrictions or network issues. Try using Chrome or Firefox.';
				} else if (err.message === 'Failed to complete pairing') {
					errorMessage = 'Failed to complete pairing. Please try scanning the QR code again.';
				} else {
					errorMessage = 'Something went wrong. Please try again or use a different browser.';
				}
			} else {
				errorMessage = 'An unexpected error occurred. Please try again.';
			}
		}
	}
</script>

<svelte:head>
	<title>Claude AFK - Enable Notifications</title>
</svelte:head>

<div class="flex min-h-screen items-center justify-center bg-gray-50 dark:bg-gray-900 p-4">
	<div class="w-full max-w-md rounded-lg bg-white dark:bg-gray-800 p-8 text-center shadow-lg">
		<h1 class="mb-4 text-2xl font-bold text-gray-900 dark:text-gray-100">Claude AFK</h1>

		{#if !notificationsSupported}
			<div class="text-red-600 dark:text-red-400">
				<p class="font-semibold">Notifications Not Supported</p>
				<p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
					Your browser doesn't support push notifications. Please use a modern browser like Chrome,
					Firefox, or Edge.
				</p>
			</div>
		{:else if status === 'idle'}
			<p class="mb-6 text-gray-600 dark:text-gray-300">
				Enable notifications to receive alerts when Claude Code needs your attention.
			</p>
			<button
				onclick={enableNotifications}
				class="w-full rounded-lg bg-blue-600 px-6 py-3 font-semibold text-white transition-colors hover:bg-blue-700"
			>
				Enable Notifications
			</button>
		{:else if status === 'requesting'}
			<p class="text-gray-600 dark:text-gray-300">Requesting permission...</p>
		{:else if status === 'subscribing'}
			<p class="text-gray-600 dark:text-gray-300">Setting up notifications...</p>
		{:else if status === 'completing'}
			<p class="text-gray-600 dark:text-gray-300">Completing pairing...</p>
		{:else if status === 'success'}
			<div class="text-green-600 dark:text-green-400">
				<svg class="mx-auto mb-4 h-16 w-16" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"
					></path>
				</svg>
				<p class="text-xl font-semibold">Success!</p>
				<p class="mt-2 text-gray-600 dark:text-gray-300">Notifications are now enabled. You can close this page.</p>
			</div>
		{:else if status === 'error'}
			<div class="text-red-600 dark:text-red-400">
				<p class="font-semibold">Error</p>
				<p class="mt-2 text-sm text-gray-600 dark:text-gray-400">{errorMessage}</p>
				<button
					onclick={() => {
						status = 'idle';
						errorMessage = '';
					}}
					class="mt-4 rounded bg-gray-200 dark:bg-gray-700 px-4 py-2 font-semibold text-gray-800 dark:text-gray-200 transition-colors hover:bg-gray-300 dark:hover:bg-gray-600"
				>
					Try Again
				</button>
			</div>
		{/if}
	</div>
</div>
