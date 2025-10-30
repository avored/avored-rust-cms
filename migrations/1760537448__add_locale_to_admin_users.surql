-- Add locale field to admin_users table
-- This allows each admin user to set their preferred language/locale
DEFINE FIELD locale ON TABLE admin_users TYPE string DEFAULT "en";
