//! LDAP support for the `r2d2` connection pool.
use ldap3::{exop::WhoAmI, LdapConn, LdapError};
use r2d2::ManageConnection;

/// An `r2d2::ManageConnection` for `ldap3::LdapConn`s.
///
/// ## Example
///
/// ```no_run
/// use std::thread;
/// use r2d2_ldap::LDAPConnectionManager;
///
/// fn main() {
///     let pool = r2d2::Pool::new(LDAPConnectionManager("ldap://example.org")).unwrap();
///
///     for i in 0..10i32 {
///         let pool = pool.clone();
///         thread::spawn(move || {
///             let mut ldap = pool.get().unwrap();
///             ldap.simple_bind("uid=john,cn=users,dc=example,dc=org", "password").unwrap();
///         });
///     }
/// }
/// ```
pub struct LDAPConnectionManager(&'static str);

impl ManageConnection for LDAPConnectionManager {
    type Connection = LdapConn;
    type Error = LdapError;

    fn connect(&self) -> Result<LdapConn, LdapError> {
        LdapConn::new(self.0)
    }
    fn is_valid(&self, conn: &mut LdapConn) -> Result<(), LdapError> {
        conn.extended(WhoAmI).map(|_| ())
    }
    fn has_broken(&self, conn: &mut LdapConn) -> bool {
        conn.extended(WhoAmI).is_err()
    }
}
