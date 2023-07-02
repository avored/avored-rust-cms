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

### Dev CLI
