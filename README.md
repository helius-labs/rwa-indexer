# Real World Assets Indexer

Indexer for real world assets, facilitating the integration and management of real-world financial assets on the blockchain.

## Components

- **Indexer**: A background system processing messages from a Messenger, parsing accounts with Parsers to store in a Postgres database.

- **API**: Provides access to RWA accounts via a JSON RPC API.

## Programs Indexed

1. **Asset Management**: For asset issuers to create rules for asset management, leveraging Token-Extensions for transfer controls and privacy.

2. **Policy Engine Program**: Enables on-chain management of token transfer policies, including transaction size, count, and velocity.

3. **Identity Registry**: Allows asset issuers to manage wallet-to-identity relationships, crucial for the Asset Management program.

4. **Asset Data Registry**: For managing naive asset data like ownership (titles, deeds, audits), legal (filings, liens), and informational data.

The indexer captures account data from RWA programs through geyser, utilizing the [digital-asset-validator plugin](https://github.com/helius-labs/digital-asset-validator-plugin) to forward accounts to Redis. The data is then serialized and inserted into a database for API retrieval.

More about [RWA can be found here](https://github.com/bridgesplit/rwa-token).

## Running the Indexer Locally

To get the indexer up and running on your local machine, follow these steps:

1. **Start Local Services**:
   Ensure you have Postgres and Redis instances running locally.

2. **Environment Variables**:
   Set the necessary environment variables:

    ```shell
    export INDEXER_DATABASE_CONFIG='{listener_channel="backfill_item_added", url="postgres://postgres@localhost/rwa"}'
    export INDEXER_RPC_CONFIG='{url="http://localhost:8899", commitment="finalized"}'
    export INDEXER_MESSENGER_CONFIG='{messenger_type="Redis", connection_config={ batch_size=1, redis_connection_str="redis://localhost" } }'
    export INDEXER_METRICS_HOST=127.0.0.1
    export INDEXER_METRICS_PORT=8125
    ```
    The above assumes a local postgres database `rwa`

3. **Run the Indexer**:
   Navigate to the `indexer` directory and start the indexer:

    ```shell
    cargo run -p indexer
    ```

4. **API Environment Variable**:
   Configure the environment for the API:

    ```shell
    export APP_DATABASE_URL=postgres://postgres@localhost/rwa
    export APP_SERVER_PORT=9090
    ```

5. **Run the API**:
   In the `api` folder, initiate the API service:

    ```shell
    cargo run -p rwa_api
    ```

6. **Account Forwarder Tool**:
   Use the account forwarder to process RWA accounts for a specific token mint:

    ```shell
    cargo run -- --redis-url 'redis://localhost:6379' --rpc-url '<RPC_URL>' mint --mint <MINT_ADDRESS>
    ```

## Running Tests Locally

Ensure all changes to the indexer or API are covered by integration tests:

- Refer to `tests.README.md` for detailed testing instructions.
- Execute tests from the `tests` directory:

    ```shell
    cargo test
    ```
