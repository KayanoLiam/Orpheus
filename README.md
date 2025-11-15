# Orpheus

[中文版本](README.zh.md) | English Version

## Project Overview

Orpheus is a Supabase-like full-stack application system built on the Rust ecosystem, designed to provide ready-to-use identity authentication, session management, and data access capabilities for web applications. The project uses Actix-Web as the core web framework, PostgreSQL as the primary data storage, and Redis for session storage and caching acceleration. The frontend is built with Next.js 16 + React 19 + TypeScript, providing a modern user interface.

The goal of this project is to build a scalable, self-hostable full-stack application infrastructure with good security and performance.

## Core Features

### User System
- User registration, login, logout
- Password reset functionality
- User deletion functionality
- Secure password hashing and verification (based on Argon2)
- Bearer Token authentication mechanism

### Session Management
- Redis-based session storage for improved performance and recoverability
- Automatic session expiration and refresh mechanisms
- Middleware-based unified authentication

### Database Layer
- PostgreSQL as the primary database
- Unified connection pool and error handling abstraction
- Type-safe database operations (SQLx)

### API Design
- RESTful-style interfaces
- Structured JSON response format
- Clear error and status code specifications
- GitHub API integration example

### Frontend Interface
- Modern React 19 + Next.js 16 application
- Responsive design supporting mobile devices
- TypeScript type safety
- Tailwind CSS modern styling
- Component-based architecture

### Code Quality Assurance
- Strict type and error handling strategies
- Complete test suite (unit tests + integration tests)
- Modular architecture for easy development and maintenance
- Structured logging

## Technology Stack

### Backend
| Component | Version | Description |
|-----------|---------|-------------|
| Rust | 2021 Edition | Main language emphasizing safety and performance |
| Actix-Web | 4.x | High-performance web framework |
| PostgreSQL | - | Primary data storage (relational database) |
| Redis | 0.23.1 | Session / cache storage |
| Argon2 | 0.5 | Password hashing algorithm |
| SQLx | 0.8.6 | Type-safe database operations |
| Tokio | 1.x | Async runtime |
| Tracing | 0.1 | Structured logging |

### Frontend
| Component | Version | Description |
|-----------|---------|-------------|
| Next.js | 16.0.3 | React full-stack framework |
| React | 19.2.0 | UI library |
| TypeScript | 5 | Type-safe JavaScript |
| Tailwind CSS | 4 | Modern CSS framework |
| Radix UI | - | Headless UI component library |
| Lucide React | 0.553.0 | Modern icon library |
| Axios | 1.13.2 | HTTP client |

## Project Structure

```
orpheus/
├── src/                    # Backend source code
│   ├── main.rs             # Application entry point
│   ├── config.rs           # Configuration constants
│   ├── auth/               # Authentication module
│   │   └── session_store.rs # Redis session management
│   ├── handlers/           # HTTP request handlers
│   │   ├── user_handler.rs # User-related interfaces
│   │   ├── session_handler.rs # Session-related interfaces
│   │   └── github_handler.rs # GitHub API integration
│   ├── middlewares/        # Middlewares
│   │   └── session.rs      # Session validation middleware
│   └── models/             # Data models
│       ├── user.rs         # User model
│       ├── session.rs      # Session model
│       └── response.rs     # API response model
├── frontend/               # Frontend source code
│   ├── app/                # Next.js application directory
│   │   ├── layout.tsx      # Application layout
│   │   ├── page.tsx        # Main page
│   │   └── globals.css     # Global styles
│   ├── components/         # React components
│   │   └── ui/             # UI component library
│   ├── lib/                # Utility library
│   └── public/             # Static assets
├── tests/                  # Test suite
│   ├── user_handler_tests.rs
│   ├── session_handler_tests.rs
│   ├── session_store_tests.rs
│   └── session_middleware_tests.rs
├── docker-compose.yml      # Docker container orchestration
├── Dockerfile              # Backend Docker image build
├── .env                    # Environment variable configuration
├── Cargo.toml              # Rust project dependency configuration
└── README.md               # Project documentation
```

## Quick Start

### Environment Requirements
- Rust 1.70+
- Node.js 20+
- PostgreSQL 12+
- Redis 6+
- Docker & Docker Compose (optional)

### Installation Steps

