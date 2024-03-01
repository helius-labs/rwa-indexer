use function_name::named;

use itertools::Itertools;
use rwa_api::api::{self, ApiContract};

use serial_test::serial;

use super::common::*;

#[tokio::test]
#[serial]
#[named]
async fn test_get_all_accounts() {
    let name = trim_test_name(function_name!());
    let setup = TestSetup::new(name.clone()).await;

    let seeds: Vec<SeedEvent> = vec![seed_token_mint(
        "mZ7ZGCykdQgvDsmDuXFsrCcFS77NAQxQZGhHNWQYvPe",
    )];
    apply_migrations_and_delete_data(setup.db.clone()).await;
    index_seed_events(&setup, seeds.iter().collect_vec()).await;
    let request: api::GetAllAccounts = serde_json::from_str(
        r#"{
        "id": "mZ7ZGCykdQgvDsmDuXFsrCcFS77NAQxQZGhHNWQYvPe"
    }"#,
    )
    .unwrap();
    let response = setup.rwa_api.get_all_accounts(request).await.unwrap();
    insta::assert_json_snapshot!(setup.name.clone(), response);
}
