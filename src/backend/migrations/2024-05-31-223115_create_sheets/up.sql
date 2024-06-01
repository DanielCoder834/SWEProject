CREATE TABLE sheet_elems
(
    id     BIGINT PRIMARY KEY,
    title   VARCHAR(100) NOT NULL,
    sheet_column_identifier VARCHAR(100)  NOT NULL,
    sheet_row    BIGINT          NOT NULL,
    sheet_value  VARCHAR(100) NOT NULL,
    sheet_id VARCHAR(100) NOT NULL
);
