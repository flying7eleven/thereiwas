# Changelog

All notable changes since the last release are documented here.

## [Unreleased]

### Added
- **(backend)** Add authentication route for retrieving a JSON Web Token ([03c4256])
- **(backend)** Add logging of calling IP during authentication ([2e1af47])
- **(backend)** Add a `/health` stub method which always returns HTTP 204 ([73f44c8])
- **(frontend)** Add the initial startup code for the website ([59c4226])i
- **(frontend)** Start the new frontend project ([5ca96be])
- **(docker)** Add files for creating a frontend container ([1240247])

### Changed
- **(backend)** Make `client_id` case sensitive ([fe54d15])
- **(backend)** Change authentication parameters to `client_id` and `client_secret` ([6f5c50d])
- **(backend)** Change the way login requests are logged ([fe1808f])
- **(backend)** Remove additional text from audit log entries ([39e95b4])
- **(backend)** Extract and simplify code for audit logging ([2ccafb4])
- **(backend)** General authentication improvements (in-progress) ([a2d3a16])
- **(backend)** Replace `.get(0)` with `.first()` ([068fef8])
- **(backend)** Use a string slice to prevent an object copy ([d940e5a])
- **(backend)** Reduce complexity of a data mapping operation ([acd35f9])

### Fixed
- **(backend)** Rename the `topic` column to `reporting_device` in the `locations` table ([d12ac88])

### CI / Build
- Ensure the frontend and backend containers are only built for the correct tags ([8f6ed5e])
- Update the workflows to also support the frontend ([c05b212])
- Rename the title of the used workflows ([4149c1e])

### Documentation
- Update the initial README file ([5b43299])
- Update the license years ([5f83c01])

[2e1af47]: https://github.com/flying7eleven/thereiwas/commit/2e1af47
[39e95b4]: https://github.com/flying7eleven/thereiwas/commit/39e95b4
[03c4256]: https://github.com/flying7eleven/thereiwas/commit/03c4256
[fe54d15]: https://github.com/flying7eleven/thereiwas/commit/fe54d15
[6f5c50d]: https://github.com/flying7eleven/thereiwas/commit/6f5c50d
[a2d3a16]: https://github.com/flying7eleven/thereiwas/commit/a2d3a16
[2ccafb4]: https://github.com/flying7eleven/thereiwas/commit/2ccafb4
[fe1808f]: https://github.com/flying7eleven/thereiwas/commit/fe1808f
[d12ac88]: https://github.com/flying7eleven/thereiwas/commit/d12ac88
[73f44c8]: https://github.com/flying7eleven/thereiwas/commit/73f44c8
[59c4226]: https://github.com/flying7eleven/thereiwas/commit/59c4226
[8f6ed5e]: https://github.com/flying7eleven/thereiwas/commit/8f6ed5e
[5f83c01]: https://github.com/flying7eleven/thereiwas/commit/5f83c01
[4149c1e]: https://github.com/flying7eleven/thereiwas/commit/4149c1e
[5b43299]: https://github.com/flying7eleven/thereiwas/commit/5b43299
[c05b212]: https://github.com/flying7eleven/thereiwas/commit/c05b212
[1240247]: https://github.com/flying7eleven/thereiwas/commit/1240247
[5ca96be]: https://github.com/flying7eleven/thereiwas/commit/5ca96be
[068fef8]: https://github.com/flying7eleven/thereiwas/commit/068fef8
[d940e5a]: https://github.com/flying7eleven/thereiwas/commit/d940e5a
[acd35f9]: https://github.com/flying7eleven/thereiwas/commit/acd35f9
