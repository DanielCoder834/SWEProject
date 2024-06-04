CREATE TABLE publisher_sheets
(
    sheets_id   INTEGER REFERENCES sheets (id),
    publisher_id INTEGER REFERENCES publishers (id),
    PRIMARY KEY (sheets_id , publisher_id)
);