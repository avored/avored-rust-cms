 REMOVE TABLE password_rest;
DEFINE TABLE password_rest;

DEFINE FIELD email ON TABLE password_rest TYPE string;
DEFINE FIELD token ON TABLE password_rest TYPE string;
DEFINE FIELD created_at ON TABLE password_rest TYPE datetime;