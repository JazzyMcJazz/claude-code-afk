import { json, error } from '@sveltejs/kit';
import { eq } from 'drizzle-orm';
import { db } from '$lib/server/db';
import { pairingSessions } from '$lib/server/db/schema';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ params }) => {
	const session = await db
		.select()
		.from(pairingSessions)
		.where(eq(pairingSessions.id, params.pairingId))
		.get();

	if (!session) {
		error(404, 'Pairing session not found');
	}

	const complete = session.completedAt !== null;

	return json({
		complete,
		deviceToken: complete ? session.deviceToken : null
	});
};
