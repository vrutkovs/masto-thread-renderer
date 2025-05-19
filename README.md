# Masto-Thread-Renderer

A web service that renders Mastodon threads in various formats, allowing for easy embedding and conversion of Mastodon conversations.

## Features

- Render Mastodon threads with embedded toots
- Convert Mastodon threads to Markdown
- Support for media attachments and alt texts
- Automatic thread reconstruction, including finding the root toot and all replies

## Installation

### Prerequisites

- Rust (latest stable version recommended)
- Cargo
- Node.js and npm (for building frontend assets)

### Building

Clone the repository and build the project:

```bash
git clone https://github.com/yourusername/masto-thread-renderer.git
cd masto-thread-renderer
cargo build --release
```

The build process will automatically handle npm dependencies through the `npm_rs` crate.

## Usage

### Running the server

```bash
cargo run
```

By default, the server will run on `http://localhost:8000`.

### Environment Variables

- `RUST_LOG`: Sets the log level (e.g., `info`, `debug`, `trace`)

### Health Check

```
GET /healthz
```

Returns `OK` if the service is running properly.

### Index Page

```
GET /
```

Renders the main index page with a form to enter a Mastodon toot URL.

### Thread Embedding

```
GET /thread?url={mastodon_toot_url}
```

Renders a full thread with embedded toots, starting from the provided URL.

### Markdown Conversion

```
GET /markdown?url={mastodon_toot_url}
```

Converts a Mastodon thread to Markdown format, including media attachments.

## How It Works

1. When provided with a Mastodon toot URL, the service finds the original toot.
2. It then traverses both ancestors and descendants to reconstruct the full thread.
3. For embedding, it generates the necessary iframe code to display each toot.
4. For Markdown, it extracts the content and media attachments and converts them to Markdown format.

## Development

### Running Tests

```bash
cargo test
```

### Template System

The application uses Askama templates located in the `public/html` directory:

- `index.html` - The main index page
- `thread.html` - Template for rendering embedded threads
- `markdown.html` - Template for Markdown conversion
- `error.html` - Error page template
- `components/` - Reusable template components

## License

[Apache-2.0 License](LICENSE)
