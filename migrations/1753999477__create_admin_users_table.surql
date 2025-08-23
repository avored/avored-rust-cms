REMOVE TABLE admin_users;
DEFINE TABLE admin_users;

DEFINE FIELD full_name ON TABLE admin_users TYPE string;
DEFINE FIELD email ON TABLE admin_users TYPE string;
DEFINE FIELD password ON TABLE admin_users TYPE string;
DEFINE FIELD profile_image ON TABLE admin_users TYPE string;
DEFINE FIELD is_super_admin ON TABLE admin_users TYPE bool;
DEFINE FIELD created_by ON TABLE admin_users TYPE string;
DEFINE FIELD updated_by ON TABLE admin_users TYPE string;
DEFINE FIELD created_at ON TABLE admin_users TYPE datetime;
DEFINE FIELD updated_at ON TABLE admin_users TYPE datetime;
DEFINE INDEX admin_users_email_index ON TABLE admin_users COLUMNS email UNIQUE;
