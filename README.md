# Priority Queue Service

This project implements a Priority Queue Service using Rust, Actix Web, and Redis. The service allows you to manage URLs with different priorities, handle retries with exponential backoff, and manage dead-letter queues for failed retries. It provides a RESTful API for adding URLs to the queue, fetching URLs for processing, and retrying failed URLs.

## Features

- **Priority Queue:** URLs are stored with a priority score. Lower scores have higher priority.
- **Retry Logic:** If processing a URL fails, it can be retried with an exponential backoff strategy.
- **Dead-Letter Queue:** URLs that fail too many times are moved to a dead-letter queue for further inspection or manual processing.
- **Asynchronous Operations:** Uses asynchronous Redis operations and Actix Web for efficient handling of HTTP requests.

## Project Structure

```plaintext
priority_queue_service/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── api/
│   │   ├── mod.rs
│   │   ├── add_url.rs
│   │   ├── fetch_url.rs
│   │   ├── retry_url.rs
│   ├── services/
│   │   ├── mod.rs
│   │   ├── priority_queue_service.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── url_data.rs
│   ├── config/
│   │   ├── mod.rs
│   │   ├── config.rs
│   └── utils/
│       ├── mod.rs
│       ├── logging.rs
│       ├── time.rs
├── Cargo.toml
└── .env
```

### `src/`

- **`main.rs`**: Entry point of the application.
- **`lib.rs`**: Library file that includes all modules.
- **`api/`**: Contains Actix Web route handlers.
  - **`add_url.rs`**: Handles the `/add_url` endpoint.
  - **`fetch_url.rs`**: Handles the `/fetch_url` endpoint.
  - **`retry_url.rs`**: Handles the `/retry_url` endpoint.
- **`services/`**: Contains the core service logic.
  - **`priority_queue_service.rs`**: Implements the `PriorityQueueService`.
- **`models/`**: Contains data models used in the API.
  - **`url_data.rs`**: Defines the `UrlData` struct.
- **`config/`**: Handles configuration settings.
  - **`config.rs`**: Manages configuration such as Redis URL.
- **`utils/`**: Contains utility functions.
  - **`logging.rs`**: Initializes the logging system.
  - **`time.rs`**: Handles time-related utilities.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (latest stable version)
- [Redis](https://redis.io/) (running locally or remotely)
- [Cargo](https://doc.rust-lang.org/cargo/) (Rust package manager)

### Setup

1. **Clone the repository:**

   ```bash
   git clone https://github.com/your-username/priority_queue_service.git
   cd priority_queue_service
   ```

2. **Set up environment variables:**

   Create a `.env` file in the root directory with the following content:

   ```env
   REDIS_URL=redis://127.0.0.1/
   ```

3. **Install dependencies:**

   Cargo will handle dependencies for you. Just build the project:

   ```bash
   cargo build
   ```

4. **Run the server:**

   ```bash
   cargo run
   ```

   The server will start on `http://127.0.0.1:8080`.

## Usage

### API Endpoints

#### 1. Add a URL to the Queue

- **Endpoint:** `POST /add_url`
- **Description:** Adds a new URL to the priority queue.
- **Request Body:**

  ```json
  {
      "url": "http://example.com",
      "priority": 5
  }
  ```

- **Response:**

  ```json
  {
      "message": "URL added to the queue: http://example.com"
  }
  ```

#### 2. Fetch the Next URL

- **Endpoint:** `GET /fetch_url`
- **Description:** Fetches the next URL from the queue for processing.
- **Response:**

  ```json
  {
      "url": "http://example.com",
      "score": 4.9
  }
  ```

  If no URLs are available:

  ```json
  {
      "message": "No URLs available in the queue"
  }
  ```

#### 3. Retry a Failed URL

- **Endpoint:** `POST /retry_url`
- **Description:** Retries a URL that failed to process.
- **Request Body:**

  ```json
  {
      "url": "http://example.com",
      "priority": 5
  }
  ```

- **Response:**

  ```json
  {
      "message": "URL scheduled for retry: http://example.com"
  }
  ```

### Testing with `curl`

**Add a URL:**

```bash
curl -X POST http://localhost:8080/add_url -H "Content-Type: application/json" -d '{"url":"http://example.com", "priority":3}'
```

**Fetch a URL:**

```bash
curl -X GET http://localhost:8080/fetch_url
```

**Retry a Failed URL:**

```bash
curl -X POST http://localhost:8080/retry_url -H "Content-Type: application/json" -d '{"url":"http://example.com", "priority":2}'
```

## Logging

Logs are handled using the `env_logger` crate. Ensure that logging is set up by calling `utils::logging::init_logger()` in the `main.rs` file.

## Configuration

- Redis connection settings and other configurations are managed through environment variables. Adjust these settings in the `.env` file or within `src/config/config.rs`.
