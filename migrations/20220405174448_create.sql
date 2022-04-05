CREATE TABLE IF NOT EXISTS boardgames
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT,          -- auto, /!\ can be reused if previously deleted, use AUTOINCREMENT to prevent that
    created      TEXT    NOT NULL DEFAULT CURRENT_TIMESTAMP, -- YYYY-MM-DD HH:MM:SS
    gameid       INTEGER NOT NULL,                           -- BGG id
    title        TEXT    NOT NULL UNIQUE,
    published    INTEGER,
    playing_time INTEGER,
    min_players  INTEGER,
    max_players  INTEGER
)