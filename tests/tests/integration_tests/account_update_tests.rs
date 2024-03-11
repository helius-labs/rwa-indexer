use function_name::named;

use itertools::Itertools;
use rwa_api::api::{self, ApiContract};

use serial_test::serial;

use super::common::*;

#[tokio::test]
#[serial]
#[named]
async fn test_get_rwa_accounts_by_mint() {
    let name = trim_test_name(function_name!());
    let setup = TestSetup::new(name.clone()).await;

    let seeds: Vec<SeedEvent> = vec![seed_token_mint(
        "Ea1yrC1xRXd6tWcHL4yhGRB31j6jTwdeqd3e9LHaYUwj",
    )];
    apply_migrations_and_delete_data(setup.db.clone()).await;
    index_seed_events(&setup, seeds.iter().collect_vec()).await;
    let request: api::GetRwaAccountsByMint = serde_json::from_str(
        r#"{
        "id": "Ea1yrC1xRXd6tWcHL4yhGRB31j6jTwdeqd3e9LHaYUwj"
    }"#,
    )
    .unwrap();
    let response = setup
        .rwa_api
        .get_rwa_accounts_by_mint(request)
        .await
        .unwrap();
    insta::assert_json_snapshot!(setup.name.clone(), response);
}
