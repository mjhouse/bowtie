# bowtie

This is the repository for the bowtie social media platform. It's a social media
website built from the ground up around three principles:

* __Be reliable__. This site, post-1.0 should very rarely or never fail. If it does
  it should fail gracefully with error messages and directions for the affected users.

* __Be accessible__. Each page should be as static as possible so that screenreaders
  can parse them, and use semantic html with appropriate hinting and attributes so that
  disabled users can navigate.

* __Be as fast as humanly possible__. This means no huge javascript bundle, no
  front-end framework and few database queries per page. In return for these self-imposed 
  constraints, pages load in less than 50ms. Often much less.

I can make it reliable by using a reliable language (in this case, Rust) for the back end,
and avoiding unnecessary javascript, hacky css or dynamic anything. Once compiled, it should
run the same the thousandth time as it did the first. Part of targeting reliability is using
simple code, which also allows me to make the site accessible to everyone.

If the site is reliable, compiled and simply designed, it will be much easier to make it 
fast. Currently, on pages without database queries, load time is less than 10ms- with queries
it's closer to 50ms. It's a work in progress.

## Project overview

### Root project

#### src/

The main.rs file is the entry point for the whole site. 
It passes all of the routes to Rocket and starts the server.

#### resources/

Contains all templates, css, js, images and fonts. It's expected that js/css for
a page will be at an identical relative path (i.e. html at 'resources/profile/write.html'
have a stylesheet at 'resources/css/write.scss' and scripts in 'resources/profile/write.js').

#### docs/

Has generated documentation for the project.

### Crates

This is the directory where all the backend code lives. It's split up into three
projects. The directories are data, models and routes, but in all the config files
and import statements they're called bowtie_data, bowtie_models and bowtie_routes.

#### /crates/data/

This project has all the database-interaction stuff in it. The database migrations
are here, and any time you need to run diesel_cli for some reason, you probably need
to run it from this directory. It generates a src/schema.rs file that defines the
database for the project.

#### /crates/models/

All of the database models and most of the public facing (bowtie_routes-facing, I 
should say) objects live here. Users, Views, Posts etc.

#### /crates/routes/

All of the routes listed in main.rs are from this directory. Right now I have them
organized by the section of the site that they route for- so profile.rs handles all
the logged-in user stuff, auth.rs handles all of the login/logout stuff and public
is all the pages that are publicly available, although they will render differently
to users who are logged in, obviously (menus etc.)

The most important file in this project that isn't a route is resources.rs- it manages
all the html, css and js content that is loaded when the server starts and combines
them on the fly when they're requested.

### Setup

#### Windows 10

##### Set the following environment variables
PQ_LIB_DIR  = C:\Program Files\PostgreSQL\12\lib
OPENSSL_DIR = C:\Program Files\OpenSSL-Win64

##### Run
cargo install diesel_cli --no-default-features --features postgres

### Done

That's it. Basically request flow goes route -> model-action -> database and then back
up until the route renders a response.