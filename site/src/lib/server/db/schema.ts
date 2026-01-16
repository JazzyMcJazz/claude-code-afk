import { sqliteTable, text, integer } from 'drizzle-orm/sqlite-core';

export const pairingSessions = sqliteTable('pairing_sessions', {
	id: text('id').primaryKey(),
	pairingToken: text('pairing_token').notNull().unique(),
	deviceToken: text('device_token').unique(),
	pushSubscription: text('push_subscription'),
	createdAt: integer('created_at', { mode: 'timestamp' }).notNull(),
	completedAt: integer('completed_at', { mode: 'timestamp' })
});

export const pendingDecisions = sqliteTable('pending_decisions', {
	id: text('id').primaryKey(), // nanoid(21)
	deviceToken: text('device_token').notNull(),
	toolUseId: text('tool_use_id').notNull(),
	claudeSessionId: text('claude_session_id').notNull(),
	title: text('title').notNull(),
	message: text('message').notNull(),
	decision: text('decision'), // null=pending, 'allow', 'dismiss'
	createdAt: integer('created_at', { mode: 'timestamp' }).notNull(),
	decidedAt: integer('decided_at', { mode: 'timestamp' }),
	expiresAt: integer('expires_at', { mode: 'timestamp' }).notNull()
});
