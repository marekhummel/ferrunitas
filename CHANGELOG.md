# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Support for f32 via feature

### Changed
-

### Deprecated
-

### Removed
-

### Fixed
-

### Security
-

## [0.3.0] - 2025-09-11

### Added
- Added multiple unit tests for better code coverage.
- Updated github workflow to push on tags.
- Added support for serde via a feature.
- Added support for no_std.
- Added support for approx crate via a feature.


## [0.2.0] - 2025-09-08

### Added
- Initial release to crates.io, including all necessary preperations for that (LICENSE, README, CHANGELOG, documentation, examples etc.)
- Support for offset quantities (temperature scales, see #1)
- Support for tagged quantities via a feature flag (`quantity_tags`), see #5
- QOL changes for measure creation.


## [0.1.0] - 2025-09-01

### Added
- Initial completion of basic type structure (Unit, Measure, Quantity)
