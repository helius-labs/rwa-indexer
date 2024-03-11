use {
    acc_forwarder::{
        fetch_and_send_account, fetch_and_send_data_accounts, fetch_and_send_identity_accounts,
        fetch_and_send_policy_accounts, read_lines,
    },
    anyhow::Context,
    clap::Parser,
    figment::{map, value::Value},
    futures::stream::StreamExt,
    log::warn,
    plerkle_messenger::{MessengerConfig, ACCOUNT_STREAM},
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_sdk::pubkey::Pubkey,
    std::{env, str::FromStr, sync::Arc},
    tokio::sync::Mutex,
    tracing_subscriber::fmt,
};

#[derive(Parser)]
#[command(next_line_help = true)]
struct Args {
    #[arg(long)]
    redis_url: String,
    #[arg(long)]
    rpc_url: String,
    #[command(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Clone)]
enum Action {
    Account {
        #[arg(long)]
        account: String,
    },
    AccountScenario {
        #[arg(long)]
        scenario_file: String,
    },
    MintScenario {
        #[arg(long)]
        scenario_file: String,
    },
    Mint {
        // puts in mint, token, and metadata account
        #[arg(long)]
        mint: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger();

    let args = Args::parse();
    let config_wrapper = Value::from(map! {
        "redis_connection_str" => args.redis_url,
        "pipeline_size_bytes" => 1u128.to_string(),
    });
    let config = config_wrapper.into_dict().unwrap();
    let messenger_config = MessengerConfig {
        messenger_type: plerkle_messenger::MessengerType::Redis,
        connection_config: config,
    };
    let mut messenger = plerkle_messenger::select_messenger(messenger_config)
        .await
        .unwrap();
    messenger.add_stream(ACCOUNT_STREAM).await.unwrap();
    messenger
        .set_buffer_size(ACCOUNT_STREAM, 10000000000000000)
        .await;
    let messenger = Arc::new(Mutex::new(messenger));

    let client = RpcClient::new(args.rpc_url.clone());

    match args.action {
        Action::Account { account } => {
            let pubkey = Pubkey::from_str(&account)
                .with_context(|| format!("failed to parse account {account}"))?;
            fetch_and_send_account(pubkey, &client, &messenger, false).await?;
        }
        Action::AccountScenario { scenario_file } => {
            let mut accounts = read_lines(&scenario_file).await?;
            while let Some(maybe_account) = accounts.next().await {
                match maybe_account {
                    Ok(account) => match account.parse::<Pubkey>() {
                        Ok(acc) => {
                            match fetch_and_send_account(acc, &client, &messenger, false).await {
                                Ok(_) => {}
                                Err(e) => {
                                    warn!("Failed to fetch and send account: {:?}", e);
                                    continue;
                                }
                            }
                        }
                        Err(e) => {
                            warn!("Failed to parse account: {:?}", e);
                            continue;
                        }
                    },
                    Err(e) => {
                        warn!("Failed to get next account: {:?}", e);
                        continue;
                    }
                }
            }
        }
        Action::MintScenario { scenario_file } => {
            // let mut accounts = read_lines(&scenario_file).await?;
            // while let Some(maybe_account) = accounts.next().await {
            //     match maybe_account {
            //         Ok(account) => match account.parse() {},
            //         Err(e) => {
            //             warn!("Failed to get next account: {:?}", e);
            //             continue;
            //         }
            //     }
            // }
        }

        Action::Mint { mint } => {
            let mint =
                Pubkey::from_str(&mint).with_context(|| format!("failed to parse mint {mint}"))?;

            let asset_controller_pda = acc_forwarder::find_asset_controller_pda(&mint).0;
            let data_pda = acc_forwarder::find_data_registry_pda(&mint).0;
            let identifier_pda = acc_forwarder::find_identifier_registry_pda(&mint).0;
            let policy_pda = acc_forwarder::find_policy_engine_pda(&mint).0;

            fetch_and_send_account(mint, &client, &messenger, false).await?;
            for pubkey in &[asset_controller_pda, data_pda, identifier_pda, policy_pda] {
                fetch_and_send_account(*pubkey, &client, &messenger, true).await?;
            }
            fetch_and_send_data_accounts(data_pda, &client, &messenger).await?;
            fetch_and_send_identity_accounts(identifier_pda, &client, &messenger).await?;
            fetch_and_send_policy_accounts(policy_pda, &client, &messenger).await?;
        }
    }

    Ok(())
}

pub fn init_logger() {
    let env_filter = env::var("RUST_LOG").unwrap_or("info".to_string());
    let t = tracing_subscriber::fmt().with_env_filter(env_filter);
    t.event_format(fmt::format::json()).init();
}
