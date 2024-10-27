# crypto-token-metrics-api - a midgard api clone

## API Documentation - https://crypto-token-metrics-api.onrender.com/swagger-ui/

## Project Structure

- **src/**: Contains the main application logic.
  - **main.rs**: Entry point that initializes the `server` and sets up the primary application components and also initializes the `database`.
  - **api_docs.rs/**:  Handles the setup and integration of `API documentation`, linking models and routes for clarity.
  - **models/**: Defines data models representing `MongoDB collections`, such as `PoolDepthPriceHistory` and `SwapHistory`.
  - **controllers/**: Houses `database interaction functions`, including `get_rune_pool_history_api`, which processes user queries to the database.
  - **routes/**: Manages `HTTP routing` functions, grouping different `API endpoints` (e.g., `/depths`, `/earnings`) that interact with the database and generate responses.
  - **services/**: Contains `fetchers` that interact with the `Midgard API`, retrieve necessary data, and store it in the local application database to replicate Midgard’s behavior.
  - **utils/**: Provides `utility functions` that reduce `code duplication` and enforce best practices across the project.