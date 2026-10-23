# Integrity, evidence and retained history

2026-10-23. This supplemental lane connects audit-log cryptography to the capture
and state-history study. It does not select a cryptographic scheme, dependency or
new product requirement. [Bibliography](bibliography.tsv) records seven primary
works: six acquired originals and one unavailable report. Reading was bounded to
the sections stated below; there has been no cryptographic implementation audit.

The central distinction for Deleuze is between retaining an arrival and later
demonstrating that retained history has not changed. A valid digest authenticates
what was committed under its assumptions. It cannot establish that an event was
captured if that event was omitted before the commitment. Likewise, detecting
deletion does not restore the deleted bytes. These are separate obligations for
the gate-capture study, even if one implementation ultimately serves both.

| Lineage | Mechanism and reading boundary | Relevance to the in-house corpus |
| --- | --- | --- |
| [Authenticated Append-only Skip Lists](https://arxiv.org/abs/cs/0302010), 2003 | Introduction/background and proof interface framing. Published digests bind sequence position; membership and advancement evidence address different questions. Construction and security proof were not read completely. | Study how a retained prefix could be checked without replaying an entire linear chain. An external digest and its identity/version are part of the evidence boundary. |
| [LogFAS](https://reitermk.github.io/papers/2012/FC.pdf), 2012 | Abstract, introduction and comparison-table framing only. Public verification, compromise resilience and append-only security motivate schemes with different signer/verifier costs. Detailed algorithms and security reductions remain unread. | A citation front for later tradeoff analysis, not a basis for choosing an algorithm or quoting performance. |
| [RFC 9162](https://www.rfc-editor.org/rfc/rfc9162.html), 2021, Experimental | Introduction, Merkle proof framing, log-operation/MMD and security sections selected. An SCT promises future inclusion within the Maximum Merge Delay; an inclusion proof and a consistency proof provide different evidence. Detecting inconsistent views involves observation outside one isolated response. | A useful receipt-semantics counterexample: a signed promise need not mean current durable inclusion. Certificate Transparency's workload and trust model are not automatically Deleuze's. |
| [RFC 5848](https://www.rfc-editor.org/rfc/rfc5848.html), 2010, Standards Track | Introduction, signature-group/sequence/reboot/hash-block definitions and missing-message discussion selected. Authentication, sequencing and loss detection depend on the sender/collector configuration. A collector cannot treat every unmatched signed hash as proof that it was meant to receive that message. | Compare polyglot/remote envelope identity, restart epochs and independently observed omissions. Signatures do not recover a missing payload or remove the transport's failure cases. |



## Questions to carry into design discussion

- What evidence must survive a crash, lost device, malicious writer or later
  compromised key? These are different failure models with different costs.
- Which prefix is anchored, where, by whom, and when? What identifies its schema,
  producer epoch and sequence domain?
- Can a verifier distinguish a malformed tail, a missing original, a rewritten
  history, an unavailable anchor and a promise whose completion is unknown?
- Can expensive proof construction and indexing move off the producer path
  while preserving the exact bytes and meaning of the capture receipt?
- Does a proposed proof preserve raw observations, or only a projection? Can it
  expose sensitive values unnecessarily? These remain study questions.

Bellare–Yee's *Forward Integrity for Secure Audit Logs* is catalogued but its
original URL timed out. It is **not read** and no result here is attributed to it.
The original claims and later cryptographic literature need further study before
selecting a scheme. This lane is a scoped supplement, not a complete security
literature review.

## Provenance and restoration

Each acquired original's primary URL, retained local path and SHA-256 are in
[bibliography.tsv](bibliography.tsv). PDFs remain ignored under
`papers/pdfs/integrity/`; RFC text originals remain ignored under
`quarantine/sources/integrity/documents/`. Derived text is in ignored
`context/integrity/`. Restore an original by fetching its recorded URL to its
recorded path and checking SHA-256; a changed website is not the same original.
These are private research copies, not licensed for redistribution by this repo.
No original acquisition was modified. Reproducible execution/build closures are
outside this acquisition.
