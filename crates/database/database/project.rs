use sqlx::PgPool;

use crate::Result;

/// TODO.
#[derive(Clone)]
pub struct ProjectsDatabase {
    pool: PgPool,
}

#[derive(Debug)]
pub struct CreateProjectOptions {}

#[derive(Debug)]
pub struct UpdateProjectOptions {
    name: Option<String>,
}

#[derive(Debug)]
pub struct ProjectData {
    name: String,
}

impl ProjectsDatabase {
    /// Returns a new [`ProjectsDatabase`].
    #[inline]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// TODO.
    pub async fn create_project(
        &self,
        account_name: &str,
        project_options: CreateProjectOptions,
    ) -> Result<u64> {
        Ok(0)
    }

    /// TODO.
    pub async fn update_project(
        &self,
        account_name: &str,
        project_name: &str,
        project_options: UpdateProjectOptions,
    ) -> Result<()> {
        Ok(())
    }

    /// TODO.
    pub async fn list_projects(&self, account_name: &str) -> Result<()> {
        Ok(())
    }

    /// TODO.
    pub async fn retrieve_project(&self, account_name: &str, project_name: &str) -> Result<()> {
        Ok(())
    }

    /// TODO.
    pub async fn delete_project(&self, account_name: &str, project_name: &str) -> Result<()> {
        Ok(())
    }
}
