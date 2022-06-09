CREATE TABLE
    IF NOT EXISTS boardgames (
        -- auto, /!\ can be reused if previously deleted, use AUTOINCREMENT to prevent that
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        -- BGG id
        gameid INTEGER NOT NULL UNIQUE INDEX,
        title TEXT NOT NULL,
        published INTEGER,
        playing_time INTEGER,
        min_players INTEGER,
        max_players INTEGER
    );