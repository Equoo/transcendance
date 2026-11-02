*This project has been created as part of the 42 curriculum by pjarnac, dernst, dderny, tjooris.*

# KeepGrouped

## Description

KeepGrouped is a web platform to organize events and keep their participants together in one place. Each event gets its own chat channel, shared files and an AI assistant that answers questions from the event documents.

Key features:

- Event creation, registration and calendar
- Real-time chat (channels, categories, direct messages, read state)
- Friends, blocking and user profiles with avatars
- File upload and management (S3-compatible storage)
- AI chatbot with RAG over uploaded documents and events.
- Roles, permissions and an admin panel
- Interface in English, French and Chinese

## Instructions

Prerequisites:

- Git
- Docker Engine 29.3 or newer
- Docker Compose v5.1 or newer
- OpenSSL, to generate the secrets
- A machine with at least 4 GB of RAM, or a GPU with 4 GB of VRAM, for the LLM

1. Clone this repository.
2. Create the environment file and fill every value in it: port, server name, database password, Garage access/secret/RPC keys, admin login, email and password, JWT key. The comments in `.env.example` give the command to generate each secret.

   ```bash
   cp .env.example .env
   ```

3. Start everything:

   ```bash
   docker compose up -d
   ```

4. Open <https://localhost:4242> (or the `PORT` set in `.env`) and log in with the default admin account. The TLS certificate is self-signed, so the browser shows a warning that you have to accept.

The source code of each service lives in its own repository: `frontend`, `backend`, `backend-ia`, `keepgrouped-llama.cpp`.

## Team Information

| Login | Role(s) | Responsibilities |
| --- | --- | --- |
| pjarnac | Product Owner & Developper | Events, file storage, AI integration in the backend, infrastructure and CI, invitations |
| dernst | Project Manager & Developper | Authentication, users, roles, admin panel, multi-language|
| dderny | Technical Lead & Developper | Chat, channels, friends and relationships |
| tjooris | Developper | AI service (RAG pipeline, chatbot) |

## Project Management

- Organization: Work dispatched by big features. PR Reviews
- Tools: Forgejo (Git hosting, CI, container registry, issues).
- Communication: Discord on a daily basis.

## Technical Stack

| Layer | Technologies |
| --- | --- |
| Frontend | React 19, TypeScript, React Router, Vite, Tailwind CSS, TanStack Query, Zustand, i18next, SignalR |
| Backend | ASP.NET Core (.NET 11), Entity Framework Core, SignalR, JWT auth |
| AI service | Rust, Axum, Rig, FastEmbed, ChromaDB |
| LLM | llama.cpp server running Gemma 4 E2B (quantized) |
| Database | PostgreSQL |
| Storage | Garage (S3-compatible) |

Justification:

- PostgreSQL: relational data (users, events, channels, roles) with many relations and constraints; first-class EF Core support.
- ASP.NET Core + SignalR: typed, fast backend with built-in real-time transport.
- Rust for the AI service: low memory footprint and good async support for document parsing and streaming responses.
- Local LLM with llama.cpp: no external API, data stays on the server.
- Garage: lightweight self-hosted S3 storage for files and avatars.

## Database Schema

Main tables (PostgreSQL, managed by EF Core migrations in `backend/Migrations`):

| Table | Key fields | Relations |
| --- | --- | --- |
| Users | Id (string), UserName, Email, PasswordHash, Language (enum), IsOnline (bool) | Role (n-1), Avatar file (1-1) |
| Roles | Id, Name, Ranking, Permission (flags) | Users (1-n) |
| Events | Id, Name, Date, Size (int), Location, Tags (string[]), Description | Organizer (n-1), Channel (1-1), Users via Registrations (n-n), EventRoles, Files |
| EventRoles | Id, Name | Events (n-n) |
| Channels | Id, Type (enum), Name, Topic, Order | Category (n-1), Event (1-1), Members (1-n), RolesWhitelist (1-n) |
| ChannelCategories | Id, Name, Order | Channels (1-n), RolesWhitelist (1-n) |
| ChannelMembers, ChannelAcks | composite key (ChannelId, UserId) | Membership and last read message |
| ChannelRoles | ChannelId or CategoryId, RoleId | Role whitelist for channels and categories |
| Messages | Id, Content (max 8192), SentAt, MessageRefId | Channel (n-1), Sender (n-1), replied message |
| Relationships | MeId, UserId, Type (enum), Nickname, Since | Friends, pending requests and blocks |
| Files | Key, Name, Length, ContentType, ETag | Creator (n-1), Events (n-n) |
| Invitations, RefreshTokens | Id, ExpireAt, Usages | Platform invitations and sessions |

