# Unnamed combination URL shortener/pastebin

This is a combination URL shortener/pastebin written in Rust and [actix-web](https://github.com/actix/actix-web). It supports connecting to any mySQL server.

## Warning

I would not recommend using this in production. It was originally just a project to acquaint me with rust and actix-web. Large portions were written hastily with the help of Copilot, and thus, are probably not best-in-class when it comes to software design.

### Things I'd have to do to make this usable

* Proper error handling (for example, showing the user a "unable to access database" error through HTTP)
* Proper static 404 support
* Proper logging (instead of println!s placed while debugging that I forgot to remove)
* Remodeling the Count struct as an option

## Required files

### config.toml

A fully populated config.toml is required to run the server. The parameters that are default to actix-web are explained in the config.

The parameters specific to the url shortener are:

* `[application.html]` → Things related to HTML and templating
    * `template` → Whether to template at all or not
    * `template_index` → Whether to template the index.html (/)
    * `template_static` → Whether to template static html (this is sometimes needed as the submission form is often on a static page, as it is in this repo's example html)
    * `path` → the path that html will be read from. Might want to make this /var/www/url_shortener, for instance.
    * `static_path` → the subdirectory of `path` where static is stored. Unlike the above, this is actually displayed as part of the url.
    * `domain` → The domain you're hosting from, used for templating
* `[application.database]` → Things related to connecting to the mySQL database
    * `host`
    * `port` → Usually 3306.
    * `username`
    * `password`
    * `database`

### HTML files

In your HTML directory, these files are required

* `index.html`
* `url.html` → The page shown after a URL is submitted
* `paste.html` → The page shown after a paste is submitted

## Forms

Submitting a URL/paste is done through a POST request.

Here are the fields necessary:

* `content` → this is the URL to be shortened or the paste to be submitted
* `content_type` → this is either `Pastebin` or `Url`, make this a hidden value on form

## Templating

This app has its own inbuilt templating engine. It's extremely simple; include these within your HTML and they will get replaced.

* `{content}` → The original URL or the paste-text
* `{shortened}` → The shortened suffix (yourdomain.com/*)
* `{domain}` → Your domain, as set in the config.toml
* `{count:0}` → The total number of URLs shortened
* `{count:1}` → The total number of pastes submitted
* `{count:2}` → The total number of all entries

