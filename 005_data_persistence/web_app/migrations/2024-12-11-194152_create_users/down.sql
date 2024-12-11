-- This file should undo anything in `up.sql`

-- Drop the user_id column in the to_do table
ALTER TABLE to_do DROP COLUMN user_id;

-- Drop the users table
DROP TABLE users;