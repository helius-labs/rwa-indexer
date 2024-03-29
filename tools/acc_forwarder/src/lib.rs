use common::utils::{
    ASSET_CONTROLLER_PROGRAM_ID, DATA_REGISTRY_PROGRAM_ID, IDENTIFIER_REGISTRY_PROGRAM_ID,
    POLICY_ENGINE_PROGRAM_ID,
};
use solana_client::{
    rpc_config::RpcProgramAccountsConfig,
    rpc_filter::{Memcmp, RpcFilterType},
};
use {
    anyhow::Context,
    futures::stream::{BoxStream, StreamExt},
    log::{error, info},
    plerkle_messenger::ACCOUNT_STREAM,
    plerkle_serialization::{
        serializer::serialize_account, solana_geyser_plugin_interface_shims::ReplicaAccountInfoV2,
    },
    serde::de::DeserializeOwned,
    solana_account_decoder::{UiAccount, UiAccountEncoding},
    solana_client::client_error::Result as RpcClientResult,
    solana_client::{
        nonblocking::rpc_client::RpcClient, rpc_config::RpcAccountInfoConfig,
        rpc_request::RpcRequest, rpc_response::Response as RpcResponse,
    },
    solana_program::pubkey::Pubkey,
    solana_sdk::{
        account::Account,
        commitment_config::{CommitmentConfig, CommitmentLevel},
    },
    std::{fmt, io::Result as IoResult, sync::Arc},
    tokio::sync::Mutex,
    tokio::{
        fs::File,
        io::{stdin, AsyncBufReadExt, BufReader},
        time::{sleep, Duration},
    },
    tokio_stream::wrappers::LinesStream,
};

const REGISTRY_OFFSET: usize = 9;

