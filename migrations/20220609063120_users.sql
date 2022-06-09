CREATE TABLE
    IF NOT EXISTS users (
        -- auto, /!\ can be reused if previously deleted, use AUTOINCREMENT to prevent that
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        -- BGG username
        username TEXT NOT NULL,
    )
CREATE TABLE
    boargames_users (
        boargame_id INTEGER NOT NULL REFERENCES boardgames (gameid)
        ON DELETE CASCADE,
        user_id TEXT NOT NULL REFERENCES users (username)
        ON DELETE CASCADE,
        PRIMARY KEY (user_id, boargame_id)
    )