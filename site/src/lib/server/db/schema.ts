import { sqliteTable, text, integer } from 'drizzle-orm/sqlite-core';

export const pairingSessions = sqliteTable('pairing_sessions', {
	id: text('id').primaryKey(),
	pairingToken: text('pairing_token').notNull().unique(),
	deviceToken: text('device_token').unique(),
	pushSubscription: text('push_subscription'),
	createdAt: integer('created_at', { mode: 'timestamp' }).notNull(),
	completedAt: integer('completed_at', { mode: 'timestamp' })
});
