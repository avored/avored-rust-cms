use crate::api::proto::misc::{SetupRequest, SetupResponse};
use crate::error::Result;
use crate::providers::avored_database_provider::DB;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use std::collections::BTreeMap;
/// misc service
pub struct MiscService {}

impl MiscService {
    pub(crate) async fn setup(
        &self,
        req: SetupRequest,
        password_salt: &str,
        (ds, ses): &DB,
    ) -> Result<SetupResponse> {
        let sql = "

        REMOVE TABLE settings;
        DEFINE TABLE settings;

        DEFINE FIELD identifier ON TABLE settings TYPE string;
        DEFINE FIELD value      ON TABLE settings TYPE string;
        DEFINE FIELD created_at ON TABLE settings TYPE datetime;
        DEFINE FIELD updated_at ON TABLE settings TYPE datetime;
        DEFINE FIELD created_by ON TABLE settings TYPE string;
        DEFINE FIELD updated_by ON TABLE settings TYPE string;

        CREATE settings CONTENT {
            identifier: 'general_site_name',
            value: 'Avored rust cms',
            created_by: $email,
            updated_by: $email,
            created_at: time::now(),
            updated_at: time::now()
        };

        CREATE settings CONTENT {
            identifier: 'auth_cms_token',
            value: '',
            created_by: $email,
            updated_by: $email,
            created_at: time::now(),
            updated_at: time::now()
        };


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

        CREATE admin_users CONTENT {
            full_name: $full_name,
            email: $email,
            password: $password,
            profile_image: $profile_image,
            is_super_admin: $is_super_admin,
            created_by: $email,
            updated_by: $email,
            created_at: time::now(),
            updated_at: time::now()
        };

        REMOVE TABLE password_rest;
        DEFINE TABLE password_rest;

        DEFINE FIELD email ON TABLE password_rest TYPE string;
        DEFINE FIELD token ON TABLE password_rest TYPE string;
        DEFINE FIELD created_at ON TABLE password_rest TYPE datetime;


        REMOVE TABLE roles;
        DEFINE TABLE roles;

        DEFINE FIELD name ON TABLE roles TYPE string;
        DEFINE FIELD identifier ON TABLE roles TYPE string;
        DEFINE FIELD created_by ON TABLE roles TYPE string;
        DEFINE FIELD updated_by ON TABLE roles TYPE string;
        DEFINE FIELD created_at ON TABLE roles TYPE datetime;
        DEFINE FIELD updated_at ON TABLE roles TYPE datetime;
        DEFINE INDEX roles_identifier_index ON TABLE roles COLUMNS identifier UNIQUE;

        CREATE roles CONTENT {
            name: 'Administrator',
            identifier: 'administrator',
            created_by: $email,
            updated_by: $email,
            created_at: time::now(),
            updated_at: time::now()
        };


        REMOVE TABLE admin_user_role;
        DEFINE TABLE admin_user_role;

        DEFINE FIELD created_by ON TABLE admin_user_role TYPE string;
        DEFINE FIELD updated_by ON TABLE admin_user_role TYPE string;
        DEFINE FIELD created_at ON TABLE admin_user_role TYPE datetime;
        DEFINE FIELD updated_at ON TABLE admin_user_role TYPE datetime;


        REMOVE TABLE components;
        DEFINE TABLE components;

        DEFINE FIELD name ON TABLE components TYPE string;
        DEFINE FIELD identifier ON TABLE components TYPE string;
        DEFINE FIELD created_by ON TABLE components TYPE string;
        DEFINE FIELD updated_by ON TABLE components TYPE string;
        DEFINE FIELD created_at ON TABLE components TYPE datetime;
        DEFINE FIELD updated_at ON TABLE components TYPE datetime;
        DEFINE INDEX components_identifier_index ON TABLE components COLUMNS identifier UNIQUE;


        REMOVE TABLE pages;
        DEFINE TABLE pages;

        DEFINE FIELD name ON TABLE pages TYPE string;
        DEFINE FIELD identifier ON TABLE pages TYPE string;
        DEFINE FIELD created_by ON TABLE pages TYPE string;
        DEFINE FIELD updated_by ON TABLE pages TYPE string;
        DEFINE FIELD created_at ON TABLE pages TYPE datetime;
        DEFINE FIELD updated_at ON TABLE pages TYPE datetime;
        DEFINE INDEX pages_identifier_index ON TABLE pages COLUMNS identifier UNIQUE;

        REMOVE TABLE assets;
        DEFINE TABLE assets;


        DEFINE TABLE fields;

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

        CREATE collections CONTENT {
            name: 'Pages',
            identifier: 'pages',
            created_by: $email,
            updated_by: $email,
            created_at: time::now(),
            updated_at: time::now()
        };


        DEFINE TABLE security_audits SCHEMAFULL;

        DEFINE FIELD security_audit_id ON security_audits TYPE string 
            ASSERT $value != NONE AND string::len($value) > 0;

        DEFINE FIELD admin_user_id ON security_audits TYPE option<string>;

        DEFINE FIELD session_id ON security_audits TYPE option<string>;

        DEFINE FIELD ip_address ON security_audits TYPE string 
            ASSERT $value != NONE AND (
                string::is::ipv4($value) OR 
                string::is::ipv6($value) OR 
                $value = 'unknown'
            );

        DEFINE FIELD user_agent ON security_audits TYPE option<string>;

        DEFINE FIELD endpoint ON security_audits TYPE option<string>;

        DEFINE FIELD request_method ON security_audits TYPE option<string>;

        DEFINE FIELD total_authentication_attempts ON security_audits TYPE int DEFAULT 0;
        DEFINE FIELD failed_authentication_attempts ON security_audits TYPE int DEFAULT 0;
        DEFINE FIELD blocked_injection_attempts ON security_audits TYPE int DEFAULT 0;
        DEFINE FIELD rate_limited_requests ON security_audits TYPE int DEFAULT 0;
        DEFINE FIELD suspicious_activities_detected ON security_audits TYPE int DEFAULT 0;
        DEFINE FIELD security_violations ON security_audits TYPE int DEFAULT 0;

        DEFINE FIELD uptime_seconds ON security_audits TYPE int DEFAULT 0;

        DEFINE FIELD security_health_score ON security_audits TYPE float DEFAULT 100.0 
            ASSERT $value >= 0.0 AND $value <= 100.0;

        DEFINE FIELD created_at ON security_audits TYPE datetime DEFAULT time::now();
        DEFINE FIELD updated_at ON security_audits TYPE datetime DEFAULT time::now();

        DEFINE FIELD metadata ON security_audits TYPE option<object>;


        DEFINE TABLE security_alerts SCHEMAFULL;

        DEFINE FIELD alert_id ON security_alerts TYPE string 
            ASSERT $value != NONE AND string::len($value) > 0;
        DEFINE FIELD alert_type ON security_alerts TYPE string 
            ASSERT $value INSIDE [
                'authentication_failure',
                'injection_attempt', 
                'rate_limit_exceeded',
                'suspicious_activity',
                'privilege_escalation',
                'data_breach_attempt',
                'unauthorized_access',
                'malformed_request',
                'brute_force_attack',
                'session_hijacking'
            ];

        DEFINE FIELD severity ON security_alerts TYPE string 
            ASSERT $value INSIDE ['low', 'medium', 'high', 'critical'];

        DEFINE FIELD message ON security_alerts TYPE string 
            ASSERT $value != NONE AND string::len($value) > 0;

        DEFINE FIELD source ON security_alerts TYPE string 
            ASSERT $value != NONE;

        DEFINE FIELD affected_resource ON security_alerts TYPE option<string>;

        DEFINE FIELD metadata ON security_alerts TYPE option<object>;

        DEFINE FIELD resolved ON security_alerts TYPE bool DEFAULT false;
        DEFINE FIELD resolved_at ON security_alerts TYPE option<datetime>;
        DEFINE FIELD resolved_by ON security_alerts TYPE option<string>;

        DEFINE FIELD created_at ON security_alerts TYPE datetime DEFAULT time::now();

        DEFINE INDEX idx_security_audits_ip ON security_audits FIELDS ip_address;
        DEFINE INDEX idx_security_audits_user ON security_audits FIELDS admin_user_id;
        DEFINE INDEX idx_security_audits_created ON security_audits FIELDS created_at;
        DEFINE INDEX idx_security_audits_endpoint ON security_audits FIELDS endpoint;
        DEFINE INDEX idx_security_audits_session ON security_audits FIELDS session_id;

        DEFINE INDEX idx_security_alerts_severity ON security_alerts FIELDS severity;
        DEFINE INDEX idx_security_alerts_type ON security_alerts FIELDS alert_type;
        DEFINE INDEX idx_security_alerts_created ON security_alerts FIELDS created_at;
        DEFINE INDEX idx_security_alerts_resolved ON security_alerts FIELDS resolved;
        DEFINE INDEX idx_security_alerts_source ON security_alerts FIELDS source;

        DEFINE INDEX idx_security_audits_user_created ON security_audits FIELDS admin_user_id, created_at;
        DEFINE INDEX idx_security_alerts_severity_created ON security_alerts FIELDS severity, created_at;

        INSERT INTO security_audits {
            security_audit_id: 'audit_001',
            admin_user_id: 'admin_123',
            session_id: 'session_456',
            ip_address: '192.168.1.100',
            user_agent: 'Mozilla/5.0 (Test Browser)',
            endpoint: '/api/auth/login',
            request_method: 'POST',
            total_authentication_attempts: 1,
            failed_authentication_attempts: 0,
            security_health_score: 100.0,
            metadata: {
                'test_data': true,
                'environment': 'development'
            }
        };

        -- Sample security alert record
        INSERT INTO security_alerts {
            alert_id: 'alert_001',
            alert_type: 'authentication_failure',
            severity: 'medium',
            message: 'Multiple failed login attempts detected',
            source: '192.168.1.100',
            affected_resource: '/api/auth/login',
            resolved: false,
            metadata: {
                'attempt_count': 3,
                'time_window': '5 minutes',
                'test_data': true
            }
        };

    ";

        let password = req.password.as_bytes();
        let salt = SaltString::from_b64(password_salt).unwrap();

        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password, &salt)
            .expect("Error occurred while encrypted password")
            .to_string();

        let vars = BTreeMap::from([
            ("full_name".into(), "Admin".into()),
            ("email".into(), req.email.into()),
            ("password".into(), password_hash.as_str().into()),
            ("profile_image".into(), "".into()),
            ("is_super_admin".into(), true.into()),
        ]);

        let responses = ds.execute(sql, ses, Some(vars)).await.unwrap();

        println!("{responses:?}");
        println!();
        println!("Migrate fresh done!");

        let reply = SetupResponse { status: true };

        Ok(reply)
    }
}


impl MiscService {
    /// create misc service instance
    pub async fn new() -> Result<Self> {
        Ok(Self {})
    }
}
