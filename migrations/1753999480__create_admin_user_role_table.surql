REMOVE TABLE admin_user_role;
DEFINE TABLE admin_user_role;

DEFINE FIELD created_by ON TABLE admin_user_role TYPE string;
DEFINE FIELD updated_by ON TABLE admin_user_role TYPE string;
DEFINE FIELD created_at ON TABLE admin_user_role TYPE datetime;
DEFINE FIELD updated_at ON TABLE admin_user_role TYPE datetime;