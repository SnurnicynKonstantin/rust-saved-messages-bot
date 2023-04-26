use std::sync::Arc;
use axum::{Extension, Json};
use crate::api::ApiContext;
use crate::api::responses::TestResponse;
use crate::api::*;

pub async fn get_tests(
    Extension(ctx): Extension<Arc<ApiContext>>,
) -> Result<Json<TestResponse>> {
    let tests = ctx
        .test_service
        .get_tests()
        .await;

    Ok(Json(TestResponse::from(tests)))
}

// pub async fn post_address_check(
//     Json(req): Json<AddressCheckRequest>,
//     Extension(ctx): Extension<Arc<ApiContext>>,
// ) -> Result<Json<CheckedAddressResponse>> {
//     let address = ctx
//         .ton_service
//         .check_address(req.address)
//         .await
//         .map(AddressValidResponse::new);
//
//     Ok(Json(CheckedAddressResponse::from(address)))
// }