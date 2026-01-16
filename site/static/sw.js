/// <reference lib="webworker" />

// @ts-check

/** @type {ServiceWorkerGlobalScope} */
// @ts-expect-error - ServiceWorkerGlobalScope is not defined in the global scope
const sw = self;

/**
 * Submit a decision to the backend
 * @param {string} decisionId
 * @param {string} toolUseId
 * @param {'allow' | 'dismiss'} decision
 */
async function submitDecision(decisionId, toolUseId, decision) {
    try {
        const response = await fetch(`/api/decision/${decisionId}/submit`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ decision, toolUseId })
        });
        if (!response.ok) {
            console.error('Failed to submit decision:', response.status);
        }
    } catch (err) {
        console.error('Error submitting decision:', err);
    }
}

sw.addEventListener('push', (event) => {
    /** @type {{ title?: string; body?: string; icon?: string; badge?: string; data?: object; tag?: string; renotify?: boolean; requireInteraction?: boolean; actions?: Array<{action: string; title: string}> } | null} */
    const data = event.data?.json() ?? {};

    const title = data?.title || 'Claude Code';
    const options = {
        body: data?.body || '',
        icon: data?.icon || '/icon-192.png',
        badge: data?.badge || '/badge-72.png',
        data: data?.data || {},
        requireInteraction: data?.requireInteraction ?? true,
        tag: data?.tag || 'claude-code-notification',
        renotify: data?.renotify ?? false,
        actions: data?.actions || []
    };

    event.waitUntil(sw.registration.showNotification(title, options));
});

sw.addEventListener('notificationclick', (event) => {
    const notificationData = event.notification.data || {};
    const action = event.action;

    // If this is a decision notification and user clicked "Allow"
    if (action === 'allow' && notificationData.type === 'decision') {
        event.notification.close();
        event.waitUntil(
            submitDecision(notificationData.decisionId, notificationData.toolUseId, 'allow')
        );
        return;
    }

    // Default click behavior (clicked notification body, not an action button)
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

sw.addEventListener('notificationclose', (event) => {
    const notificationData = event.notification.data || {};

    // If this is a decision notification and it was dismissed (closed without clicking Allow)
    if (notificationData.type === 'decision' && notificationData.decisionId) {
        event.waitUntil(
            submitDecision(notificationData.decisionId, notificationData.toolUseId, 'dismiss')
        );
    }
});

sw.addEventListener('install', () => {
    sw.skipWaiting();
});

sw.addEventListener('activate', (event) => {
    event.waitUntil(sw.clients.claim());
});
