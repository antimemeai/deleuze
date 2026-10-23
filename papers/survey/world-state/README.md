# World-state acquisition, causal time, temporal state and provenance

Acquired and studied 2026-10-23. The operator’s priorities are to record every arrival at Deleuze’s instrumentation gate, fund the desired reliability, and make capture extremely fast. This survey studies the adjacent state machinery without selecting a model, storage engine, dependency or architecture.

The lane catalogues **47 primary works**, retains **41 PDF originals** (40 new downloads and the existing Lamport PDF), and references two original web documents plus one publisher abstract in place. Three original downloads remain unavailable; the publisher-abstract entry also lacks full text. Central sections of **17 works** were read; other reading is explicitly marked as selected passages, introduction, abstract, web reading, or catalogued-only. Eight implementation archives contain **46,600 extracted files**. Acquisition is not a claim that the entire corpus was read or verified.

- [Bibliography](bibliography.tsv): authors, publication and acquired-edition information, DOI where established, primary URL, original location and SHA-256, acquisition date/status, and precise reading status.
- [Pinned implementations](references.tsv) and [source-reading record](source-reading.tsv): commits, rights evidence, hashes, paths read and missing submodules.
- [Citation map](citation-map.tsv): selected relationships checked in the original works. This is a reading map, not an exhaustive citation graph.
- [2024–2026 forward checks](forward-checks.tsv): seven recent works, exactly what primary material was checked, and whether each relationship is a traced citation or a conceptual connection.
- [Remaining acquisition and citation fronts](SHOPPING_LIST.md).
- [Acquisition and restore utility](acquire.py). Originals remain in ignored quarantine. Existing Lamport material is referenced in its existing location; web specifications are linked rather than duplicated.

## The questions these lineages answer

| Lineage | Its object | Useful distinction for Deleuze | Claim it does not establish |
|---|---|---|---|
| Logical/vector/hybrid time | Relations among observed events | Observation order, causal order and clock time differ | A sortable timestamp does not recover all causal dependencies |
| Consistent cuts and snapshots | A process/channel state compatible with an execution | A state image includes in-flight effects and recovery assumptions | Simultaneous wall-clock readings need not form a consistent state |
| Temporal databases | Assertions indexed by valid and transaction/system time | What was believed when differs from when the assertion applies | A corrected current view is not the original arrival history |
| Data and system provenance | Derivation or information-flow relationships | Input origin, query witnesses and OS dependencies answer different questions | A provenance edge is not automatically an exact causal explanation |
| CRDTs and CALM | Convergent state/program outcomes | Convergence and monotonicity can avoid some coordination | Convergence does not certify every attempted or repeated arrival was retained |
| Incremental computation | Changes to derived collections over logical versions | Fast updated views can avoid repeated full reconstruction | Compacted differences need not preserve their original chronology |
| Streaming/recovery | Stateful transformations plus progress and replay | “No more earlier inputs,” “committed state” and “visible output” are separate claims | Processing exactly once does not record every delivery attempt |

These boundaries matter together. Preserving every gate arrival is an acquisition claim. Answering a query about an observed world, reconstructing an application, or explaining a result adds assumptions about instrumentation coverage, retained inputs and computation. None of the surveyed state models makes those assumptions disappear.

## Causal time and consistent state

