# Contracts issues

Open issues for **aidline-contracts**, grouped by complexity. GitHub is the source of truth: [see all issues](https://github.com/aidline-org/aidline-contracts/issues).

| Complexity | Wave points | Open issues |
| --- | --- | --- |
| Trivial | 100 | 9 |
| Medium | 150 | 14 |
| High | 200 | 7 |
| **Total** | | **30** |

Good first issues: 9. They are small, well scoped and a good way to start.

To pick one up, comment on the issue to get assigned, then follow [CONTRIBUTING.md](CONTRIBUTING.md).

## Trivial (100 points)

| # | Issue | Type | Good first issue |
| --- | --- | --- | --- |
| [#1](https://github.com/aidline-org/aidline-contracts/issues/1) | Add doc comments to every public contract function | documentation | yes |
| [#2](https://github.com/aidline-org/aidline-contracts/issues/2) | Emit an event when the admin changes | enhancement | yes |
| [#3](https://github.com/aidline-org/aidline-contracts/issues/3) | Test donations from several donors that exactly reach the goal | testing | yes |
| [#11](https://github.com/aidline-org/aidline-contracts/issues/11) | Add tests for the read only view functions | testing | yes |
| [#12](https://github.com/aidline-org/aidline-contracts/issues/12) | Add a version view to the contract | enhancement | yes |
| [#13](https://github.com/aidline-org/aidline-contracts/issues/13) | Limit the length of metadata and proof URIs | security | yes |
| [#14](https://github.com/aidline-org/aidline-contracts/issues/14) | Reject campaigns where the verifier is also the beneficiary | security | yes |
| [#15](https://github.com/aidline-org/aidline-contracts/issues/15) | Add an is_refundable view | enhancement | yes |
| [#16](https://github.com/aidline-org/aidline-contracts/issues/16) | Document every contract call with Stellar CLI examples | documentation | yes |

## Medium (150 points)

| # | Issue | Type | Good first issue |
| --- | --- | --- | --- |
| [#4](https://github.com/aidline-org/aidline-contracts/issues/4) | Let the admin reassign a campaign's verifier | enhancement |  |
| [#5](https://github.com/aidline-org/aidline-contracts/issues/5) | Allow anyone to extend storage TTL for long running campaigns | enhancement |  |
| [#6](https://github.com/aidline-org/aidline-contracts/issues/6) | Accept the remaining amount when a donation would pass the goal | enhancement |  |
| [#7](https://github.com/aidline-org/aidline-contracts/issues/7) | Property tests for refund math | testing |  |
| [#8](https://github.com/aidline-org/aidline-contracts/issues/8) | Publish generated TypeScript bindings from CI | tooling |  |
| [#17](https://github.com/aidline-org/aidline-contracts/issues/17) | Pause switch for emergencies | security |  |
| [#18](https://github.com/aidline-org/aidline-contracts/issues/18) | Upgradeable contract with an admin controlled upgrade path | enhancement |  |
| [#19](https://github.com/aidline-org/aidline-contracts/issues/19) | Let creators update metadata before the first donation | enhancement |  |
| [#20](https://github.com/aidline-org/aidline-contracts/issues/20) | Allow a one time deadline extension | enhancement |  |
| [#21](https://github.com/aidline-org/aidline-contracts/issues/21) | Let verifiers reject a milestone with a reason | enhancement |  |
| [#22](https://github.com/aidline-org/aidline-contracts/issues/22) | Optional platform fee sent to a treasury | enhancement |  |
| [#23](https://github.com/aidline-org/aidline-contracts/issues/23) | Emergency fast track for the first milestone | enhancement |  |
| [#24](https://github.com/aidline-org/aidline-contracts/issues/24) | Measure and document resource costs per function | documentation |  |
| [#25](https://github.com/aidline-org/aidline-contracts/issues/25) | Assert exact events in tests | testing |  |

## High (200 points)

| # | Issue | Type | Good first issue |
| --- | --- | --- | --- |
| [#9](https://github.com/aidline-org/aidline-contracts/issues/9) | Multisig verification: m of n verifiers per milestone | enhancement |  |
| [#10](https://github.com/aidline-org/aidline-contracts/issues/10) | Per campaign token so campaigns can raise in USDC | enhancement |  |
| [#26](https://github.com/aidline-org/aidline-contracts/issues/26) | Per milestone due dates with partial refunds when overdue | enhancement |  |
| [#27](https://github.com/aidline-org/aidline-contracts/issues/27) | Matching funds from sponsors | enhancement |  |
| [#28](https://github.com/aidline-org/aidline-contracts/issues/28) | Fuzz testing harness | security |  |
| [#29](https://github.com/aidline-org/aidline-contracts/issues/29) | Verifier bonds that can be slashed for fraud | security |  |
| [#30](https://github.com/aidline-org/aidline-contracts/issues/30) | Pledges that fund future milestones on approval | enhancement |  |
