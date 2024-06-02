CREATE TABLE sheets (
    id  INTEGER NOT NULL,
    sheet_elem_id INTEGER NOT NULL,
    title   VARCHAR(100) NOT NULL,
    primary key (id, sheet_elem_id)
)