CREATE TABLE sheet_elems
(
    id     UUID PRIMARY KEY,
    sheet_column_identifier VARCHAR(100)  NOT NULL,
    sheet_row    INTEGER          NOT NULL,
    sheet_value  VARCHAR(100) NOT NULL,
    sheet_id UUID NOT NULL REFERENCES sheets(id),
    UNIQUE (sheet_column_identifier, sheet_row, sheet_id)
);
