# Avored rust content management system
AvoRed Rust CMS implement with the help of axum web framework and surrealdb as database.


[![AvoRedCMS](https://github.com/avored/avored-rust-cms/actions/workflows/rust-test.yml/badge.svg)](https://github.com/avored/avored-rust-cms/actions/workflows/rust-test.yml)

### Join community via slack
[![Join community via Slack](https://img.shields.io/badge/Slack-4A154B?style=for-the-badge&logo=slack&logoColor=white)](https://join.slack.com/t/avoredrustcms/shared_invite/zt-22031l11y-EYp3a3oWVVFaZ8WCWZAkJQ)

## Demo admin

Please visit [Demo AvoRed Rust CMS Admin](https://demo.avored.com/admin)

## Installation

    git clone https://github.com/avored/avored-rust-cms.git
    cd avored-cms
    

Set up your `.env` file you can rename the existing `.env.example` file make sure the database username and password setup correctly.

### Start your application backend
    cargo run

Once the application start you can visit `http://localhost:8080/setup` this url will create database tables and admin user once you submit it and redirect to login screen but igonore this page as we got react admin setup in progress.

### Start react application admin
    cd react-admin
    npm i
    npm start

Visit `localhost:3000/admin` to access the react admin for the portal. Right now react admin does not have many pages redone yet but work in progress.


## Features

- [x] Admin Users
- [x] Roles/Permissions
- [ ] Components
- [ ] Pages
- [ ] Collection
- [ ] SEO
- [ ] Asset Manager

## RoadMap
 - [ ] Rest API
 - [ ] GraphQL API
 - [ ] Content Workflow

##### Seting up full Stack project in RUST

 - [x] Web Framework ([Axum](https://github.com/tokio-rs/axum))
 - [x] SurrealDB ([Surreal DB](https://surrealdb.com/))
 - [x] VIEW ([Handlebars](https://github.com/sunng87/handlebars-rust))
 - [x] Form Validation ([Validator](https://github.com/Keats/validator))
 - [x] Display Form Validation Error
 - [x] Flash messages
 - [ ] Form validation middleware
 
### Dev help 

How to do a loop inside the handlebar template

    {{#each validation_message as |message|}}
        {{ message.mssage }}
    {{/each}}

How to render a variable in handlebar template 

    {{ variable_name }}

How to call heloper method in handlebar template 

    {{ helper_method_name "argument" ~}}


How to start a surrealdb
surreal start --user root --pass root --bind 0.0.0.0:8000 file://test.db
