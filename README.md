# Reeverb

Open-source testimonial platform built with Rust. Collect, manage, and display social proof on your website.

Self-hosted alternative to Senja, Testimonial.to, and Famewall.

## Features

- Collect text and video testimonials via customizable forms
- Embeddable widgets (Wall of Love, carousel, cards, popups, and more)
- Import from Twitter/X, Google Reviews, Product Hunt, and other platforms
- Tag, filter, and manage testimonials with an approval workflow
- Analytics dashboard for widget performance
- AI-powered sentiment analysis and video transcription
- Full REST API with JWT authentication
- Self-host with Docker or use Reeverb Cloud

## Stack

| Layer | Technology |
|-------|-----------|
| Backend | [Rapina](https://github.com/arferreira/rapina) (Rust web framework) |
| Database | PostgreSQL via SeaORM |
| Storage | S3-compatible (MinIO, AWS S3, Cloudflare R2) |
| Widgets | Lightweight WASM bundles with Shadow DOM isolation |
| Auth | JWT + OAuth2 (Google, GitHub) |

## Quick Start

### Prerequisites

- Rust 1.85+
- PostgreSQL 16+
- Docker (optional, for self-hosting)

### Development

```bash
git clone https://github.com/zaptech-dev/reeverb.git
cd reeverb
cp .env.example .env
# Edit .env with your database credentials
cargo run
```

The server starts at `http://localhost:3000`.

### Docker

```bash
docker compose up -d
```

## API

All endpoints are documented via OpenAPI. Once running, visit `/__rapina/openapi.json` for the full spec.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT — see [LICENSE](LICENSE).
