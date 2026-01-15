import { error } from '@sveltejs/kit';
import { eq } from 'drizzle-orm';
import { db } from '$lib/server/db';
import { pairingSessions } from '$lib/server/db/schema';
import { env } from '$env/dynamic/private';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
	const session = db
		.select()
		.from(pairingSessions)
		.where(eq(pairingSessions.pairingToken, params.pairingToken))
		.get();

	if (!session) {
		console.debug('Pairing session not found', params.pairingToken);
		error(404, 'Pairing session not found');
	}

	if (session.completedAt) {
		console.debug('Pairing session already completed', params.pairingToken);
		error(400, 'Pairing session already completed');
	}

	if (!env.VAPID_PUBLIC_KEY) {
		console.debug('VAPID public key not found', env.VAPID_PUBLIC_KEY);
		error(500, 'Server not configured for push notifications');
	}

	return {
		pairingToken: params.pairingToken,
		vapidPublicKey: env.VAPID_PUBLIC_KEY
	};
};
