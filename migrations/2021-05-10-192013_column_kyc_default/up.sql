-- Your SQL goes here
UPDATE users set kyc_level = 0;
ALTER TABLE users
ALTER COLUMN kyc_level SET DEFAULT 0,
ALTER COLUMN kyc_level SET NOT NULL
