/// <reference lib="webworker" />

// @ts-check

/** @type {ServiceWorkerGlobalScope} */
// @ts-expect-error - ServiceWorkerGlobalScope is not defined in the global scope
const sw = self;

sw.addEventListener('push', (event) => {
    /** @type {{ title?: string; body?: string; icon?: string; badge?: string; data?: object } | null} */
    const data = event.data?.json() ?? {};

    const title = data?.title || 'Claude Code';
    const options = {
        body: data?.body || '',
        icon: data?.icon || '/icon-192.png',
        badge: data?.badge || '/badge-72.png',
        data: data?.data || {},
        requireInteraction: true,
        tag: 'claude-code-notification'
    };

    event.waitUntil(sw.registration.showNotification(title, options));
});

sw.addEventListener('notificationclick', (event) => {
    event.notification.close();

    event.waitUntil(
        sw.clients.matchAll({ type: 'window' }).then((clientList) => {
            for (const client of clientList) {
                if ('focus' in client) {
                    return client.focus();
                }
            }
            if (sw.clients.openWindow) {
                return sw.clients.openWindow('/');
            }
        })
    );
});

sw.addEventListener('install', () => {
    sw.skipWaiting();
});

sw.addEventListener('activate', (event) => {
    event.waitUntil(sw.clients.claim());
});
