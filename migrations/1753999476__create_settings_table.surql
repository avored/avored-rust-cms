REMOVE TABLE settings;
DEFINE TABLE settings;

DEFINE FIELD identifier ON TABLE settings TYPE string;
DEFINE FIELD value      ON TABLE settings TYPE string;
DEFINE FIELD created_at ON TABLE settings TYPE datetime;
DEFINE FIELD updated_at ON TABLE settings TYPE datetime;
DEFINE FIELD created_by ON TABLE settings TYPE string;
DEFINE FIELD updated_by ON TABLE settings TYPE string;