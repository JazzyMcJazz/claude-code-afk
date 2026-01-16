import { json, error } from '@sveltejs/kit';
import { eq, and } from 'drizzle-orm';
import { db } from '$lib/server/db';
import { pendingDecisions } from '$lib/server/db/schema';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ params, request }) => {
	const authHeader = request.headers.get('Authorization');

	if (!authHeader?.startsWith('Bearer ')) {
		error(401, 'Missing or invalid authorization header');
	}

	const deviceToken = authHeader.slice(7);
	const { decisionId } = params;

	const decision = await db
		.select()
		.from(pendingDecisions)
		.where(and(eq(pendingDecisions.id, decisionId), eq(pendingDecisions.deviceToken, deviceToken)))
		.get();

	if (!decision) {
		error(404, 'Decision not found');
	}

	// Check if expired
	const now = new Date();
	if (decision.expiresAt < now) {
		return json({
			status: 'expired',
			decision: null
		});
	}

	// Check if decided
	if (decision.decision) {
		return json({
			status: 'decided',
			decision: decision.decision
		});
	}

	return json({
		status: 'pending',
		decision: null
	});
};
