use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::Path;
use crate::api::ApiContext;
use crate::api::responses::AccountResponse;
use crate::api::*;
use crate::api::requests::CreateAccountRequest;

pub async fn get_accounts(
    Extension(ctx): Extension<Arc<ApiContext>>,
) -> Result<Json<AccountResponse>> {
    let accounts = ctx
        .account_service
        .get_accounts()
        .await;

    Ok(Json(AccountResponse::from(accounts)))
}

pub async fn save_account(
    Extension(ctx): Extension<Arc<ApiContext>>,
    Json(req): Json<CreateAccountRequest>
) -> Result<Json<AccountResponse>> {
    let account = ctx
        .account_service
        .save_account(req.into())
        .await;

    Ok(Json(AccountResponse::from(account)))
}

pub async fn get_account_by_id(
    Extension(ctx): Extension<Arc<ApiContext>>,
    Path(id): Path<i64>,
) -> Result<Json<AccountResponse>> {
    let account = ctx
        .account_service
        .get_account_by_id(id)
        .await;

    Ok(Json(AccountResponse::from(account)))
}