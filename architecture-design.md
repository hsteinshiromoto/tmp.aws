# Architecture Design: Kiro + AWS Security Agent Design Review Workflow

**Version**: 1.0
**Date**: 2026-05-01
**Status**: Draft

---

## 1. System Components and Responsibilities

### 1.1 Kiro Specs & Docs (Source of Truth)

- Maintains all project specifications, requirements, user stories, and design documents
- Produces architecture artifacts through the AIDLC workflow (Inception → Construction → Operations)
- Stores documents in `aidlc-docs/` directory structure including requirements, application design, functional design, NFR design, and infrastructure design
- Triggers the export step that feeds the review pipeline

### 1.2 Architecture Document (Export Artifact)

- A markdown document exported from Kiro containing the current architecture state
- Sections include: system components, data flows, auth approach, data storage/encryption, and external integrations
- Serves as the input contract for the AWS Security Agent
- Uploaded to the Agent Space for automated review

### 1.3 AWS Security Agent — Design Review Engine

- Consumes the exported architecture document from the Agent Space
- Performs three core analysis functions:
  - **Compliance Check** — Validates architecture against security baselines, regulatory frameworks (e.g., SOC 2, HIPAA, PCI-DSS), and organizational policies
  - **Architecture Risk Analysis** — Identifies structural weaknesses, single points of failure, blast radius concerns, and anti-patterns
  - **Threat Modeling** — Applies threat frameworks (e.g., STRIDE) to identify attack surfaces, trust boundaries, and unmitigated threats

### 1.4 Review Report (Findings Artifact)

- Structured output from the Design Review Engine
- Contains prioritized gaps, risk ratings, and actionable recommendations
- Organized by severity (Critical / High / Medium / Low) with remediation guidance

### 1.5 Update Specs & Docs + Plan Bolts (Feedback Loop)

- Consumes the Review Report findings
- Updates Kiro specs and design documents to address identified gaps
- Creates "Plan Bolts" — targeted work items that bolt onto the existing implementation plan to remediate security findings
- Closes the feedback loop, enabling iterative review cycles

---

## 2. Data Flow Between Components

```
Step  Source                    Data                        Destination
────  ────────────────────────  ──────────────────────────  ──────────────────────────
 1    Kiro Specs & Docs         Architecture Document (MD)  Agent Space (upload)
 2    Agent Space               Architecture Document       Design Review Engine
 3    Design Review Engine      Compliance results          Review Report
 4    Design Review Engine      Risk analysis results       Review Report
 5    Design Review Engine      Threat model results        Review Report
 6    Review Report             Findings + Recommendations  Kiro Specs & Docs
 7    Kiro Specs & Docs         Updated designs + Bolts     (next review cycle)
```

### Data Flow Details

1. **Export (Kiro → Agent Space)**: Kiro exports the architecture document as a markdown file. The document is uploaded to the AWS Security Agent's workspace (Agent Space) via a supported transfer mechanism (e.g., S3 upload, API call, or manual upload).

2. **Ingestion (Agent Space → Review Engine)**: The Design Review Engine reads the architecture document from the Agent Space. Parsing extracts structured sections for targeted analysis.

3. **Analysis (Review Engine → Report)**: The engine runs compliance, risk, and threat analyses in parallel. Results are aggregated into a single Review Report with cross-referenced findings.

4. **Feedback (Report → Kiro)**: The Review Report is returned to the development workflow. Findings are mapped back to specific specs and design documents. Plan Bolts are generated as discrete, actionable remediation tasks.

5. **Iteration**: The updated architecture can be re-exported for another review cycle until findings are resolved.

---

## 3. Authentication and Authorization Approach

### 3.1 Kiro to Agent Space

| Concern | Approach |
|---|---|
| Identity | IAM roles or federated identity (SSO) for the user/CI pipeline triggering the export |
| Authorization | Scoped IAM policies granting `s3:PutObject` or agent API `invoke` permissions to the Agent Space |
| Transport | TLS 1.2+ for all data in transit |

