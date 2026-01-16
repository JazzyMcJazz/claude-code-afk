import { json, error } from '@sveltejs/kit';
import { eq } from 'drizzle-orm';
import { db } from '$lib/server/db';
import { pendingDecisions } from '$lib/server/db/schema';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({ params, request }) => {
	const { decisionId } = params;
	const { decision, toolUseId } = await request.json();

	if (!decision || !['allow', 'dismiss'].includes(decision)) {
		error(400, 'Invalid decision - must be "allow" or "dismiss"');
	}

	if (!toolUseId) {
		error(400, 'toolUseId is required');
	}

	// Find the decision record
	const pendingDecision = await db
		.select()
		.from(pendingDecisions)
		.where(eq(pendingDecisions.id, decisionId))
		.get();

	if (!pendingDecision) {
		error(404, 'Decision not found');
	}

	// Validate toolUseId matches to prevent tampering
	if (pendingDecision.toolUseId !== toolUseId) {
		error(403, 'Tool use ID mismatch');
	}

	// Check if already decided
	if (pendingDecision.decision) {
		return json({
			success: true,
			message: 'Decision already recorded',
			decision: pendingDecision.decision
		});
	}

	// Check if expired
	const now = new Date();
	if (pendingDecision.expiresAt < now) {
		return json({
			success: false,
			message: 'Decision has expired'
		});
	}

	// Update the decision
	await db
		.update(pendingDecisions)
		.set({
			decision,
			decidedAt: now
		})
		.where(eq(pendingDecisions.id, decisionId));

	return json({
		success: true,
		decision
	});
};
