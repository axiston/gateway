//! TODO.

use sea_orm::sea_query::{Alias, IntoIden};
use sea_orm::DynIden;
use sea_orm_migration::{MigrationTrait, MigratorTrait};

use crate::{AppDatabase, Result};

/// TODO.
#[derive(Debug, Clone, Default)]
pub struct AppDatabaseMigrator;

impl AppDatabaseMigrator {
    /// Returns a new [`AppDatabaseMigrator`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait::async_trait]
impl MigratorTrait for AppDatabaseMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![]
    }

    fn migration_table_name() -> DynIden {
        Alias::new("migrations").into_iden()
    }
}

/// Extends [`AppDatabase`] with migration methods.
pub trait AppDatabaseExt {
    /// Applies `steps` pending migrations.
    async fn apply_migrations(&self, steps: Option<u32>) -> Result<()>;

    /// Rolls back `steps` pending migrations.
    async fn rollback_migrations(&self, steps: Option<u32>) -> Result<()>;
}

impl AppDatabaseExt for AppDatabase {
    async fn apply_migrations(&self, steps: Option<u32>) -> Result<()> {
        let conn = self.as_database_connection();
        AppDatabaseMigrator::up(conn, steps).await?;
        Ok(())
    }

    async fn rollback_migrations(&self, steps: Option<u32>) -> Result<()> {
        let conn = self.as_database_connection();
        AppDatabaseMigrator::down(conn, steps).await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{AppDatabase, AppDatabaseExt, Result};

    #[tokio::test]
    async fn migrator() -> Result<()> {
        let addr = "postgresql://usr:pwd@localhost:5432/db";
        let conn = AppDatabase::connect_single_instance(addr).await?;
        let _ = conn.apply_migrations(None).await?;
        Ok(())
    }
}
