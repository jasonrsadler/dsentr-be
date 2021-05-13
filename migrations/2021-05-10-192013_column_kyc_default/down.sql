-- This file should undo anything in `up.sql`ALTER TABLE users
ALTER TABLE users
ALTER COLUMN kyc_level DROP NOT NULL