[Lamport 1978](https://lamport.azurewebsites.net/pubs/time-clocks.pdf) defines happened-before from local order, send/receive and transitivity. Logical clocks preserve that relation in one direction; total ordering adds an arbitrary process tie-break. [Fidge 1988](https://zoo.cs.yale.edu/classes/cs426/2012/lab/bib/fidge88timestamps.pdf) and [Mattern 1989](https://pages.cs.wisc.edu/~remzi/Classes/739/Fall2018/Papers/mattern89.pdf) are catalogued vector-time roots; Fidge’s original could not be downloaded in this run. The [global-state report](https://www.cs.cornell.edu/courses/cs5414/2017fa/papers/chapt4.pdf), §6, defines a consistent cut by closure under causal predecessors. Its ghost-deadlock example demonstrates why collecting local states is insufficient.

[Chandy–Lamport 1985](https://lamport.azurewebsites.net/pubs/chandy.pdf), §§2–4, records process states and channel contents using markers. The model assumes reliable FIFO channels; termination also requires marker delivery and participation. The recorded state need not literally have appeared in the observed interleaving, but is compatible with a reachable rearrangement. **Research implication:** a distributed “world snapshot” needs a declared consistency interpretation; it cannot be inferred solely from a collection timestamp or a total log order.

[HLC 2014](https://cse.buffalo.edu/tech-reports/2014-04.pdf), §§2–3, preserves one-way causality with a physical-clock-derived component and logical counter. Its closeness and counter bounds have clock/rate assumptions. It does not make unequal timestamps a proof of causality. [Spanner 2012](https://www.usenix.org/system/files/conference/osdi12/osdi12-final-16.pdf) exposes clock uncertainty as part of a substantially larger transaction system; only its introduction was read here. [Interval Tree Clocks](https://webarchive.di.uminho.pt/haslab.uminho.pt/cbm/files/itc.pdf) is acquired but not read and remains a dynamic-membership frontier.



## Temporal assertions and event history

[Snodgrass–Ahn 1985](https://www2.cs.arizona.edu/~rts/pubs/SIGMOD85.pdf), §4, distinguishes transaction time, valid time and user-defined time. A rollback database remembers what was stored; a historical database can describe the best current account of reality without retaining previous mistakes. [Jensen–Soo–Snodgrass 1993](https://www2.cs.arizona.edu/~rts/pubs/ICDE93.pdf), selected §2, separates conceptual bitemporal semantics from representations, using transaction/valid-time chronons and snapshot equivalence. A conceptual clock-tick update is not prescribed as a disk implementation.

[Fowler’s original event-sourcing essay](https://martinfowler.com/eaaDev/EventSourcing.html) describes changes initiated by events and rebuilding application state from them. Its claim is stronger than merely keeping a change log, and its replay sections discuss external interactions. The page itself identifies the material as draft. [Helland’s CIDR immutability paper](https://www.cidrdb.org/cidr2015/Papers/CIDR15_Paper16.pdf) is acquired but not read; it is not used as authority for a design decision here.



## Provenance: what was copied, what contributed, and what could influence it

[Buneman–Khanna–Tan 2001](https://www.cis.upenn.edu/~sanjeev/papers/icdt01_data_provenance.pdf) separates why and where provenance. The acquired PDF’s text encoding is damaged; pages 3 and 7 were rendered and read visually. [Green–Karvounarakis–Tannen 2007](https://www.cs.ucdavis.edu/~green/papers/pods07.pdf), §§3–4, annotates tuples with semiring values: union/projection combine alternatives with addition, joins combine contributions with multiplication. Its primary algebra is positive relational algebra; this does not by itself give arbitrary imperative programs the same provenance semantics. The [Cheney–Chiticariu–Tan survey](https://homepages.inf.ed.ac.uk/jcheney/publications/provdbsurvey.pdf) was acquired and its scope read as a guide to deeper citation work.

[PROV-DM](https://www.w3.org/TR/2013/REC-prov-dm-20130430/) provides conceptual entities, activities, agents and relations, with separate companion constraints. That vocabulary is an interchange model, not a durability mechanism. Its identification of responsible agents also does not authenticate an asserted identity.

[PASSv2 / Layering in Provenance Systems](https://www.usenix.org/legacy/events/usenix09/tech/full_papers/muniswamy-reddy/muniswamy-reddy.pdf), use cases and §4, shows why workflow provenance and system-level provenance each omit relationships available at another layer. It discusses cross-layer names, object versions, data/metadata consistency and cycles. **Research implication:** for evaluation fidelity, “the file was read” and “this value contributed to this answer” are different observations; retaining both does not require pretending they have equal precision.

[CamFlow 2017](https://camflow.org/publications/socc-2017.pdf), §§3–5, models versioned kernel objects and their flows, with application-disclosed provenance and configurable capture/filter policy. Shared-memory flows are conservatively approximated because syscall observation cannot identify each intra-process access. [CamQuery 2018](https://camflow.org/publications/ccs-2018.pdf), introduction and architecture, moves graph analysis toward the live capture path. Its capture/collection/storage/query layers expose additional boundaries between an event occurring, being transported and being available to analysis. These historical evaluations are not measurements of Deleuze’s gate throughput or reliability.

[GProM 2014](https://www.cs.uic.edu/~bglavic/dbgroup/assets/pdfpubls/AG14.pdf) can reconstruct transaction provenance when database audit history and time travel exist. The current acquired rewriter dispatches SQL/Datalog provenance requests into query transformations. **Research implication:** post-hoc lineage can derive relationships from retained history; it cannot reconstruct original bytes or nondeterministic inputs that were never captured.



## Convergent state and incremental views

[Shapiro et al. 2011](https://inria.hal.science/inria-00555588/document), §§2.2–2.3, gives state-based convergence conditions using inflationary updates and join-semilattice merges. Message loss/duplication/reordering is compatible with that model only when new state eventually reaches replicas. Operation-based replication has different dissemination requirements. [Delta-state CRDTs](https://arxiv.org/pdf/1603.01529), §§3–4, transfers joinable delta states and groups; several changes can be subsumed by one communicated state. The acquired bytes are the 2016 v1 preprint, separately identified from the 2018 journal publication.

[Kleppmann–Beresford 2017](https://www.cl.cam.ac.uk/~arb33/papers/KleppmannBeresford-CRDT-JSON-TPDS2017.pdf) was read selectively for dependencies, concurrent assignment and tombstones; its datatype handles nested replicated documents, not arbitrary native application memory. [CALM](https://arxiv.org/pdf/1901.01930), selected §2.2, explains coordination-freedom through monotonic program outcomes within its formal model. Neither a mergeable state nor a monotonic answer says that every observation, retry or rejected input appears in an append history. The JSON title is historical research context, not a proposal to override Deleuze’s native/binary doctrine.

[Differential Dataflow 2013](https://www.cidrdb.org/cidr2013/Papers/CIDR13_Paper111.pdf), §3, represents changing collections by differences over partially ordered versions, including removals. [DBSP 2023](https://www.vldb.org/pvldb/vol16/p1601-budiu.pdf), §§1–2, defines incrementalization through differentiation, computation and integration over streams. These are models of derived computation; their reuse of work is relevant to fast world-state views, independently of how original arrivals are retained.

The pinned Differential source makes the boundary concrete: `TraceReader::set_logical_compaction` permits changing historical timestamps while preserving later query comparisons. `difference.rs` permits retiring accumulated zero differences. Those are intentional view/storage semantics. They cannot be assumed to retain a forensic chronology. Automerge’s `Change` retains actor, sequence, dependency hashes and typed verified/unverified decoding states; that is a distinct retained-change structure, not a generic log-sink contract.

## Streaming completeness, state recovery and effect boundaries

[MillWheel 2013](https://www.vldb.org/pvldb/vol6/p1033-akidau.pdf), §§5–6, combines record identity, retries, duplicate detection and atomic state updates, with strong productions and low watermarks. Late records remain an application decision. Its processing guarantee operates inside a specified framework boundary. Recording every delivery attempt is a different requirement from suppressing duplicate processing of an identified record.

[The Dataflow Model 2015](https://www.vldb.org/pvldb/vol8/p1792-Akidau.pdf), §§1–2, separates event and processing time, windows, triggers and refinements. Its discussion explicitly permits heuristic watermarks when outside producers can return arbitrarily late. [Watermarks 2021](https://www.vldb.org/pvldb/vol14/p3135-begoli.pdf), §§3–5, compares watermarks, frontiers and punctuations, then contrasts in-band and out-of-band propagation. The same abstract completeness semantics can incur different update latency and recovery behavior. A trusted progress assertion and a heuristic estimate must not silently exchange meanings.

[Asynchronous Barrier Snapshots 2015](https://arxiv.org/pdf/1506.08603), §4, avoids retaining channel state for acyclic dataflows by alignment under FIFO/quasi-reliable-channel assumptions. Cycles require a different treatment with selected channel records. A current Flink archive is also acquired; its aligned/unaligned state machine is not identical to that historical paper. One read test explicitly preserves uploaded snapshot state when its acknowledgement RPC fails, because the coordinator may already reference it. An ambiguous acknowledgement is not evidence of an absent write.

[Naiad 2013](https://pages.cs.wisc.edu/~shivaram/cs744-readings/Naiad.pdf), §2, uses epochs and loop coordinates plus constraints on when vertices may send messages or request notification. The [2021 verified progress paper](https://drops.dagstuhl.de/storage/00lipics/lipics-vol193-itp2021/LIPIcs.ITP.2021.10/LIPIcs.ITP.2021.10.pdf) extends earlier clocks work to include worker-local propagation and previously implicit graph assumptions. Its proved frontiers are safe lower bounds. It explicitly stops safely under failures/stalls/channel resets rather than implementing recovery. The proof is not evidence that today’s entire Rust checkout or an event store is verified.

[Rollback recovery](https://www.cs.rice.edu/~dbj/pubs/CMU-CS-99-148.pdf), introduction, distinguishes checkpoint-only recovery from logging nondeterministic determinants; it excludes Byzantine faults. The acquired original is the 1999 technical report, not the later 2002 journal bytes. [Falkirk Wheel](https://arxiv.org/pdf/1503.08877), introduction, explores mixed recovery policies and selective rollback over logical times; the acquired v1 labels itself work in progress. Re-executing a computation, retaining all original inputs and committing irreversible external effects remain different obligations.



## Checked 2024–2026 fronts

Forward searches checked current primary paper records and originals, rather than treating recent repository commits as evidence that the bibliography was current. These are concrete fronts, not an exhaustive survey of work published through October 2026. [forward-checks.tsv](forward-checks.tsv) records the searches’ evidence boundaries.

| Recent work | Checked relation and relevance | Reading/version boundary |
|---|---|---|
| [Dynamic Datalog as higher-order DBSP, 2024](https://ceur-ws.org/Vol-3801/paper1.pdf) | Directly cites DBSP and Differential; treats rules and facts as changing inputs to a positive-Datalog interpreter | Original acquired; introduction/related work read; evaluation not independently reproduced |
| [KAIROS, S&P 2024](https://tfjmp.org/publications/2024-sp.pdf) | Direct CamFlow/CamQuery lineage; learned event anomaly scores and attack summaries over provenance streams | Original acquired; abstract/opening read; detection performance is not capture completeness |
| [Eg-walker, EuroSys 2025](https://arxiv.org/pdf/2409.14252) | A full editing-event DAG with parent IDs and deterministic replay, separate from temporary merge state | Acquired 2024 v1 preprint; §2 centrally read; text editing remains its domain |
| [Keyed watermarks, FGCS 2025](https://doi.org/10.1016/j.future.2025.107796) | Publisher describes per-key progress to address delayed substreams; lists the 2021 watermarks paper | Publisher abstract and deposited authorship checked; full text blocked; no quantitative result adopted |
| [ProHunter, March 2026](https://arxiv.org/abs/2603.19658) | Compaction, semantic abstraction and sampled threat graphs expose a recent analysis/storage frontier | Preprint original acquired; abstract/opening read; exact citation edges not yet traced |
| [ProvICS, July 2026](https://arxiv.org/abs/2607.05989) | Combines host/PLC provenance, decoded industrial traffic and physical telemetry in a hardware testbed | Preprint original acquired; abstract/intro read; dataset not acquired; time alignment alone does not establish causal completeness |
| [Fixing the Fixpoint, September 2026](https://arxiv.org/pdf/2610.00530v1) | Directly challenges a convergence-detection suggestion in DBSP and discusses Differential | v1 preprint acquired; counterexample and internal-convergence definitions/theorems centrally read; Lean artifact not acquired or run |

The last paper changes how the earlier DBSP reading must be qualified. Its §2.1 shows that a delta-of-deltas iteration can produce an empty difference and subsequently a nonempty retraction; stopping at the first zero gives a wrong transitive closure. The paper explicitly distinguishes naive/seminaive iteration, where that criterion is sound. §5 makes internal stability a sufficient condition for external convergence; the converse holds only for specified circuit classes. Its formalization reports Lean proofs, while existing Feldera implementation practice is a separate object. This is a concrete frontier around “no more changes” claims, not evidence that every incremental implementation has the reported bug.

## What the pinned implementations actually contribute

Exact commits and archive hashes are in [references.tsv](references.tsv). Paths and reading bounds are in [source-reading.tsv](source-reading.tsv). All observations below are static; no builds, application tests, benchmarks or verification tools ran.

| Reference | Read mechanism | Material boundary |
|---|---|---|
| [Timely](https://github.com/TimelyDataflow/timely-dataflow/tree/3012f8f2eb339cb8e5abd8b64ef0a46d69808393) | Minimal antichains; capability creation/downgrade/drop | Retaining a capability can stop progress; frontier tracking is not persistence |
| [Differential](https://github.com/TimelyDataflow/differential-dataflow/tree/aa8745f93ea8abe131104fc7885ba4fd47e63902) | Trace compaction; difference identities | Historical distinctions may deliberately disappear |
| [Automerge](https://github.com/automerge/automerge/tree/0ee5e2c5395adfe29ed62d5d73496410183f2a85) | Change dependencies and verified decoding | Document/change validity is distinct from durable acceptance |
| [Flink](https://github.com/apache/flink/tree/8d01380ff5bf9892462ef9808b1b56fd9723f54d) | Barrier state machine; lost-ACK test source | State upload and coordinator acknowledgement can diverge |
| [Beam](https://github.com/apache/beam/tree/6848f8454daa33e11a002c753d049e8a7384e65f) | Watermark estimator/holds; closed-window drop test | Analytical windows intentionally discard arrivals and count that loss |
| [XTDB](https://github.com/xtdb/xtdb/tree/ac64978bb67c4c10e1f70c2a9c347906d3c96b4f) | Log API, local writer, temporal vectors, failure tests | Queue order, durable receipt and query-visible temporal state are separate |
| [GProM](https://github.com/IITDBGroup/gprom/tree/3f415bc5a2d6c3b5b08987a6a68c4686f728c5f2) | Query/provenance rewrite dispatch | Lineage reconstruction depends on a suitable recorded history |
| [CamFlow libprovenance](https://github.com/CamFlow/libprovenance/tree/46255580589d1d5c751cebe960daedc4c5724b27) | Per-CPU relay readers and callbacks | This snapshot is the userspace reader; kernel capture and runtime submodule are missing |

XTDB supplies an especially concrete API contrast: `enqueueMessage` fixes partition order and returns a `Deferred` completed on durability; `appendMessage` awaits it. `LocalLog` batches, calls `FileChannel.force(true)`, then completes receipts. On a write failure it closes that partition’s queue and fails retained handles; tests inject codec failure and check later appends fail while sibling partitions continue. This reading does **not** verify full-write, crash, filesystem or hardware correctness: the read path uses a single `FileChannel.write` call, and no I/O-fault experiment was performed. Its `BitemporalConsumer` separately carries valid and system-time bounds.

CamFlow’s userspace callback path can filter records, omit unset callbacks, or report read/size errors through an optional logger. Its source archive omits `C-Thread-Pool`, and the kernel hooks are outside this capture. This is useful material for investigating capture boundaries, not evidence of end-to-end losslessness.

## Acquisition, restoration and remaining claims

Run from the repository root:

```sh
python3 papers/survey/world-state/acquire.py restore
```

This uses the recorded commit archive URLs, verifies original hashes, and extracts regular source members while removing Git/macOS/build detritus. It does not resolve moving HEAD. PDF originals are also checked against recorded hashes. An online original may move or change; hash mismatch requires investigation, not silently updating a citation. Submodules are absent and explicitly listed in the source-reading record. Nothing was installed or adopted as a dependency; no in-house source was modified.

The retained corpus supports source-grounded comparison, not a quantified nines or speed claim. A future reliability argument still needs the gate’s exact acceptance boundary, fault model, recovery behavior, retained payload and dependency scope, and workload-specific latency/throughput evidence. These are unresolved research obligations. The literature does not justify dropping arrivals to achieve a faster analytical watermark, convergent state or smaller compacted view.
