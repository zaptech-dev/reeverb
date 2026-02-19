# Reeverb

Open-source testimonial platform built with Rust. Collect, manage, and display social proof on your website.

Self-hosted alternative to Senja, Testimonial.to, and Famewall.

## Status

Early development. The foundation is in place — auth, projects, testimonials, tags, and a basic dashboard. See the [roadmap](#roadmap) for what's coming next.

## Stack

| Layer | Technology |
|-------|-----------|
| Backend | [Rapina](https://userapina.com) (Rust web framework) |
| Frontend | [Leptos](https://leptos.dev) (Rust/WASM, CSR) |
| Database | PostgreSQL via SeaORM |
| Auth | JWT (email/password) |
| Deploy | Docker, Railway |

## What works today

- User registration and login (JWT)
- Projects CRUD
- Testimonials CRUD (text, with nested project routes)
- Tags and testimonial tagging
- Dashboard with project management
- Single binary serves both the API and the WASM dashboard
- Railway deployment with auto-deploy on push to main
- OpenAPI spec generation
- CI pipeline (check, fmt, clippy, test, API checks)

## Quick Start

### Prerequisites

- Rust 1.88+
- PostgreSQL 16+
- [Trunk](https://trunkrs.dev) (for building the dashboard frontend)

### Development

```bash
git clone https://github.com/zaptech-dev/reeverb.git
cd reeverb
cp .env.example .env
# Edit .env with your database credentials
cargo run
```

The API starts at `http://localhost:3000`. For frontend development, run `trunk serve` in `crates/dashboard/` (serves on `:8080`, proxies API calls to `:3000`).

### Docker

```bash
docker compose up -d
```

## API

All endpoints are documented via OpenAPI. Once running, visit `/__rapina/openapi.json` for the full spec.

## Roadmap

### v0.1 — Foundation (in progress)
- [x] Auth (email/password + JWT)
- [x] Projects CRUD
- [x] Testimonials CRUD
- [x] Tags system
- [x] Dashboard (Leptos CSR)
- [x] Railway deployment
- [ ] Collection forms (public submission pages)
- [ ] CSV import
- [ ] Wall of Love widget + embed system
- [ ] Approval workflow
- [ ] Docker Compose for self-hosting

### v0.2 — Forms & Collection
- [ ] Visual form builder
- [ ] Video testimonial upload (S3)
- [ ] Rating system
- [ ] Email notifications

### v0.3 — Widgets
- [ ] Widget templates (carousel, cards, popup, marquee, badge, minimal)
- [ ] Visual widget customizer
- [ ] Shadow DOM isolation

### v0.4 — Import & Sync
- [ ] Import connectors (Twitter/X, Google Reviews, Product Hunt)
- [ ] CSV import/export
- [ ] Auto-sync engine

### v0.5 — AI & Analytics
- [ ] Sentiment analysis
- [ ] Video transcription
- [ ] Analytics dashboard

### v0.6 — Scale
- [ ] OAuth (Google, GitHub)
- [ ] Multi-user / teams
- [ ] API keys
- [ ] White-label

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT — see [LICENSE](LICENSE).
