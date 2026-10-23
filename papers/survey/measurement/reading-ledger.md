# Reading ledger

Acquired 2026-10-23. A section read is not a whole-paper review; proofs and experimental artifacts require further review where stated. Derived text is under `quarantine/sources/survey-measurement/literature/`. No code was executed except acquisition, text extraction and byte verification.

| Work | Read depth in this wave | Still open |
|---|---|---|
| Open Versus Closed | Core: abstract, §§1–2; §5.1 principles and explanations; selected workload-method descriptions | Full scheduling analyses, partly-open calibration, re-evaluation on capture workloads |
| Tail at Scale | Selected: variability/fanout discussion, hedged/tied requests, good-enough IR, mutation discussion | No numerical reproduction or whole-system transfer claim |
| Tene London 2013 talk | Partial presentation text: agenda and selected introductory assumptions/requirements material; provider exposes text through slide 100 of 254 | Full deck PDF and remaining slides; video not watched |
| wrk2 README | Core: schedule-relative latency, coordinated-omission discussion, precision/calibration limitations | Full socket-error and reconnect model |
| HdrHistogram README | Core: precision/range, concurrency and raw versus corrected values | Distribution estimator validation and other language ports |
| TailBench | Core §IV: traffic shaper, queue, three configurations, statistics/methodology; abstract/introduction | All application evaluations and confidence-interval implementation |
| Producing Wrong Data | Selected §§1, 3.1–3.3 and §7: layout bias, experimental setup and mitigation | All results/figures; no independent repetition |
| Rigorous Benchmarking | Core §§3–4 and §6: levels, effect size, initialization/independence; selected recommendations | Full variance-cost derivation, estimator implementation, applicability to dependent tails |
| Scientific Benchmarking | Core §4.1–4.2: setup, warm/cold cache, timing perturbation, synchronization and block versus event measures; selected §3 summaries | Full statistical derivations and HPC experiments |
| Metastable Failures | Core §§1–2.3: state model, retries/cache/error logging examples | Complete prevention taxonomy and source artifacts |
| Metastable in the Wild | Selected §§1, 2.1 and 3.1–3.3: study limits, load/capacity/amplification model, GC/recovery discussion | Case-study reproduction, appendices/model proofs |
| Little 1961 | Selected theorem assumptions, population discussion and rejected units | Proof not independently checked; modern finite-window/sample-path extensions |
| Network Calculus | Selected chapter 1 definitions 1.1.1/1.3.1, arrival-curve discussion, theorem statements 1.4.1–2 | Proofs not checked, no inferred universal service guarantee; rest of book unread |
| NUMA memory policy v6.14 | Selected scope, task/VMA future-allocation and cpuset interaction sections | Measurement on target machines, other OSes/architectures |
| Ftrace duration | Selected §§1–2 and §4: observer path, filtering/discard, historical overhead setup | Raw author measurements unavailable in paper; no extrapolation to current tracing |
| TailBench++ | Preprint abstract, lifecycle/method descriptions; publisher 2026 abstract/intro/lifecycle discussion via primary page; selected harness code below | Published revision's full experiment sections; source-to-journal exact release correspondence |
| MSF-Model | Abstract/version/publication metadata only | Model, parameter assumptions, evaluation and source |
| Characterizing Metastable Faults | Selected abstract/§1–2 motivation, retry-storm model and critique of overload-only characterization | Causal definitions, proofs, scheduling methodology and Nyx implementation |

Inspected source at pinned revisions:

- **wrk2** `src/wrk.c`: startup/calibration, per-connection throughput/catch-up, `usec_to_next_send`, `response_complete`, `check_timeouts`, `time_us`; `src/wrk.h` connection state. `time_us` uses `gettimeofday`; clock-adjustment behavior is untested here. Histograms sample response callbacks, not an independently recovered all-arrival ledger.
- **HdrHistogram** `AbstractHistogram.java`: recording entry points, expected-interval correction loop, count/single-value branches and mutually exclusive recording-time/post-hoc correction documentation. README concurrency is not a proof of every variant's implementation.
- **TailBench++** `harness/client.cpp`: planned arrival creation, variable-QPS reset, late flag/warning, `finiReq` timing; `tbench_client_networked.cpp`: independent sender/receiver loops and termination limits; `msgs.h` timestamp fields. No claim that the complete artifact has been reviewed.
