-- Add migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT true,
    last_login TIMESTAMP WITH TIME ZONE
);

CREATE UNIQUE INDEX idx_users_username_lower 
ON users (LOWER(username));

CREATE UNIQUE INDEX idx_users_email_lower 
ON users (LOWER(email));

-- Add a comment to the table
COMMENT ON TABLE users IS 'Stores user account information';
