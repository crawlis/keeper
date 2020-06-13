DROP TABLE nodes;

CREATE TABLE nodes (
    id SERIAL PRIMARY KEY,
    node VARCHAR NOT NULL,
    visited BOOL NOT NULL
);

CREATE TABLE parents (
    id SERIAL PRIMARY KEY,
    parent VARCHAR NOT NULL,
    node VARCHAR NOT NULL
);

ALTER TABLE
    nodes
add
    constraint "unique_node_name" unique ("node");

ALTER TABLE
    parents
add
    constraint "unique_parent_name" unique ("parent", "node");