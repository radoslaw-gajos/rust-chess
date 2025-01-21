CREATE TABLE IF NOT EXISTS games (
	uuid uuid PRIMARY KEY,
	white integer REFERENCES accounts,
	black integer REFERENCES accounts
)
