CREATE TABLE accounts (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    mas_instance TEXT,
    mas_client_id TEXT,
    mas_client_secret TEXT,
    mas_access_token TEXT
);