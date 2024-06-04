CREATE TYPE Ownership AS ENUM (
    'Publisher', 'Subscriber'
);
CREATE TABLE updates
(
    id       UUID PRIMARY KEY NOT NULL,
    owner_id UUID             NOT NULL,
    update_value VARCHAR(1000) NOT NULL,
    ownership Ownership,
    createDate DATE
)