pub async fn fetch_and_send_program_accounts(
    program: Pubkey,
    client: &RpcClient,
    messenger: &Arc<Mutex<Box<dyn plerkle_messenger::Messenger>>>,
    filters: Vec<RpcFilterType>,
) -> anyhow::Result<()> {
    let accounts = client
        .get_program_accounts_with_config(
            &program,
            RpcProgramAccountsConfig {
                filters: Some(filters),
                account_config: RpcAccountInfoConfig {
                    encoding: Some(solana_account_decoder::UiAccountEncoding::Base64),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .await?;

    let current_slot = client
        .get_slot()
        .await
        .context("Failed to get current slot")?;

    for (account_pubkey, account_info) in accounts {
        send_account(account_pubkey, account_info, current_slot, messenger)
            .await
            .context(format!("Failed to send account {}", account_pubkey))?;
    }
    Ok(())
}

pub async fn fetch_and_send_identity_accounts(
    registry: Pubkey,
    client: &RpcClient,
    messenger: &Arc<Mutex<Box<dyn plerkle_messenger::Messenger>>>,
) -> anyhow::Result<()> {
    const IDENTITY_ACCOUNT_LEN: u64 = 83;

    fetch_and_send_program_accounts(
        IDENTIFIER_REGISTRY_PROGRAM_ID,
        client,
        messenger,
        vec![
            RpcFilterType::DataSize(IDENTITY_ACCOUNT_LEN),
            RpcFilterType::Memcmp(Memcmp::new_raw_bytes(
                REGISTRY_OFFSET,
                registry.to_bytes().to_vec(),
            )),
        ],
    )
    .await?;
    Ok(())
}

pub async fn fetch_and_send_tracker_account(
    registry: Pubkey,
    client: &RpcClient,
    messenger: &Arc<Mutex<Box<dyn plerkle_messenger::Messenger>>>,
) -> anyhow::Result<()> {
    const TRACKER_ACCOUNT_LEN: u64 = 473;

    fetch_and_send_program_accounts(
        ASSET_CONTROLLER_PROGRAM_ID,
        client,
        messenger,
        vec![
            RpcFilterType::DataSize(TRACKER_ACCOUNT_LEN),
            RpcFilterType::Memcmp(Memcmp::new_raw_bytes(
                REGISTRY_OFFSET,
                registry.to_bytes().to_vec(),
            )),
        ],
    )
    .await?;
    Ok(())
}

pub async fn fetch_and_send_data_accounts(
    registry: Pubkey,
    client: &RpcClient,
    messenger: &Arc<Mutex<Box<dyn plerkle_messenger::Messenger>>>,
) -> anyhow::Result<()> {
    const DATA_ACCOUNT_LEN: u64 = 337;

    fetch_and_send_program_accounts(
        DATA_REGISTRY_PROGRAM_ID,
        client,
        messenger,
        vec![
            RpcFilterType::DataSize(DATA_ACCOUNT_LEN),
            RpcFilterType::Memcmp(Memcmp::new_raw_bytes(
                REGISTRY_OFFSET,
                registry.to_bytes().to_vec(),
            )),
        ],
    )
    .await?;
    Ok(())
}

pub async fn fetch_and_send_policy_accounts(
    registry: Pubkey,
    client: &RpcClient,
    messenger: &Arc<Mutex<Box<dyn plerkle_messenger::Messenger>>>,
) -> anyhow::Result<()> {
    const POLICY_ACCOUNT_LEN: u64 = 69;

    fetch_and_send_program_accounts(
        POLICY_ENGINE_PROGRAM_ID,
        client,
        messenger,
        vec![
            RpcFilterType::DataSize(POLICY_ACCOUNT_LEN),
            RpcFilterType::Memcmp(Memcmp::new_raw_bytes(
                REGISTRY_OFFSET,
                registry.to_bytes().to_vec(),
            )),
        ],
    )
    .await?;
    Ok(())
}

/// fetch account from node and send it to redis
pub async fn fetch_and_send_account(
    pubkey: Pubkey,
    client: &RpcClient,
    messenger: &Arc<Mutex<Box<dyn plerkle_messenger::Messenger>>>,
    ok_to_fail: bool,
) -> anyhow::Result<()> {
    let fetch_result = fetch_account(pubkey, client).await;
    let (account, slot) = match fetch_result {
        Ok((account, slot)) => (account, slot),
        Err(e) => {
            if ok_to_fail {
                return Ok(());
            } else {
                return Err(anyhow::anyhow!("Failed to fetch account: {:?}", e));
            }
        }
    };
    send_account(pubkey, account, slot, messenger).await
}

/// fetch account and slot with retries
pub async fn fetch_account(pubkey: Pubkey, client: &RpcClient) -> anyhow::Result<(Account, u64)> {
    const CONFIG: RpcAccountInfoConfig = RpcAccountInfoConfig {
        encoding: Some(UiAccountEncoding::Base64Zstd),
        commitment: Some(CommitmentConfig {
            commitment: CommitmentLevel::Finalized,
        }),
        data_slice: None,
        min_context_slot: None,
    };

    let response: RpcResponse<Option<UiAccount>> = rpc_tx_with_retries(
        client,
        RpcRequest::GetAccountInfo,
        serde_json::json!([pubkey.to_string(), CONFIG]),
        3,
        pubkey,
    )
    .await
    .with_context(|| format!("failed to get account {pubkey}"))?;

    let account: Account = response
        .value
        .ok_or_else(|| anyhow::anyhow!("failed to get account {pubkey}"))?
        .decode()
        .ok_or_else(|| anyhow::anyhow!("failed to parse account {pubkey}"))?;

    Ok((account, response.context.slot))
}

/// send account data to redis
pub async fn send_account(
    pubkey: Pubkey,
    account: Account,
    slot: u64,
    messenger: &Arc<Mutex<Box<dyn plerkle_messenger::Messenger>>>,
) -> anyhow::Result<()> {
    let fbb = flatbuffers::FlatBufferBuilder::new();

    let account_info = ReplicaAccountInfoV2 {
        pubkey: &pubkey.to_bytes(),
        lamports: account.lamports,
        owner: &account.owner.to_bytes(),
        executable: account.executable,
        rent_epoch: account.rent_epoch,
        data: &account.data,
        write_version: 0,
        txn_signature: None,
    };
    let is_startup = false;

    let fbb = serialize_account(fbb, &account_info, slot, is_startup);
    let bytes = fbb.finished_data();

    messenger.lock().await.send(ACCOUNT_STREAM, bytes).await?;
    sleep(Duration::from_millis(10)).await;
    info!("sent account {} to stream", pubkey);

    Ok(())
}

pub async fn rpc_tx_with_retries<T, E>(
    client: &RpcClient,
    request: RpcRequest,
    value: serde_json::Value,
    max_retries: u8,
    error_key: E,
) -> RpcClientResult<T>
where
    T: DeserializeOwned,
    E: fmt::Debug,
{
    let mut retries = 0;
    let mut delay = Duration::from_millis(500);
    loop {
        match client.send(request, value.clone()).await {
            Ok(value) => return Ok(value),
            Err(error) => {
                if retries < max_retries {
                    error!("retrying {request} {error_key:?}: {error}");
                    sleep(delay).await;
                    delay *= 2;
                    retries += 1;
                } else {
                    return Err(error);
                }
            }
        }
    }
}

pub async fn read_lines(path: &str) -> anyhow::Result<BoxStream<'static, IoResult<String>>> {
    Ok(if path == "-" {
        LinesStream::new(BufReader::new(stdin()).lines()).boxed()
    } else {
        let file = File::open(path)
            .await
            .with_context(|| format!("failed to read file: {:?}", path))?;
        LinesStream::new(BufReader::new(file).lines()).boxed()
    }
    .filter_map(|line| async move {
        match line {
            Ok(line) => {
                let line = line.trim();
                (!line.is_empty()).then(|| Ok(line.to_string()))
            }
            Err(error) => Some(Err(error)),
        }
    })
    .boxed())
}
