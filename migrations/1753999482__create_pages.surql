REMOVE TABLE pages;
DEFINE TABLE pages;

DEFINE FIELD name ON TABLE pages TYPE string;
DEFINE FIELD identifier ON TABLE pages TYPE string;
DEFINE FIELD created_by ON TABLE pages TYPE string;
DEFINE FIELD updated_by ON TABLE pages TYPE string;
DEFINE FIELD created_at ON TABLE pages TYPE datetime;
DEFINE FIELD updated_at ON TABLE pages TYPE datetime;
DEFINE INDEX pages_identifier_index ON TABLE pages COLUMNS identifier UNIQUE;