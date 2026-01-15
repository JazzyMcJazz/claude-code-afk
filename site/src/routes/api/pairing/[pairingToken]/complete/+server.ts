import { json, error } from '@sveltejs/kit';
import { eq } from 'drizzle-orm';
import { nanoid } from 'nanoid';
import { db } from '$lib/server/db';
import { pairingSessions } from '$lib/server/db/schema';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({ params, request }) => {
	const session = await db
		.select()
		.from(pairingSessions)
		.where(eq(pairingSessions.pairingToken, params.pairingToken))
		.get();

	if (!session) {
		error(404, 'Pairing session not found');
	}

	if (session.completedAt) {
		error(400, 'Pairing session already completed');
	}

	const { subscription } = await request.json();

	if (!subscription || !subscription.endpoint) {
		error(400, 'Invalid push subscription');
	}

	const deviceToken = nanoid(32);

	await db
		.update(pairingSessions)
		.set({
			pushSubscription: JSON.stringify(subscription),
			deviceToken,
			completedAt: new Date()
		})
		.where(eq(pairingSessions.pairingToken, params.pairingToken));

	return json({ success: true });
};
