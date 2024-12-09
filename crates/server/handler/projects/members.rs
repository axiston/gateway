use axum::http::StatusCode;
use axum::routing::{delete, get, patch, post, Router};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::extract::{Json, Path};
use crate::handler::projects::ProjectPathParams;
use crate::handler::Result;
use crate::service::AppState;

#[must_use]
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "projects.ts")]
struct InviteMemberRequest {
    pub email: String,
}

#[must_use]
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "projects.ts")]
struct InviteMemberResponse {
    pub account: String,
}

/// `POST ./projects/:project/members`
#[tracing::instrument]
async fn invite_another_member(
    Path(params): Path<ProjectPathParams>,
    Json(request): Json<InviteMemberRequest>,
) -> Result<(StatusCode, Json<InviteMemberResponse>)> {
    let response = InviteMemberResponse {
        account: "".to_owned(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

#[must_use]
#[derive(Debug, Default, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "projects.ts")]
struct ListMembersRequest {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[must_use]
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "projects.ts")]
struct ListMembersResponse {
    pub ignore: (),
}

/// Returns a list of all current and invited members.
///
/// `GET ./projects/:project/members`
#[tracing::instrument]
async fn list_all_members(
    Path(params): Path<ProjectPathParams>,
    request: Option<Json<ListMembersRequest>>,
) -> Result<(StatusCode, Json<ListMembersResponse>)> {
    let Json(request) = request.unwrap_or_default();
    let limit = request.limit.unwrap_or(20);
    let offset = request.offset.unwrap_or_default();

    let response = ListMembersResponse { ignore: () };
    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "projects.ts")]
struct DeleteMemberRequest {
    pub account: String,
}

/// `DELETE ./projects/:project/members`
#[tracing::instrument]
async fn delete_one_member(
    Path(params): Path<ProjectPathParams>,
    Json(request): Json<DeleteMemberRequest>,
) -> Result<StatusCode> {
    Ok(StatusCode::OK)
}

/// Returns a [`Router`] with all related routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/projects/:project/members", post(invite_another_member))
        .route("/projects/:project/members", get(list_all_members))
        .route("/projects/:project/members", delete(delete_one_member))
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::projects::members::routes;
    use crate::service::{AppConfig, AppState};

    #[tokio::test]
    async fn routes() -> anyhow::Result<()> {
        let config = AppConfig::default();
        let state = AppState::connect(config).await?;
        let app = routes().with_state(state);
        let server = TestServer::new(app)?;

        Ok(())
    }
}
