CREATE TABLE publisher_sheets
(
    sheets_id   BIGINT REFERENCES sheet_elems (id),
    publishers_id BIGINT REFERENCES publishers (id),
    PRIMARY KEY (sheets_id , publishers_id)
);