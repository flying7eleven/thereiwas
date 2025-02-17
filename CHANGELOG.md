# Changelog

All notable changes since the last release are documented here.

## [Unreleased]

Nothing so far

## 2024.2.17 (backend-only)

### Added
- **(backend)** Prepare the container to mount the encryption keys ([501e743])
- **(backend)** Add default paths for the private and public key ([fb79f35])

### Changed
- **(backend)** Upgrade the used dependencies ([aecdf19])
- **(backend)** Remove unnecessary borrowing ([8663110])
- **(backend)** Remove unused imports ([0711aa6])

### Documentation
- Fix version name in the changelog ([d82c945])


## 2025.2.14 (backend-only)

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


## 2025.1.15 (backend-only)

### Added
- **(backend)** Store authentication requests: Introduced logic to record all authentication requests in the database ([70775c2]).
- **(backend)** Authorization requests table: Created a new `authorization_requests` table ([4b8ff7a]).
- **(backend)** Request guard: Implemented a guard that ensures only allowed clients can post to the API ([8a8ac06]).
- **(backend)** Client credentials table: Added a new table to store client credentials ([4a0d252]).
- **(backend)** AP tracking:
    - Update the `last_seen` column upon re-discovery of an access point ([db960c6]).
    - Store the initial time an access point was first detected ([5506e6a]).
    - Added a column to track the time an AP was last seen ([3a87b6d]).

### Changed
- **(backend)** Use the `?` operator: Removed explicit error handling in favor of the idiomatic `?` operator ([2b39d68]).
- **(backend)** String handling: Switched from copying strings to using slices for improved efficiency ([bce50b7]).
- **(backend)** `_.map_or` readability: Enhanced clarity by improving how `map_or` is called ([40e13c8]).
- **(backend)** RustRover configs: Moved the run configurations to the correct directory ([7b97309]).
- **(backend)** Upgrade dependencies: Bumped versions of third-party libraries and tools for performance and security improvements ([76bdc94], [f718ca4]).

### Fixed
- **(backend)** Client unwrapping bug: Properly handle client lookup errors by using correct unwrap logic and robust error handling ([b9e8a6d]).

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
[70775c2]: https://github.com/flying7eleven/thereiwas/commit/70775c2
[4b8ff7a]: https://github.com/flying7eleven/thereiwas/commit/4b8ff7a
[8a8ac06]: https://github.com/flying7eleven/thereiwas/commit/8a8ac06
[4a0d252]: https://github.com/flying7eleven/thereiwas/commit/4a0d252
[db960c6]: https://github.com/flying7eleven/thereiwas/commit/db960c6
[5506e6a]: https://github.com/flying7eleven/thereiwas/commit/5506e6a
[3a87b6d]: https://github.com/flying7eleven/thereiwas/commit/3a87b6d
[2b39d68]: https://github.com/flying7eleven/thereiwas/commit/2b39d68
[bce50b7]: https://github.com/flying7eleven/thereiwas/commit/bce50b7
[40e13c8]: https://github.com/flying7eleven/thereiwas/commit/40e13c8
[7b97309]: https://github.com/flying7eleven/thereiwas/commit/7b97309
[76bdc94]: https://github.com/flying7eleven/thereiwas/commit/76bdc94
[f718ca4]: https://github.com/flying7eleven/thereiwas/commit/f718ca4
[b9e8a6d]: https://github.com/flying7eleven/thereiwas/commit/b9e8a6d
[8663110]: https://github.com/flying7eleven/thereiwas/commit/8663110
[0711aa6]: https://github.com/flying7eleven/thereiwas/commit/0711aa6
[501e743]: https://github.com/flying7eleven/thereiwas/commit/501e743
[aecdf19]: https://github.com/flying7eleven/thereiwas/commit/aecdf19
[fb79f35]: https://github.com/flying7eleven/thereiwas/commit/fb79f35
[d82c945]: https://github.com/flying7eleven/thereiwas/commit/d82c945
