# Avored Rust CMS
AvoRed Rust CMS implement with the help of axum web framework and diesel orm. 


[![AvoRedCMS](https://github.com/avored/avored-rust-cms/actions/workflows/rust.yml/badge.svg)](https://github.com/avored/avored-rust-cms/actions/workflows/rust.yml)


## Installation

    git clone https://github.com/avored/avored-cms
    cd avored-cms/rust-api
    

Setup your `.env` file you can rename the existing `.env.example` file make sure the database username and password setup correctly.

     
    // Install Sea ORM  CLI
    cargo install sea-orm-cli

    sea-orm-cli migrate up

    // Now we will execute the cargo run

    cargo run

##### Seting up full Stack project in RUST

 - [ ] Web Framework ([Axum](https://github.com/tokio-rs/axum))
 - [ ] DB ([Postgres SQL](https://www.postgresql.org/))
 - [ ] DB ORM ([Sea Orm CLI](https://www.sea-ql.org))
 - [ ] VIEW ([Handlebars](https://github.com/sunng87/handlebars-rust))
 - [ ] Form Validation ([Validator](https://github.com/Keats/validator))
 - [ ] Todo Display Form Validation Error
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