## Features List

| Feature | Description | Member(s) |
| --- | --- | --- |
| Authentication | Register, login, JWT + refresh tokens, sessions list | dernst |
| Users and profiles | Profile, avatar, language, password change | dernst |
| Admin panel | Manage users, roles and permissions | dernst |
| Invitations | Invite users to the platform | pjarnac |
| Events | Create, edit, register, calendar, event roles | pjarnac |
| Chat | Channels, categories, DMs, replies, read state, real time | dderny |
| Friends | Friend requests, blocking, online status | dderny |
| Files | Upload, rename, download, delete | pjarnac |
| Activity Status | Automatic update with browser activity, real time. | pjarnac
| AI assistant | Chatbot answering from event documents (RAG) | tjooris, pjarnac |
| Translations | English, French, Chinese | dernst |

## Modules

| Module | Type | Points | Implementation | Member(s) |
| --- | --- | --- | --- | --- |
| Framework for frontend and backend | Major | 2 | React + ASP.NET Core | all |
| Real-time features | Major | 2 | SignalR hub for chat and activity | dderny, pjarnac |
| User interaction | Major | 2 | Chat, profiles, friends | dderny, dernst |
| Standard user management | Major | 2 | Auth, profiles, avatars | dernst, pjarnac |
| Advanced permissions system | Major | 2 | Roles, channel whitelists, admin panel | dernst |
| RAG system | Major | 2 | Rust service, FastEmbed + ChromaDB | tjooris |
| LLM system interface | Major | 2 | Streaming chatbot on llama.cpp | tjooris, pjarnac |
| ORM | Minor | 1 | Entity Framework Core | all |
| File upload and management | Minor | 1 | Garage S3 storage | pjarnac |
| Multiple languages | Minor | 1 | i18next, 3 languages | dernst |
| Custom-made design system | Minor | 1 | Buttons, various inputs, modals, badges, list, etc.. | dernst, pjarnac, dderny
| Support for additional browsers | Minor | 1 | Firefox, opera, chrome | all
| **Total** | | **19** | | |

## Individual Contributions

- **pjarnac**: events (backend and frontend), file storage with Garage, AI backend integration and RAG queue, llama.cpp image, Docker Compose, CI. Challenges: Creating a easily usable dev env with devcontainers was really hard, file management with the horrible Amazon S3 Documentation.
- **dernst**: authentication with JWT, users, roles, multi-languages, permissions, invitations, admin panel. Challenges: Did you know that handling permissions is definitely not easy? One tiny mistake and a simple member can become an admin. Add token management on top of that, and you have the reason behind my 4 months of nightmare.
- **dderny**: chat backend and UI, channels, friends and relationships. Challenges: Did you know that replicate Discord it's a huge work ? The more challenging was database structure and messages paginations.
- **tjooris**: Rust AI service, document parsing, embeddings, chatbot. Challenges: Learning Rust was tough, but ingesting files using multipart was by far the most challenging part.

## Resources

- [ASP.NET Core](https://learn.microsoft.com/aspnet/core), [EF Core](https://learn.microsoft.com/ef/core), [SignalR](https://learn.microsoft.com/aspnet/core/signalr)
- [React](https://react.dev), [React Router](https://reactrouter.com), [TanStack Query](https://tanstack.com/query)
- [Rig](https://docs.rig.rs), [ChromaDB](https://docs.trychroma.com), [llama.cpp](https://github.com/ggml-org/llama.cpp)
- [PostgreSQL](https://www.postgresql.org/docs/), [Garage](https://garagehq.deuxfleurs.fr/documentation/)

AI usage: Used for late redesign of certains frontend components (Modals, Inputs, Buttons). Big scale refactor and file moves. Research. Bug finding. A part of this README.
