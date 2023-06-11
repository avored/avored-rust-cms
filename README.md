# Avored Rust CMS
AvoRed Rust CMS implement with the help of axum web framework and diesel orm. 


[![AvoRedCMS](https://github.com/avored/avored-rust-cms/actions/workflows/rust.yml/badge.svg)](https://github.com/avored/avored-rust-cms/actions/workflows/rust.yml)


## Installation

    git clone https://github.com/avored/avored-cms
    cd avored-cms/rust-api
    

Setup your `.env` file you can rename the existing `.env.example` file make sure the database username and password setup correctly.

     
    // Install Diesel CLI
    cargo install diesel_cli --no-default-features --features postgres

    diesel setup
    diesel migration run

    // Now we will execute the cargo run

    cargo run

### Dev CLI

Tailwind CLI
     
    tailwindcss -i resouces/css/input.css -o static/css/app.css --watch



### CREDIT

Hexagonal architech pattern concept credit goes to `https://github.com/antoinecarton/hexagonal-rust`
