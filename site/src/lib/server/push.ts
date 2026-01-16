import webpush from 'web-push';
import { env } from '$env/dynamic/private';

export interface NotificationPayload {
	title: string;
	body: string;
	icon?: string;
	badge?: string;
	data?: Record<string, unknown>;
	tag?: string;
	renotify?: boolean;
	requireInteraction?: boolean;
	actions?: Array<{ action: string; title: string; icon?: string }>;
}

// Lazy initialization - only runs at runtime when actually sending notifications
let vapidInitialized = false;
function ensureVapidInitialized() {
	if (vapidInitialized) return;

	if (!env.VAPID_PUBLIC_KEY || !env.VAPID_PRIVATE_KEY || !env.VAPID_SUBJECT) {
		throw new Error('VAPID keys not configured - cannot send push notifications');
	}

	webpush.setVapidDetails(env.VAPID_SUBJECT, env.VAPID_PUBLIC_KEY, env.VAPID_PRIVATE_KEY);
	vapidInitialized = true;
}

export async function sendPushNotification(
	subscription: webpush.PushSubscription,
	payload: NotificationPayload
): Promise<void> {
	ensureVapidInitialized();
	await webpush.sendNotification(subscription, JSON.stringify(payload));
}

export { webpush };
