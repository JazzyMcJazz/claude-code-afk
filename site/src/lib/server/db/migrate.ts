import { drizzle } from 'drizzle-orm/better-sqlite3';
import { migrate } from 'drizzle-orm/better-sqlite3/migrator';
import Database from 'better-sqlite3';

/**
 * Run database migrations
 * This uses drizzle-orm's migrate() function which only needs the generated SQL files,
 * not drizzle-kit. Safe to run in production.
 */
export function runMigrations(databaseUrl: string): void {
    const client = new Database(databaseUrl);
    const db = drizzle(client);

    console.log('Running database migrations...');
    migrate(db, { migrationsFolder: './drizzle' });
    console.log('Migrations completed successfully');

    client.close();
}
