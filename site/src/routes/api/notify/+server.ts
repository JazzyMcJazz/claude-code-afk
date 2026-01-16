import { json, error } from '@sveltejs/kit';
import { eq } from 'drizzle-orm';
import { nanoid } from 'nanoid';
import { db } from '$lib/server/db';
import { pairingSessions, pendingDecisions } from '$lib/server/db/schema';
import { sendPushNotification } from '$lib/server/push';
import type { RequestHandler } from './$types';

// Decision expiry time in milliseconds (5 minutes)
const DECISION_EXPIRY_MS = 5 * 60 * 1000;

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

	const { title, message, tool_use_id, session_id } = await request.json();

	console.log('Sending notification:', { title, message, tool_use_id, session_id });

	if (!message) {
		error(400, 'Message is required');
	}

	if (!tool_use_id) {
		error(400, 'tool_use_id is required');
	}

	if (!session_id) {
		error(400, 'session_id is required');
	}

	const subscription = JSON.parse(session.pushSubscription);

	// Create pending decision record
	const decisionId = nanoid(21);
	const now = new Date();
	const expiresAt = new Date(now.getTime() + DECISION_EXPIRY_MS);

	await db.insert(pendingDecisions).values({
		id: decisionId,
		deviceToken,
		toolUseId: tool_use_id,
		claudeSessionId: session_id,
		title: title || 'Claude Code',
		message,
		createdAt: now,
		expiresAt
	});

	try {
		await sendPushNotification(subscription, {
			title: title || 'Claude Code',
			body: message,
			icon: '/icon-192.png',
			badge: '/badge-72.png',
			tag: tool_use_id,
			renotify: true,
			requireInteraction: true,
			actions: [{ action: 'allow', title: 'Allow' }],
			data: {
				decisionId,
				toolUseId: tool_use_id,
				type: 'decision'
			}
		});
	} catch (err) {
		console.error('Push notification failed:', err);
		error(500, 'Failed to send push notification');
	}

	return json({ success: true, decisionId });
};
