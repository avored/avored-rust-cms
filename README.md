# Avored rust content management system
AvoRed Rust CMS implement with the help of axum web framework and surrealdb as database.


[![AvoRedCMS](https://github.com/avored/avored-rust-cms/actions/workflows/rust.yml/badge.svg)](https://github.com/avored/avored-rust-cms/actions/workflows/rust-test.yml)

### Join community via slack
[![Join community via Slack](https://img.shields.io/badge/Slack-4A154B?style=for-the-badge&logo=slack&logoColor=white)](https://join.slack.com/t/avoredrustcms/shared_invite/zt-22031l11y-EYp3a3oWVVFaZ8WCWZAkJQ)

## Demo admin

Please visit [Demo AvoRed Rust CMS Admin](https://demo-admin.avored.com)

## Installation

*Steps*
1. Clone the repository and create the local .env file from the `EXAMPLE.env`.

```bash
git clone https://github.com/avored/avored-rust-cms.git
cd avored-rust-cms
cp EXAMPLE.env .env.dev
```

2. Set up your `.env.dev` file by adding the random strings for the password salt and JWT secret.  The SMTP setup should be accurate if you wanted to use the forgot password feature(optional).

**NOTE:** For local dev mail/SMTP info can be ignored, there are other ways to recover your login locally.

3. Start your application backend

```bash
cargo run
```

4. Once the application start you can visit `http://localhost:3000/setup` this url will create database tables and admin user once you submit it and redirect to login screen but ignore this page as we got react admin setup in progress.

### Start react application admin

```bash
cd react-admin
npm i
npm start
```

Visit `localhost:3000/admin` to access the react admin for the portal. Right now react admin does not have many pages redone yet but work in progress.


## Features

- [x] Admin Users
- [x] Roles/Permissions
- [x] Components
- [x] Pages
- [x] Fields
- [x] Asset Manager

## RoadMap
 - [ ] Rest API
 - [ ] GraphQL API
 - [ ] Content Workflow

##### Setting up full Stack project in RUST

 - [x] Web Framework ([Axum](https://github.com/tokio-rs/axum))
 - [x] SurrealDB ([Surreal DB](https://surrealdb.com/))
 - [x] Email Views ([Handlebars](https://github.com/sunng87/handlebars-rust))
 - [x] Display Form Validation Error

### Dev help

How to do a loop inside the handlebar template

    {{#each validation_message as |message|}}
        {{ message.mssage }}
    {{/each}}

How to render a variable in handlebar template

    {{ variable_name }}

How to call helper method in handlebar template

    {{ helper_method_name "argument" ~}}

How to start a surreal db
surreal start --user root --pass root --bind 0.0.0.0:8000 file://test.db

#### Introduction

Avored headless CMS enables efficient content management for websites, mobile apps, and various digital platforms.
By decoupling the content from the presentation layer, it gives developers the flexibility to build frontends
using their preferred technologies. With an avored headless CMS, content is centralized, making it easy to
create, manage, and seamlessly deliver it to multiple applications.

Discover more about Headless CMS concepts.

###### Features
 - **Comprehensive Asset Management**: Organize and manage images, media, files, and other assets with ease.
 - **User-Friendly Content Management**: Simplify page content management for both technical and non-technical users.
 - **Seamless Content Distribution**: Deliver your content anywhere instantly using REST APIs.
