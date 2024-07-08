-- Add up migration script here
CREATE TABLE family (
    id BIGINT PRIMARY KEY,
    username TEXT NOT NULL,
    partner_ids BIGINT[] NOT NULL DEFAULT '{}',
    parent_ids BIGINT[] NOT NULL DEFAULT '{}',
    children_ids BIGINT[] NOT NULL DEFAULT '{}',
    blocked_ids BIGINT[] NOT NULL DEFAULT '{}'
);
