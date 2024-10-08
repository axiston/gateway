use axum::http::StatusCode;
use axum::routing::{delete, get, patch, post};
use axum::Router;
use serde::{Deserialize, Serialize};

use crate::extract::{Json, Path};
use crate::handler::project::ProjectPathParams;
use crate::handler::Result;
use crate::service::AppState;

#[must_use]
#[derive(Debug, Deserialize)]
struct InviteMemberRequest {
    pub username: String,
}

#[must_use]
#[derive(Debug, Serialize)]
struct InviteMemberResponse {}

/// `POST ./:account/:project/members`
#[tracing::instrument]
async fn invite_another_member(
    Path(params): Path<ProjectPathParams>,
    Json(request): Json<InviteMemberRequest>,
) -> Result<(StatusCode, Json<InviteMemberResponse>)> {
    let response = InviteMemberResponse {};
    Ok((StatusCode::CREATED, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize)]
struct ModifyMemberRequest {
    pub username: String,
}

#[must_use]
#[derive(Debug, Serialize)]
struct ModifyMemberResponse {}

/// `PATCH ./:account/:project/members`
#[tracing::instrument]
async fn modify_one_member(
    Path(params): Path<ProjectPathParams>,
    Json(request): Json<ModifyMemberRequest>,
) -> Result<(StatusCode, Json<ModifyMemberResponse>)> {
    let response = ModifyMemberResponse {};
    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize)]
struct ListMembersRequest {}

#[must_use]
#[derive(Debug, Serialize)]
struct ListMembersResponse {}

/// `GET ./:account/:project/members`
#[tracing::instrument]
async fn list_all_members(
    Path(params): Path<ProjectPathParams>,
    Json(request): Json<ListMembersRequest>,
) -> Result<(StatusCode, Json<ListMembersResponse>)> {
    let response = ListMembersResponse {};
    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize)]
struct DeleteMemberRequest {
    pub username: String,
}

#[must_use]
#[derive(Debug, Serialize)]
struct DeleteMemberResponse {}

/// `DELETE ./:account/:project/members`
#[tracing::instrument]
async fn delete_one_member(
    Path(params): Path<ProjectPathParams>,
    Json(request): Json<DeleteMemberRequest>,
) -> Result<(StatusCode, Json<DeleteMemberResponse>)> {
    let response = DeleteMemberResponse {};
    Ok((StatusCode::OK, Json(response)))
}

/// Returns a [`Router`] with all related routes.
pub fn routes() -> Router<AppState> {
    let path = "/accounts/:account/projects/:project/members";
    Router::new()
        .route(path, post(invite_another_member))
        .route(path, patch(modify_one_member))
        .route(path, get(list_all_members))
        .route(path, delete(delete_one_member))
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::project::members;
    use crate::service::{AppConfig, AppState};

    #[tokio::test]
    async fn routes() -> anyhow::Result<()> {
        let config = AppConfig::default();
        let state = AppState::connect(config).await?;
        let app = members::routes().with_state(state);
        let server = TestServer::new(app)?;

        Ok(())
    }
}
