REMOVE TABLE roles;
DEFINE TABLE roles;

DEFINE FIELD name ON TABLE roles TYPE string;
DEFINE FIELD identifier ON TABLE roles TYPE string;
DEFINE FIELD created_by ON TABLE roles TYPE string;
DEFINE FIELD updated_by ON TABLE roles TYPE string;
DEFINE FIELD created_at ON TABLE roles TYPE datetime;
DEFINE FIELD updated_at ON TABLE roles TYPE datetime;
DEFINE INDEX roles_identifier_index ON TABLE roles COLUMNS identifier UNIQUE;