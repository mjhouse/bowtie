# bowtie

Hey Patrick. 

So everything is broken right now, but you can look through the code while I 
figure it out if you want. Explanation of the breakage follows, then a project 
overview.

## Why it's broken

Basically, I decided early on to try and keep session info as a JSON web token 
(JWT) to reduce the number of database hits. Updating existing tokens is sort
of wonky in Rocket, apparently, and I can see having to seriously expand what
I keep in that cookie as I move forward- preference info, which "view"/"persona"
is active, etc. So I decided to scrap that and just hit the database for this
stuff. There will still be a cookie, but it will just be a basic session token
that has a unique hash and an expiration date.

So I started rewriting stuff left and right to pull the JWT stuff out, and in
the process realized I didn't have a particularly well standardize 
create/destroy/update methods for all of my models.

So I'm doing those two things, and I'm hoping that will be done in a week or so
if work permits.

## Project overview

### Root project

#### main.rs

So if you look under src/ all that you'll see right now is a main.rs file with
some commented code. That's the entry point for the whole thing. It passes all
of the routes to Rocket and starts the server.

#### templates

The templates/ directory has all the Tera templates in it, obviously. You can 
look through those if you want to get an idea of how it works and then get 
annoyed at the complete lack of any client-side state/javascript lol. Even the
css is pretty minimal, actually. It's divided up into layout css and a theme.css
file. I'm planning to load different css files for different themes at some point.

#### static

This directory has static files- mostly css, and one js file that has a very
minimally functional wysiwyg editor that just does bold and italics. I plan to
expand it later. There are fonts in there, but I'm not using them. A stack of
system fonts is not that bad looking and it's basically free latency-wise.

#### docs

Has a file with some notes about building on windows. I started this project on
Ubuntu.

### Crates

This is the directory where all the backend code lives. It's split up into three
projects. The directories are data, models and routes, but in all the config files
and import statements they're called bowtie_data, bowtie_models and bowtie_routes.

#### bowtie_data

This project has all the database-interaction stuff in it. The database migrations
are here, and any time you need to run diesel_cli for some reason, you probably need
to run it from this directory. It generates a src/schema.rs file that defines the
database for the project. There's a traits.rs file that I haven't put anything in yet,
and a really big macro in models.rs file that builds database models. If you want
to see it in use you can look in the bowtie_models project. 

This project should ideally only be used by the bowtie_models project- routes/main
etc. should never have to use any of the construction macros or the schema directly.

#### bowtie_models

All of the database models and most of the public facing (bowtie_routes-facing, I 
should say) objects live here. Users, Views, Posts and probably Session models at
some point, basically.

#### bowtie_routes

All of the routes listed in main.rs are from this directory. Right now I have them
organized by the section of the site that they route for- so profile.rs handles all
the logged-in user stuff, auth.rs handles all of the login/logout stuff and public
is all the pages that are publicly available, although they will render differently
to users who are logged in, obviously (menus etc.)

### Done

That's it. Basically request flow goes route -> model-action -> database and then back
up until the route renders a response. The main focus' (focii?) of the site are:

* Speed: And I mean instantaneous speed. Slow websites really piss me off, and I want
this one to be blindingly fast. I think if I use many small, lightweight pages, then
I can get this sort of speed- and the speed will give the impression that only part of
the page is changing -> boom, web-app baby.

* Reliability: Obviously a little hit-or-miss right now, but ideally failures that happen
because of the site itself will be few and far between.

* Generic code: I like a well-organized, well-structured codebase. Mainly. But also I
think relying on macros and common patterns for things (like my model design, which is
going to be damn-near identical whether you're interacting with a Post or a User struct)
will make things easy to reason about, and easy to extend.

Anyway- I'm going to commit this and add you as a contributor. See you at work.
-Mike