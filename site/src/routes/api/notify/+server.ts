import { json, error } from '@sveltejs/kit';
import { eq } from 'drizzle-orm';
import { db } from '$lib/server/db';
import { pairingSessions } from '$lib/server/db/schema';
import { sendPushNotification } from '$lib/server/push';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({ request }) => {
	const authHeader = request.headers.get('Authorization');

	if (!authHeader?.startsWith('Bearer ')) {
		error(401, 'Missing or invalid authorization header');
	}

	const deviceToken = authHeader.slice(7);

	const session = await db
		.select()
		.from(pairingSessions)
		.where(eq(pairingSessions.deviceToken, deviceToken))
		.get();

	if (!session) {
		error(401, 'Invalid device token');
	}

	if (!session.pushSubscription) {
		error(400, 'No push subscription found');
	}

	const { title, message } = await request.json();

	if (!message) {
		error(400, 'Message is required');
	}

	const subscription = JSON.parse(session.pushSubscription);

	try {
		await sendPushNotification(subscription, {
			title: title || 'Claude Code',
			body: message,
			icon: '/icon-192.png',
			badge: '/badge-72.png'
		});
	} catch (err) {
		console.error('Push notification failed:', err);
		error(500, 'Failed to send push notification');
	}

	return json({ success: true });
};