### 3.2 Agent Space to Design Review Engine

| Concern | Approach |
|---|---|
| Identity | AWS service-to-service authentication via IAM execution roles |
| Authorization | Least-privilege role allowing the agent to read from its designated input location and write review results |
| Isolation | Agent runs in a sandboxed execution environment with no access beyond its scoped resources |

### 3.3 Review Report to Kiro

| Concern | Approach |
|---|---|
| Identity | The consuming user/automation authenticates via the same IAM/SSO identity used for export |
| Authorization | Read access to review results scoped to the originating project |
| Integrity | Report includes a hash or signature to verify it has not been tampered with in transit |

### 3.4 Principles

- No long-lived credentials; use IAM roles and temporary session tokens
- All cross-component communication over encrypted channels (TLS)
- Principle of least privilege at every boundary

---

## 4. Data Storage and Encryption Strategy

### 4.1 Storage Locations

| Data | Storage | Retention |
|---|---|---|
| Kiro Specs & Docs | Local filesystem / Git repository | Permanent (version controlled) |
| Exported Architecture Document | Agent Space (S3 bucket or agent workspace) | Duration of review cycle |
| Review Report | Agent Space output + local filesystem | Permanent (version controlled) |
| Audit Trail | `aidlc-docs/audit.md` in Git | Permanent |

### 4.2 Encryption

| State | Mechanism |
|---|---|
| At rest (S3) | SSE-S3 or SSE-KMS with customer-managed key |
| At rest (local) | OS-level disk encryption (FileVault / LUKS) |
| In transit | TLS 1.2+ for all API calls and file transfers |
| Key management | AWS KMS for all cloud-side encryption keys; key rotation enabled |

### 4.3 Data Classification

- Architecture documents may contain sensitive design details (API schemas, internal service names, infrastructure topology) — treat as **Confidential**
- Review reports may reference vulnerabilities — treat as **Restricted** until remediated
- Access to both should be limited to the project team and security reviewers

---

## 5. External Integrations

### 5.1 AWS Services

| Service | Purpose |
|---|---|
| **Amazon Bedrock / Agent Space** | Hosts the AWS Security Agent; receives architecture documents and returns review reports |
| **Amazon S3** | Intermediate storage for exported documents and review artifacts |
| **AWS KMS** | Encryption key management for data at rest |
| **AWS IAM** | Authentication and authorization for all service interactions |
| **AWS CloudTrail** | Audit logging of all API calls in the review pipeline |

### 5.2 Development Toolchain

| Integration | Purpose |
|---|---|
| **Kiro CLI** | Orchestrates the AIDLC workflow; produces and consumes architecture artifacts |
| **Git** | Version control for all specs, docs, and review reports |
| **CI/CD Pipeline** (optional) | Automates the export → review → feedback cycle on pull requests or design changes |

### 5.3 Integration Pattern

```
Developer workstation                          AWS Cloud
┌─────────────────────┐                ┌──────────────────────────────┐
│  Kiro CLI           │   HTTPS/TLS   │  Agent Space                 │
│  ┌───────────────┐  │───────────────▶│  ┌────────────────────────┐  │
│  │ aidlc-docs/   │  │               │  │ Design Review Engine   │  │
│  │ architecture- │  │               │  │ (Bedrock Agent)        │  │
│  │ design.md     │  │◀──────────────│  │                        │  │
│  └───────────────┘  │   Findings    │  └────────────────────────┘  │
│  ┌───────────────┐  │               │  ┌────────────────────────┐  │
│  │ Plan Bolts    │  │               │  │ S3 + KMS               │  │
│  └───────────────┘  │               │  └────────────────────────┘  │
└─────────────────────┘                │  ┌────────────────────────┐  │
                                       │  │ IAM + CloudTrail       │  │
                                       │  └────────────────────────┘  │
                                       └──────────────────────────────┘
```
