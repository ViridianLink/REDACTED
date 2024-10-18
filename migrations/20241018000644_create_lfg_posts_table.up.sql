-- Add up migration script here
CREATE TABLE lfg_posts (
    id BIGINT PRIMARY KEY NOT NULL,
    owner_id BIGINT NOT NULL,
    activity TEXT NOT NULL,
    start_time TIMESTAMPTZ NOT NULL,
    description TEXT NOT NULL,
    fireteam_size SMALLINT NOT NULL,
    fireteam_ids BIGINT[] NOT NULL
)
