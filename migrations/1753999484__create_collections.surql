REMOVE TABLE collections;
DEFINE TABLE collections;

DEFINE FIELD name ON TABLE collections TYPE string;
DEFINE FIELD identifier ON TABLE collections TYPE string;
DEFINE FIELD created_by ON TABLE collections TYPE string;
DEFINE FIELD updated_by ON TABLE collections TYPE string;
DEFINE FIELD created_at ON TABLE collections TYPE datetime;
DEFINE FIELD updated_at ON TABLE collections TYPE datetime;
DEFINE INDEX collections_identifier_index ON TABLE collections COLUMNS identifier UNIQUE;
DEFINE INDEX collections_identifier_index ON TABLE collections COLUMNS identifier UNIQUE;
