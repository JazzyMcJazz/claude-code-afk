import { json } from '@sveltejs/kit';
import { nanoid } from 'nanoid';
import { db } from '$lib/server/db';
import { pairingSessions } from '$lib/server/db/schema';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = async () => {
	const id = nanoid(21);
	const pairingToken = nanoid(32);

	await db.insert(pairingSessions).values({
		id,
		pairingToken,
		createdAt: new Date()
	});

	return json({
		pairingId: id,
		pairingToken
	});
};
