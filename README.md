# rustsec.org

[![Build Status][build-image]][build-link]

The <https://rustsec.org> web site

## Site Generator

The site is automatically rebuilt for every commit to the `advisory-db` repo
using the [publish-web] GitHub Actions workflow.

Site generation is handled by the [rustsec-admin] utility, which is intended to
be run on a local copy of the `gh-pages` branch of this repo:

    $ rustsec-admin web

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

[build-image]: https://github.com/RustSec/advisory-db/workflows/Publish%20Web/badge.svg
[build-link]: https://github.com/RustSec/advisory-db/actions?query=workflow:Publish+Web
[publish-web]: https://github.com/RustSec/advisory-db/blob/master/.github/workflows/publish-web.yml
[rustsec-admin]: https://github.com/RustSec/rustsec-admin
