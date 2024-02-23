mod account_updates;
mod ack;
pub mod config;
mod database;
pub mod error;
pub mod metrics;
mod program_handlers;
mod stream;

use crate::{
    account_updates::account_worker,
    ack::ack_worker,
    config::{init_logger, setup_config, PodType},
    database::setup_database,
    error::IndexerError,
    metrics::setup_metrics,
    stream::StreamSizeTimer,
};
use cadence_macros::{is_global_default_set, statsd_count};
use chrono::Duration;
use log::{error, info};
use plerkle_messenger::{
    redis_messenger::RedisMessenger, ConsumptionType, ACCOUNT_STREAM, ACC_BACKFILL,
};
use std::{sync::Arc, time};
use tokio::{signal, task::JoinSet};

#[tokio::main(flavor = "multi_thread")]
pub async fn main() -> Result<(), IndexerError> {
    init_logger();
    info!("Starting indexer");

    // Setup Configuration and Metrics ---------------------------------------------

    // Pull Env variables into config struct
    let config = setup_config();

    // Optionally setup metrics if config demands it
    setup_metrics(&config);

    // One pool many clones, this thing is thread safe and send sync
    let database_pool = setup_database(config.clone()).await;

    //The pod_type determines the type of pod the indexer is running in
    let pod_type = config.pod_type.clone().unwrap_or(PodType::Regular);

    info!("Starting Program with Pod Type: {}", pod_type);

    let mut tasks = JoinSet::new();

    // Stream Size Timers ----------------------------------------
    // Setup Stream Size Timers, these are small processes that run every 30 seconds and farm metrics for the size of the streams.
    // If metrics are disabled, these will not run.
    let stream_metrics_timer = Duration::seconds(30).to_std().unwrap();

    let mut timer_acc = StreamSizeTimer::new(
        stream_metrics_timer,
        config.messenger_config.clone(),
        match pod_type {
            PodType::Backfiller => ACC_BACKFILL,
            PodType::Regular => ACCOUNT_STREAM,
        },
    )?;
    if let Some(t) = timer_acc.start::<RedisMessenger>().await {
        tasks.spawn(t);
    }

    let (_ack_task, ack_sender) =
        ack_worker::<RedisMessenger>(config.get_messenger_client_config());
    for i in 0..config.get_account_stream_worker_count() {
        let _account = account_worker::<RedisMessenger>(
            database_pool.clone(),
            config.clone(),
            ack_sender.clone(),
            if i == 0 {
                ConsumptionType::Redeliver
            } else {
                ConsumptionType::New
            },
            &pod_type,
        );
    }

    metric! {
        statsd_count!("indexer.startup", 1);
    }
    match signal::ctrl_c().await {
        Ok(()) => {}
        Err(err) => {
            error!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        }
    }

    tasks.shutdown().await;

    Ok(())
}
