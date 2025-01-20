CREATE TABLE IF NOT EXISTS games (
	uuid VARCHAR(255) PRIMARY KEY,
	white integer REFERENCES accounts,
	black integer REFERENCES accounts
)
