# rustsec.org

The https://rustsec.org web site

## Site Generator

To regenerate the web site using the latest advisories from the `advisory-db`,
run the following command (assuming you have Rust installed):

```
$ cargo run -p rustsec-website-gen
```

## Building Site

You will need Ruby and Bundler installed.

To install Jekyll (static site builder) and its dependencies, run:

```
$ bundle
```

To build the static site, run:

```
$ bundle exec jekyll build
```

To run a web server that allows you to modify the site and view the results
live after editing, run:

```
$ bundle exec jekyll serve
```
