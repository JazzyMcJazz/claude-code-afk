CREATE TABLE `pending_decisions` (
	`id` text PRIMARY KEY NOT NULL,
	`device_token` text NOT NULL,
	`tool_use_id` text NOT NULL,
	`claude_session_id` text NOT NULL,
	`title` text NOT NULL,
	`message` text NOT NULL,
	`decision` text,
	`created_at` integer NOT NULL,
	`decided_at` integer,
	`expires_at` integer NOT NULL
);
