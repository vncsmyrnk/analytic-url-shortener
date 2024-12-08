CREATE TABLE hits (
    id SERIAL PRIMARY KEY,
    endpoint_id INTEGER NOT NULL,
    hit_time TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    ip inet NOT NULL,
    country_name VARCHAR(100),
    user_agent VARCHAR(100),
    FOREIGN KEY (endpoint_id) REFERENCES endpoints(id) ON DELETE CASCADE
);
