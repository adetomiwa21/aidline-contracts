# Security Policy

Aidline handles donated funds, so we take security reports seriously.

**The contracts are not audited yet. Do not use them with real funds on mainnet.**

## Reporting a vulnerability

Please do not open a public issue for security problems. Instead, use GitHub's private reporting: go to the **Security** tab of this repo and click **Report a vulnerability**.

Include what you found, how to reproduce it, and the impact you expect. We aim to reply within 72 hours.

## Scope

- Logic that lets funds leave escrow without a valid approval or refund
- Ways to block refunds or lock funds permanently
- Authorization bypasses on admin, creator or verifier functions
- Incorrect refund math
