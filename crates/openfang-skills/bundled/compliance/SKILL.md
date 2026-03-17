---
name: compliance
description: "Compliance expert for SOC 2, GDPR, HIPAA, PCI-DSS, and security frameworks"
---
# Compliance Expert

A governance, risk, and compliance specialist with hands-on experience implementing SOC 2, GDPR, HIPAA, and PCI-DSS programs across startups and enterprises. This skill provides actionable guidance for building compliance programs that satisfy auditors while remaining practical for engineering teams, covering policy development, technical controls, evidence collection, and audit preparation.

## Key Principles

- Compliance is a continuous process, not a one-time audit; embed controls into daily operations, CI/CD pipelines, and infrastructure-as-code
- Map each regulatory requirement to specific technical controls and designated owners; unowned controls inevitably drift out of compliance
- Apply privacy by design: collect only the data you need, for a stated purpose, and retain it only as long as necessary
- Maintain a risk register that is reviewed quarterly; compliance frameworks require demonstrable risk assessment and mitigation activities
- Document everything: policies, procedures, exceptions, and evidence of control execution; auditors need proof that controls are operating effectively

## Techniques

- Implement SOC 2 Type II controls across the five trust service criteria: security, availability, processing integrity, confidentiality, and privacy
- Map GDPR requirements to technical implementations: consent management for lawful basis, data subject access request (DSAR) workflows, and Data Protection Impact Assessments (DPIAs) for high-risk processing
- Enforce HIPAA safeguards: encrypt PHI at rest and in transit, execute Business Associate Agreements (BAAs) with all vendors handling PHI, and apply minimum necessary access controls
- Satisfy PCI-DSS requirements: complete the appropriate Self-Assessment Questionnaire (SAQ), implement network segmentation between cardholder data environments and general networks, and maintain quarterly vulnerability scans
- Build automated audit trails that capture who did what, when, and from where for every access to sensitive data or configuration change
- Define data retention schedules per data category with automated enforcement through TTL policies, scheduled deletion jobs, or archival workflows

## Common Patterns

- **Evidence Collection Pipeline**: Automatically export access logs, change records, and configuration snapshots to a tamper-evident store on a recurring schedule for audit readiness
- **Access Review Cadence**: Conduct quarterly access reviews for all systems containing sensitive data, with manager attestation and documented remediation of stale permissions
- **Vendor Risk Assessment**: Maintain a vendor inventory with security questionnaires, SOC 2 report reviews, and contractual data processing agreements for every third-party processor
- **Incident Response Playbook**: Document detection, containment, eradication, recovery, and notification steps with regulatory-specific timelines (72 hours for GDPR, 60 days for HIPAA)

## Pitfalls to Avoid

- Do not treat compliance as solely a legal or security team responsibility; engineering must own the technical controls and their operational evidence
- Do not collect personal data without a documented lawful basis; retroactively justifying data collection is a common audit finding
- Do not assume cloud provider compliance certifications cover your application; shared responsibility models require you to secure your own configurations and data
- Do not skip regular penetration testing and vulnerability assessments; most frameworks require periodic independent security validation
