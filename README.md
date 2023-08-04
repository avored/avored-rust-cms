# Avored Rust CMS
AvoRed Rust CMS implement with the help of axum web framework and surrealdb as database. 


[![AvoRedCMS](https://github.com/avored/avored-rust-cms/actions/workflows/rust.yml/badge.svg)](https://github.com/avored/avored-rust-cms/actions/workflows/rust.yml)


## Installation

    git clone https://github.com/avored/avored-cms
    cd avored-cms/rust-api
    

Setup your `.env` file you can rename the existing `.env.example` file make sure the database username and password setup correctly.

### Start your application     
    cargo run


## Features

- [ ] Users
- [ ] Roles/Permissions
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
 - [x] DB ([Postgres SQL](https://www.postgresql.org/))
 - [x] DB ORM ([Sea Orm CLI](https://www.sea-ql.org))
 - [x] VIEW ([Handlebars](https://github.com/sunng87/handlebars-rust))
 - [x] Form Validation ([Validator](https://github.com/Keats/validator))
 - [x] Display Form Validation Error
 - [ ] Todo Flash messages




### Dev Help 


Generate Sea Orm Cli Entity

    sea-orm-cli generate entity -o entity/src

Generate Sea Orm Migration Fresh

     sea-orm-cli migrate fresh


How to do a loop inside the handlebar template

    {{#each validation_message as |message|}}
        {{ message.mssage }}
    {{/each}}

How to render a variable in handlebar template 

    {{ variable_name }}

How to call heloper method in handlebar template 

    {{ helper_method_name "argument" ~}}
