# LDAP/Active Directory Authentication

This document describes how to configure and use LDAP/Active Directory authentication in AvoRed Rust CMS.

## Overview

The LDAP authentication feature allows users to authenticate using their existing LDAP or Active Directory credentials. This is particularly useful for enterprise environments where users already have directory service accounts.

## Features

- **Multi-provider Authentication**: Supports both local and LDAP authentication simultaneously
- **Automatic User Synchronization**: LDAP users are automatically created in the local database
- **Fallback Authentication**: If LDAP authentication fails, the system falls back to local authentication
- **Enterprise Ready**: Supports both OpenLDAP and Microsoft Active Directory
- **Secure Connections**: Supports TLS/SSL for secure LDAP communications
- **Configurable Timeouts**: Customizable connection and search timeouts

## Configuration

### Environment Variables

Copy the `.env.ldap.example` file to `.env` and configure the following variables:

```bash
# Enable LDAP authentication
AVORED_LDAP_ENABLED=true

# LDAP server configuration
AVORED_LDAP_SERVER=ldap://your-ldap-server.com
AVORED_LDAP_PORT=389
AVORED_LDAP_USE_TLS=true

# Directory structure
AVORED_LDAP_BASE_DN=dc=example,dc=com
AVORED_LDAP_BIND_DN=cn=admin,dc=example,dc=com
AVORED_LDAP_BIND_PASSWORD=your-password

# User search configuration
AVORED_LDAP_USER_SEARCH_BASE=ou=users,dc=example,dc=com
AVORED_LDAP_USER_SEARCH_FILTER=(uid={username})

# Attribute mapping
AVORED_LDAP_USER_ATTRIBUTE_EMAIL=mail
AVORED_LDAP_USER_ATTRIBUTE_NAME=displayName
```

### Active Directory Configuration

For Microsoft Active Directory, use these settings:

```bash
AVORED_LDAP_SERVER=ldap://ad.example.com
AVORED_LDAP_USER_SEARCH_FILTER=(sAMAccountName={username})
AVORED_LDAP_USER_ATTRIBUTE_EMAIL=userPrincipalName
AVORED_LDAP_USER_ATTRIBUTE_NAME=displayName
```

### OpenLDAP Configuration

For OpenLDAP, use these settings:

```bash
AVORED_LDAP_SERVER=ldap://openldap.example.com
AVORED_LDAP_USER_SEARCH_FILTER=(uid={username})
AVORED_LDAP_USER_ATTRIBUTE_EMAIL=mail
AVORED_LDAP_USER_ATTRIBUTE_NAME=cn
```

## How It Works

1. **Authentication Flow**:
   - User submits login credentials
   - System attempts LDAP authentication first
   - If LDAP authentication succeeds, user is logged in
   - If LDAP authentication fails, system falls back to local authentication
   - If both fail, login is rejected

2. **User Synchronization**:
   - When an LDAP user successfully authenticates for the first time
   - A local user account is automatically created
   - User information is synchronized from LDAP attributes
   - Subsequent logins update the user information if needed

3. **Security**:
   - LDAP passwords are never stored locally
   - TLS encryption is supported for secure communication
   - Connection timeouts prevent hanging connections
   - Proper error handling prevents information disclosure

## Testing LDAP Configuration

Before enabling LDAP in AvoRed, test your configuration using command-line tools:

### Using ldapsearch (Linux/macOS)

```bash
# Test connection and search
ldapsearch -x -H ldap://your-server.com:389 \
  -D "cn=admin,dc=example,dc=com" \
  -w "your-password" \
  -b "ou=users,dc=example,dc=com" \
  "(uid=testuser)"
```

### Using PowerShell (Windows/Active Directory)

```powershell
# Test AD connection
Get-ADUser -Filter "SamAccountName -eq 'testuser'" -Server "ad.example.com"
```

## Troubleshooting

### Common Issues

1. **Connection Timeout**:
   - Check firewall settings
   - Verify LDAP server address and port
   - Increase `AVORED_LDAP_CONNECTION_TIMEOUT`

2. **Authentication Failures**:
   - Verify bind DN and password
   - Check user search base and filter
   - Ensure user exists in LDAP directory

3. **User Not Found**:
   - Verify `AVORED_LDAP_USER_SEARCH_BASE`
   - Check `AVORED_LDAP_USER_SEARCH_FILTER` syntax
   - Ensure user has required attributes (email, name)

4. **TLS/SSL Issues**:
   - Verify certificate validity
   - Check TLS configuration on LDAP server
   - Try without TLS first for testing

### Logging

Enable debug logging to troubleshoot LDAP issues:

```bash
RUST_LOG=debug cargo run
```

Look for log entries containing "LDAP" to see detailed authentication flow.

## Security Considerations

1. **Use TLS**: Always enable TLS in production environments
2. **Bind Account**: Use a dedicated service account with minimal privileges
3. **Network Security**: Ensure LDAP traffic is properly firewalled
4. **Regular Updates**: Keep LDAP server and AvoRed updated
5. **Monitor Access**: Log and monitor LDAP authentication attempts

## Migration from Local Authentication

Existing local users can continue to use their local credentials even after LDAP is enabled. The system supports both authentication methods simultaneously.

To migrate users to LDAP:
1. Ensure users exist in LDAP directory
2. Enable LDAP authentication
3. Users can log in with LDAP credentials
4. Local accounts remain as backup

## Support

For issues with LDAP authentication:
1. Check the troubleshooting section above
2. Review server logs for error messages
3. Test LDAP configuration with external tools
4. Open an issue on the AvoRed GitHub repository
