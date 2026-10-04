# Changelog

All notable changes to this project are documented here. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the project uses [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.1.0] - 2026-10-04

First public testnet release.

### Added

- Milestone based escrow contract with `Emergency` and `Climate` campaign types
- Verifier registry managed by an admin
- Donations capped at the campaign goal
- Milestone release by the assigned verifier, with a proof URI emitted on chain
- Cancellation by the creator or admin
- Pro rata refunds of unreleased funds after cancellation or expiry
- Events for every state change, for indexers
- Unit tests, CI, testnet deploy script and architecture docs
