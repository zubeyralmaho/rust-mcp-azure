## Summary

<!-- One or two sentences describing the problem this PR solves and the chosen approach. -->

## Type of Change

- [ ] Bug fix (non-breaking change that fixes an issue)
- [ ] New feature (non-breaking change that adds functionality)
- [ ] Breaking change (contract, ingress, auth, or infrastructure shape)
- [ ] Documentation only
- [ ] CI / repository hygiene
- [ ] Infrastructure (`infra/`, `azure.yaml`, `Dockerfile`)

## Related Issues / ADRs

<!-- Link issues this PR closes or addresses. If it changes an accepted decision, link the new or updated ADR. -->

Closes #

## Changes

<!-- Bulleted list of meaningful changes. Keep it focused; large refactors should be split. -->

-
-

## Validation

- [ ] `cargo fmt --check`
- [ ] `cargo clippy --all-targets -- -D warnings`
- [ ] `cargo test --all-targets --locked`
- [ ] `./scripts/smoke-test.sh` against a local or deployed instance (if behavior changed)
- [ ] `az bicep build infra/main.bicep` (if `infra/` changed)
- [ ] Documentation updated (if behavior, contracts, or deployment changed)

## Security and Posture

- [ ] No new write, delete, install, or unrestricted shell paths
- [ ] No new dependencies added without justification
- [ ] Secrets remain in Container Apps secrets, not in plain env vars or code
- [ ] `safe_system_metrics` helper allowlist is unchanged, or the change is recorded in an ADR

## Reviewer Notes

<!-- Anything the reviewer should know: tradeoffs, follow-ups, or items intentionally deferred. -->