1. **Clone the project**
   ```bash
   git clone https://github.com/KayanoLiam/Orpheus.git
   cd Orpheus
   ```

2. **Configure environment variables**
   Create a `.env` file and configure the following variables:
   ```bash
   DATABASE_URL=postgres://username:password@localhost:5432/database_name
   REDIS_URL=redis://localhost:6379
   ```

3. **Backend setup**
   ```bash
   # Build the project
   cargo build --release

   # Run backend service
   cargo run
   ```

4. **Frontend setup**
   ```bash
   # Enter frontend directory
   cd frontend

   # Install dependencies
   pnpm install

   # Run development server
   pnpm dev
   ```

   Services will start at the following addresses:
   - Backend API: `http://127.0.0.1:8080`
   - Frontend application: `http://localhost:3000`

### Docker Deployment

```bash
# Start all services using Docker Compose
docker-compose up -d --build

# Check service status
docker-compose ps

# Stop services
docker-compose down
```

## API Documentation

### Public Endpoints

#### User Registration
```http
POST /signup
Content-Type: application/json

{
  "username": "string",
  "email": "string",
  "password": "string"
}
```

#### User Login
```http
POST /login
Content-Type: application/json

{
  "email": "string",
  "password": "string"
}
```

#### Password Reset
```http
POST /reset-password
Content-Type: application/json

{
  "email": "string",
  "new_password": "string"
}
```

#### Delete User
```http
DELETE /delete-user
Content-Type: application/json

{
  "email": "string",
  "password": "string"
}
```

#### Get GitHub Repository Stars
```http
GET /github/stars/{owner}/{repo}
```

### Authenticated Endpoints

#### User Logout
```http
POST /logout
Authorization: Bearer <token>
```

#### Get User Information
```http
GET /api/profile
Authorization: Bearer <token>
```

## Development Guide

### Code Standards
- Strict security coding practices (disable unwrap, expect, panic, etc.)
- Use `anyhow` for unified error handling
- Prioritize async functions for improved concurrency
- Clear type annotations for enhanced code readability
- Use `tracing` for structured logging

### Backend Development
```bash
# Run all tests
cargo test

# Run specific test modules
cargo test user_handler

# Show test output
cargo test -- --nocapture

# Format code
cargo fmt

# Run Clippy checks
cargo clippy

# Check code (without building)
cargo check
```

### Frontend Development
```bash
cd frontend

# Install dependencies
pnpm install

# Run development server
pnpm dev

# Build production version
pnpm build

# Code checking
pnpm lint

# Type checking
pnpm type-check
```

## Deployment

### Production Build

#### Backend
```bash
cargo build --release
```

#### Frontend
```bash
cd frontend
pnpm build
pnpm start
```

### Docker Deployment

The project supports complete containerized deployment, including application, PostgreSQL, and Redis services.

#### Configuration Notes

**docker-compose.yml** configures the following services:
- **app**: Orpheus application service (port 8080)
- **postgres**: PostgreSQL database (port 5432)
- **redis**: Redis cache service (port 6379)

#### Architecture Compatibility

To ensure compatibility across different platforms, the Docker configuration uses `platform: linux/amd64` settings, ensuring smooth operation on Apple Silicon (M1/M2) and other architectures.

#### Troubleshooting

**Slow builds**: First builds may take considerable time; subsequent builds will leverage Docker cache for acceleration.

**Port conflicts**: If ports are occupied, you can modify the port mappings in `docker-compose.yml`.

## Contributing

1. Fork the project
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Create a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

## Contact

- Project link: [https://github.com/KayanoLiam/Orpheus](https://github.com/KayanoLiam/Orpheus)
- Issue feedback: [Issues](https://github.com/KayanoLiam/Orpheus/issues)

## Acknowledgments

Thanks to friends who provided support during the development of this project. Your help (whether material support or spiritual encouragement) has enabled me to continuously invest time and energy in advancing the project. Here are some contributors to express sincere gratitude:

| Name/Nickname | Support Content | Notes |
|---------------|-----------------|-------|
| Ran Tiantian   | Financial support 3 USD | Provided actual support during early development |

> Support types are not limited to, but include: funding, hardware, testing assistance, promotion, suggestions, patient listening, and continuous encouragement.

---

**Note**: This is a learning project aimed at demonstrating the application of Rust in backend development and the practice of modern frontend technologies. Do not use in production environments without sufficient testing.