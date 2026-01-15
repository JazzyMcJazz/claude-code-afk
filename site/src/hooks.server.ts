import { runMigrations } from '$lib/server/db/migrate';
import { env } from '$env/dynamic/private';

// Run migrations on startup
if (env.DATABASE_URL) {
    try {
        runMigrations(env.DATABASE_URL);
    } catch (error) {
        console.error('Failed to run migrations:', error);
        throw error;
    }
}
