CREATE TABLE publisher_sheets
(
    sheets_id   UUID REFERENCES sheets (id),
    publisher_id UUID REFERENCES publishers (id),
    PRIMARY KEY (sheets_id , publisher_id)
);