Here is a clean, comprehensive, and professional `README.md` tailored precisely to your backend stack and frontend asset configuration.

---

# Omoide Tenji 

Omoide Tenji is a high-performance, minimalist digital gallery platform built with **Rust** and **React**. The application features an asynchronous backend using **Axum** and **Tokio** to seamlessly process image multi-part uploads, interface directly with **AWS S3 / LocalStack** for scalable asset hosting, and compile assets on-the-fly into structured `.zip` data downloads.

---

## 🚀 Key Features

* **Instant Gallery Generation**: Generate unique, UUID-v4 authenticated galleries dynamically.
* **Streamed Multipart Uploads**: Upload multiple images (`png`, `gif`, `webp`, `jpg`) safely capped by a server-side payload safety filter (up to 200MB).
* **S3 Integration**: Native cloud asset integration with fallback environments for local development storage (LocalStack/MinIO).
* **On-the-Fly Dynamic Zipping**: Bundle cloud gallery components into compressed downloadable `.zip` streams in memory without writing temp files to disk.
* **Single-Page Application (SPA) Serving**: Serves compiled frontend assets directly from production-optimized Axum routes with built-in client-side fallback matching.

---

## 🛠️ System Prerequisites

Ensure you have the following system dependencies installed globally before compiling:

* **Rust Toolchain**: `Rustc & Cargo (Edition 2021)` -> [Install Rust](https://rustup.rs/)
* **Node.js**: `v18.x` or above (along with `npm` or `yarn`) -> [Install Node.js](https://nodejs.org/)
* **AWS Credentials / Local Environment**: Access to an active AWS S3 bucket or a running LocalStack wrapper instance.

---

## 🗺️ Project Architecture Overview

```text
├── Cargo.toml            # Backend manifest and dependencies (Axum, AWS-SDK, Tokio)
├── src/
│   ├── main.rs           # Runtime initialization, environment mapping, TCP lifecycle
│   ├── routes.rs         # Api routers, static asset binding, and SPA fallback layers
│   ├── handler.rs        # Core operational endpoints for business logical tracking
│   ├── file_ops.rs       # Stream processing logic (Multipart ingestion & dynamic archiving)
│   └── s3.rs             # Client abstract interaction with Object Stores
├── static/               # Production build home for static files & client assets
└── frontend/             # Single Page Application source workspace (Vite + React 19)

```

---

## 🔌 API Documentation Matrix

| Method | Endpoint | Description | Payload Constraints / Type |
| --- | --- | --- | --- |
| `GET` | `/api/health` | Service health status ping verification | Returns `{ "status": "ok" }` |
| `POST` | `/api/gallery` | Creates an authenticated gallery space | Returns generated UUID mapping links |
| `GET` | `/api/gallery/:id` | Lists all object URI pointers in gallery | Resolves public URLs for target display client |
| `POST` | `/api/gallery/:id/upload` | Ingests structural multipart file list safely | Multi-part form context (`image` key fields) |
| `GET` | `/api/gallery/:id/download` | Bundles and drops direct platform archive stream | Generates application/zip binary format download |

---

## ⚙️ Environment Configuration

Create a `.env` file inside the root repository directory to guide operational routing:

```env
# Server Network Settings
PORT=5000

# AWS/S3 Storage Configuration
AWS_REGION=eu-north-1
S3_BUCKET=your-target-bucket-name

# AWS Authorization Variables (Needed if executing outside IAM Roles)
AWS_ACCESS_KEY_ID=your_access_key
AWS_SECRET_ACCESS_KEY=your_secret_key

# Optional: Local Cloud Mock Optimization (LocalStack/MinIO)
# AWS_ENDPOINT_URL=http://localhost:4566

```

---

## 🏃 Execution & Local Development Setup

### 1. Build and Link the Frontend

Navigate into the workspace client directory, install modern dependencies, and export distribution targets to the static backend path:

```bash
# Shift into the frontend workspace
cd frontend

# Install package footprints
npm install

# Build static production assets inside `static/` directory
npm run build

```

### 2. Launching the Backend Server

Return to the root system path, bind target properties, and start the cargo pipeline loop:

```bash
# Return to the root folder context
cd ..

# Run the rust development ecosystem
cargo run

```

The system will report operational binding loops via standard output streams:

```bash
Server running on port http://localhost:5000

```

---

## 🧪 Quick Test Framework Execution

```
