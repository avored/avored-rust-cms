REMOVE TABLE components;
DEFINE TABLE components;

DEFINE FIELD name ON TABLE components TYPE string;
DEFINE FIELD identifier ON TABLE components TYPE string;
DEFINE FIELD created_by ON TABLE components TYPE string;
DEFINE FIELD updated_by ON TABLE components TYPE string;
DEFINE FIELD created_at ON TABLE components TYPE datetime;
DEFINE FIELD updated_at ON TABLE components TYPE datetime;
DEFINE INDEX components_identifier_index ON TABLE components COLUMNS identifier UNIQUE;

