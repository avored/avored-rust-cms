# Avored, Rust content management system
AvoRed Rust CMS implemented with the help of [Axum](https://docs.rs/axum/latest/axum/) web framework and [SurrealDB](https://surrealdb.com/) as the database and a [React](https://react.dev/) admin frontend.


[![AvoRedCMS](https://github.com/avored/avored-rust-cms/actions/workflows/rust.yml/badge.svg)](https://github.com/avored/avored-rust-cms/actions/workflows/rust-test.yml)

### Join community via slack
[![Join community via Slack](https://img.shields.io/badge/Slack-4A154B?style=for-the-badge&logo=slack&logoColor=white)](https://join.slack.com/t/avoredrustcms/shared_invite/zt-22031l11y-EYp3a3oWVVFaZ8WCWZAkJQ)

## Demo admin

Please visit [Demo AvoRed Rust CMS Admin](https://demo-admin.avored.com)

## Installation

Make sure the prerequisite are installed and then go through the install steps to install a local build of Avored.

### Prerequisites

**Protobuf**, Google's data interchange format written in C++, is required to be installed before build.  Read more about it at the protocol buffers website: https://protobuf.dev/ or at the repository https://github.com/protocolbuffers/protobuf

* Install the precompiled binary on any OS

https://protobuf.dev/installation/

* Install Protobuf package on MacOS with `install protobuf`

**Rust**, the systems programing lanugage for backend development.  https://www.rust-lang.org/

**NodeJS**, the frontend web server and frontend toolchain. https://nodejs.org/en


### Install Steps
1. Clone the repository and create the local .env file from the `EXAMPLE.env`.

```bash
git clone https://github.com/avored/avored-rust-cms.git
cd avored-rust-cms
cp EXAMPLE.env .env.dev
ln -s .env.dev .env
```

2. Set up your `.env.dev` file by adding the random strings for the password salt and JWT secret.  The SMTP setup should be accurate if you wanted to use the forgot password feature(optional).

**NOTE:** For local dev mail/SMTP info can be ignored, there are other ways to recover your login locally.

3. Start your application backend

```bash
cargo run
```

4. Navigate to the admin web app directory `cd ts-grpc-react-admin` install the javascript dependencies and start the admin page.

```bash
npm i
npm run dev
```

5. Once the application start you can visit [http://localhost:3000/setup](http://localhost:3000/setup).  This url will create the base database tables and an admin user once you submit the form on that page.

6. After the form submit a redirect to login screen will occur [localhost:3000/admin](localhost:3000/admin).  Use the email and password you set with the setup form.

Finished!

You can now visit `localhost:3000/admin` when the backend and frontend are running to access the react admin for the portal.

**NOTE:** Right now react admin does not have many pages redone yet but work in progress.


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
