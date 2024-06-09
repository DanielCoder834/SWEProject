CREATE TYPE ownership AS ENUM (
    'publisher', 'subscriber'
);
CREATE TABLE updates
(
    id       SERIAL PRIMARY KEY NOT NULL,
    owner_id UUID             NOT NULL,
    update_value VARCHAR(1000) NOT NULL,
    ownership ownership  NOT NULL
)