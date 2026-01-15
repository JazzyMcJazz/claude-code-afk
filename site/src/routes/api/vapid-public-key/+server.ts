import { json, error } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async () => {
	if (!env.VAPID_PUBLIC_KEY) {
		error(500, 'VAPID public key not configured');
	}

	return json({
		publicKey: env.VAPID_PUBLIC_KEY
	});
};
