CREATE TABLE `pairing_sessions` (
	`id` text PRIMARY KEY NOT NULL,
	`pairing_token` text NOT NULL,
	`device_token` text,
	`push_subscription` text,
	`created_at` integer NOT NULL,
	`completed_at` integer
);
--> statement-breakpoint
CREATE UNIQUE INDEX `pairing_sessions_pairing_token_unique` ON `pairing_sessions` (`pairing_token`);--> statement-breakpoint
CREATE UNIQUE INDEX `pairing_sessions_device_token_unique` ON `pairing_sessions` (`device_token`);