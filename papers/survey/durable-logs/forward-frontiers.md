# Recent primary fronts and device-fault evidence

2026-10-23. Added after the initial durable-log survey, in response to the operator's request for 2024–2026 forward-citation coverage and concrete fault/cost evidence. These are original papers, not evidence inferred from a repository's newest HEAD. All nine additional originals are retained and hashed in [bibliography.tsv](bibliography.tsv); no corresponding new codebase was ingested in this expansion.

## Recent shared logs: acquired and partially read

**LazyLog, SOSP 2024.** [Author original](https://dassl-uiuc.github.io/pdfs/papers/lazylog.pdf), Luo, Bhat, Hu, Alagappan, Ganesan. Read §§2–3. The paper separates durable ingestion from binding records to globally ordered positions. Its append result indicates durable receipt; ordering must be resolved before a position can be consumed. Interpretation: paying for correct order need not put every ordering step on the emitter's critical path. The exact append return semantics and read-after-append behavior matter; this is not a proposal to weaken required order. Erwin implementation details, proof, benchmarks, and matching code revision remain unread/unacquired.

**SpecLog/Belfast, OSDI 2025.** [Publisher original](https://www.usenix.org/system/files/osdi25-bhat.pdf), Bhat, Hong, Luo, Hu, Ganesan, Alagappan. Read abstract and introduction. The source describes predicted record delivery followed by ordering confirmation; incorrect speculation requires recomputation. Interpretation: speculative analytical work is a different event from a confirmed acquisition result or an irreversible external effect. The full API/proof and actual misspeculation cases require further reading before invoking its latency results. No performance number from this paper is used as a Deleuze estimate.



## Recent shared logs: acquired but not substantively read

- **PALF: Replicated Write-Ahead Logging for Distributed Databases**, PVLDB 2024. [Publisher original](https://www.vldb.org/pvldb/vol17/p3745-xu.pdf), Han et al. Original first page and authors verified; body and OceanBase implementation remain unread. This is a named next reading target, not a source of detailed claims here.
- **XLL: Cross-Layer Logging for Data Deduplication in Consensus-Based Storage**, NSDI 2026. [Publisher original](https://www.usenix.org/system/files/nsdi26-shawger.pdf), Shawger, Jhingran, Arpaci-Dusseau, Arpaci-Dusseau. Publisher abstract read; full recovery mechanism and artifact remain unread. Its author-reported gains are not imported into the report's quantitative comparisons.
- **BtrLog: Low-Latency Logging for Cloud Database Systems**, 2026 preprint. [Author-submitted original](https://arxiv.org/pdf/2606.27051), Kuschewski et al., acquired in the initial catalogue. Metadata verified; body and matching artifact remain unread.

## Actual device failures and their measurement boundaries

**Understanding the Robustness of SSDs under Power Fault, FAST 2013.** [Publisher original](https://www.usenix.org/system/files/conference/fast13/fast13-final80.pdf), Zheng, Tucek, Qin, Lillibridge. Read failure taxonomy/test-header mechanism, raw-device setup, and selected results. The authors physically interrupt device power and use raw-device synchronous/direct I/O with observed cache-flush commands. The study finds multiple failure classes, including bit corruption, sub-sector torn writes, reordered/unserializable writes, metadata loss, and dead devices; 13 of 15 tested SSDs exhibit unexpected behavior. Interpretation: synchronization API use and a device's actual power-failure behavior need separate evidence. This historical cohort cannot provide a modern enterprise SSD's failure probability, and its injected-fault rates are not a deployed annual-loss rate. Exact present hardware/firmware, power-loss protection, host caches, controllers, and error propagation remain a new study front.

Three newer originals are acquired as fault-model leads; only their publisher abstracts were read:

| Primary work | Scope to investigate next | What is not established here |
|---|---|---|
| [NVMe SSD Failures in the Field: the Fail-Stop and the Fail-Slow](https://www.usenix.org/system/files/atc22-lu.pdf), Lu et al., ATC 2022 | Field cohort, correlated failure, fail-slow behavior, repair/detection implications | No full dataset/method review or transfer of its rates to the operator's hosts |
| [RL-Watchdog](https://www.usenix.org/system/files/atc24-ha.pdf), Ha et al., ATC 2024 | Active liveness detection, notification latency, actual injected power faults | No kernel implementation review; detection accuracy is not event-retention probability |
| [FailureMiner](https://www.usenix.org/system/files/fast26-wang-shuyang.pdf), Wang et al., FAST 2026 | Modern production SSD monitoring and interpretable failure prediction | No attribution of prediction gains to end-to-end capture durability; false negatives and interventions need study |



## Primary sources searched and what remains

Searches on 2026-10-23 targeted publisher/author sources with the terms `shared log`, `logging durability`, `logging recovery`, `SSD power loss`, `fail-slow`, and `SSD reliability`, together with 2024, 2025 and 2026. Domains included USENIX conference/paper pages, PVLDB, arXiv author submissions, and linked author/lab sites. Named venue paths included OSDI 2025/2026, NSDI 2024/2026, FAST 2025/2026, ATC 2024, and PVLDB 2024/2025/2026. The search found LazyLog/PALF (2024), SpecLog (2025), LogDrive/XLL/BtrLog (2026), and the device studies above. Results and references are discovery evidence, not an exhaustive scan of every venue's full proceedings or a comprehensive forward-citation graph.

Additional primary preprints discovered but **not acquired or read in this lane**:

- [LogAct: Enabling Agentic Reliability via Shared Logs](https://arxiv.org/abs/2604.07988), 2026.
- [AgileLog: A Forkable Shared Log for Agents on Data Streams](https://arxiv.org/abs/2604.14590), 2026.

Root was notified because these cross directly into agent/evaluation semantics. Other remaining searches include ACM SOSP/EuroSys/ASPLOS 2024–2026, SIGMOD/PVLDB issues beyond the named results, DSN/SRDS fault models, persistent-memory and CXL failure literature, storage-sync error handling, latent-sector errors/scrubbing, erasure-code rebuild/repair, and archival durability/cost. Exact applicable device and cloud-provider contracts need their own acquisition; this report does not substitute marketing nines or old field averages for them.
