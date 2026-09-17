# Changelog

All notable changes to this project will be documented in this file.

## [0.6.0] - 2024-10-20

### Features

- *(backend)* Replace redis with sled

### Miscellaneous Tasks

- Update GitHub actions
- Add multiplatform support in CI
- Fix typo
- Update dependencies & docs
- Bump version and enable pedantic clippy

## [0.5.0] - 2024-04-01

### Features

- *(frontend)* Add dark theme

### Documentation

- Add env variables to dev section in README.md

### Miscellaneous Tasks

- *(backend)* Update rust and dependencies
- *(frontend)* Update dependencies
- *(frontend)* Update node version
- *(backend)* Update rust and dependencies
- *(frontend)* Update node and dependencies
- *(backend)* Update rust dependencies
- *(backend)* Update rust and dependencies
- *(frontend)* Update node and dependencies
- *(frontend)* Format files
- *(backend)* Fix issue with env import in Sveltekit2
- Release 0.5.0

## [0.4.0] - 2023-08-02

### Features

- *(frontend)* Add status overview counters

### Miscellaneous Tasks

- Add changelog
- Fix an issue with the docker-compose template
- Bump version code

## [0.3.0] - 2023-08-01

### Features

- *(frontend)* Auto-refresh status
- Add support for different watchpoints
- *(frontend)* Open links in new tab
- *(backend)* Add keyword status check kind
- *(frontend)* Add display for keywords and new status codes

### Bug Fixes

- *(frontend)* Fix build error when API is absent

### Documentation

- Update README.md

### Miscellaneous Tasks

- *(frontend)* Correct formatting
- *(frontend)* Change editor settings
- *(frontend)* Update node version
- *(backend)* Bump rust version
- *(backend)* Update dependencies
- *(frontend)* Update dependencies and migrate to svelte 4
- *(frontend)* Format files
- *(backend)* Move part of the config values into environment variables as default

### Continuous Delivery

- Add automatic linting
- Change ignore paths for docker job

### Other

- *(frontend)* Make page name configurable

## [0.2.2] - 2023-07-05

### Miscellaneous Tasks

- *(backend)* Update rust, cleanup dependencies

## [0.2.1] - 2023-06-27

### Miscellaneous Tasks

- *(frontend)* Update node version

### Other

- *(frontend)* Improve footer and links
- *(frontend)* Make layout more mobile friendly

## [0.2.0] - 2023-06-26

### Features

- *(frontend)* Better error messages

### Miscellaneous Tasks

- Fix default status page settings to make it usable out-of-the-box
- *(backend)* Update dependencies
- *(frontend)* Update dependencies

### Continuous Delivery

- Add docker image workflow for master branch
- Publish images automatically to ghcr.io
- Fix image name
- Fix registry

## [0.1.0] - 2023-04-18

### Features

- *(frontend)* Improve styling
- *(backend)* Parallelize watchpoints
- *(backend)* Add logging
- *(backend)* Add static file hosting
- *(backend)* Handle more errors

### Documentation

- Add README & LICENSE
- Add usage instructions to README
- Add license notice to README

### Miscellaneous Tasks

- *(frontend)* Cleanup README and svelte config
- *(frontend)* Add .idea files
- *(backend)* Update cargo dependencies
- *(frontend)* Add SASS dependency
- *(frontend)* Use SASS rules & improve styling
- *(backend)* Update dependencies
- *(backend)* Move config example and update .gitignore
- Add docker configuration
