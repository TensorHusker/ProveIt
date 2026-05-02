# Security Policy

## Overview

ProveIt takes security seriously, particularly because formal verification tools are sometimes used in safety-critical and security-critical contexts. This document describes our security practices and how to report vulnerabilities.

## Supported Versions

ProveIt is in early development. Security updates are provided for:

| Version | Supported          |
| ------- | ------------------ |
| main    | Yes (latest)       |
| 0.1.x   | Yes                |
| < 0.1   | No                 |

Once we reach 1.0, we will support the latest minor version with security patches and provide a deprecation timeline for older versions.

## Threat Model

ProveIt's security considerations span several categories:

### 1. Soundness of Verification

The most critical security property: a verified proof must be mathematically valid.

**Threats**:
- Bugs in the verification engine accepting invalid proofs
- Inconsistencies between formal systems leading to false claims
- Subtle errors in inference rule implementations

**Mitigations**:
- Extensive property-based testing of inference rules
- Formal specification of the verification algorithm
- Independent review of changes to verification logic
- Mutation testing to find weak test coverage

### 2. Untrusted Input Processing

Proof files, geometric constructions, and configuration may come from untrusted sources.

**Threats**:
- Denial of service through deeply nested or large proofs
- Resource exhaustion (memory, CPU, disk)
- Path traversal via crafted file names
- Deserialization vulnerabilities

**Mitigations**:
- Resource limits on proof size and verification time
- Sandboxing of proof execution
- Validated parsing with explicit error handling
- No execution of arbitrary code from proof files

### 3. Dependency Vulnerabilities

Third-party crates may contain vulnerabilities.

**Threats**:
- Vulnerable dependencies introduced through `cargo`
- Supply chain attacks via typosquatting
- Malicious updates to dependencies

**Mitigations**:
- `cargo audit` in CI for known vulnerabilities
- Lock file (`Cargo.lock`) committed for reproducible builds
- Minimal dependency footprint
- Regular dependency review and updates
- `cargo-deny` for license and source verification

### 4. Accessibility-Specific Concerns

Accessibility features can have unique security implications.

**Threats**:
- Screen reader information leakage (announcing sensitive content)
- Audio cue side-channel information disclosure
- Keyboard input logging concerns

**Mitigations**:
- Configurable verbosity for screen reader output
- User control over which information is announced
- No logging of input streams without explicit consent

## Reporting a Vulnerability

### Please Do

- **Report privately first** to give us time to address the issue
- **Include detailed information** to help us reproduce
- **Allow reasonable time** for response and remediation
- **Coordinate disclosure** with the maintainers

### Please Do Not

- Open a public GitHub issue for security vulnerabilities
- Disclose the vulnerability publicly before a patch is available
- Test vulnerabilities against systems you don't own
- Access or modify data belonging to others

### How to Report

**Preferred method**: Email security reports to the project maintainers. Check the project's main repository for current contact information.

**Alternative**: Use GitHub's [private vulnerability reporting](https://docs.github.com/en/code-security/security-advisories/guidance-on-reporting-and-writing-information-about-vulnerabilities/privately-reporting-a-security-vulnerability) feature.

### What to Include

A good security report includes:

1. **Type of vulnerability** (e.g., soundness bug, DoS, code execution)
2. **Affected components** (modules, versions)
3. **Steps to reproduce** with minimal example
4. **Impact assessment** (what can an attacker do?)
5. **Suggested fix** if you have one
6. **Your name/handle** for credit (or "anonymous")
7. **Disclosure timeline** preferences

### Example Report

```
Subject: [Security] Soundness bug in identity type verification

Vulnerability: The verification engine accepts a proof where
two non-equal terms are claimed to be propositionally equal.

Affected: src/verification/identity.rs (all versions)

Reproduction:
1. Create proof: ...
2. Verify with: cargo run -- verify proof.pi
3. Verifier returns Ok(())

Expected: Verifier should return Err(TypeMismatch{...})

Impact: An attacker can construct false proofs of equality,
undermining the soundness of any verification using identity types.

Suggested fix: Add the check at line 42 of identity.rs to handle
the eta-expansion case.

Reporter: Jane Doe <jane@example.com>
Disclosure: Prefer 90 days from acknowledgment.
```

## Response Process

When we receive a security report, we will:

1. **Acknowledge receipt** within 72 hours
2. **Assess the report** and confirm the vulnerability
3. **Develop a fix** in a private branch
4. **Coordinate disclosure** with the reporter
5. **Release a patch** with security advisory
6. **Credit the reporter** (unless they prefer anonymity)

### Response Timeline

| Phase | Target Time |
|-------|-------------|
| Initial acknowledgment | 72 hours |
| Vulnerability assessment | 1 week |
| Fix development | 2-4 weeks (varies by severity) |
| Coordinated disclosure | 90 days from initial report |

For critical vulnerabilities, we may release patches more quickly.

## Severity Classification

We use the following severity levels:

### Critical

- Soundness bugs allowing arbitrary false proofs
- Remote code execution
- Authentication bypass (when applicable)

**Response**: Immediate fix, expedited disclosure

### High

- Denial of service that's difficult to mitigate
- Information disclosure of sensitive proof content
- Bypasses of resource limits

**Response**: Fix within 2 weeks, standard disclosure

### Medium

- Crashes that don't compromise verification
- Performance issues with malicious input
- Minor information disclosure

**Response**: Fix in next minor release

### Low

- Hardening opportunities
- Defense-in-depth improvements

**Response**: Address as part of regular development

## Security Best Practices for Users

When using ProveIt:

1. **Verify proof sources**: Only trust proofs from reliable sources
2. **Use latest version**: Keep ProveIt updated for security patches
3. **Review dependencies**: Audit `Cargo.lock` for your applications
4. **Apply resource limits**: Use sandboxing for untrusted proofs
5. **Validate outputs**: Cross-check critical results with other tools
6. **Report issues**: Help improve ProveIt's security

## Security Hall of Fame

Researchers who report valid vulnerabilities will be acknowledged here (with their permission):

*No reports yet.*

## Cryptographic Considerations

ProveIt does not currently provide cryptographic services. If proofs need to be:

- **Signed** for authenticity: use external tools (GPG, minisign)
- **Encrypted** for confidentiality: use full-disk encryption or external tools
- **Timestamped** for non-repudiation: use trusted timestamping authorities

If cryptographic features are added in the future, this section will be updated with the threat model and supported algorithms.

## Compliance

ProveIt aims to be suitable for use in regulated environments. We:

- Follow secure development practices
- Maintain a software bill of materials (SBOM) for dependencies
- Document our security architecture
- Support reproducible builds

For specific compliance questions (e.g., FIPS, Common Criteria), please contact the maintainers.

## Contact

For security-related questions that are not vulnerabilities:

- Open a [GitHub Discussion](https://github.com/TensorHusker/ProveIt/discussions) tagged `security`
- Review existing security advisories in the repository

For vulnerability reports, use the private channels described above.
