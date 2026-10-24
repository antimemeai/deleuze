# Public database batching source acquisitions

2026-10-23. Selected-file snapshots only, for reading. No dependency or production protocol is adopted. Public source URLs are pinned to the resolved upstream HEAD revisions below; these are not claimed to be the exact implementations evaluated in older papers. Original file bytes are retained directly, so there is no Git metadata to remove. Unlisted source paths/build closures are unacquired and unreviewed.

## postgresql: COPYRIGHT

- Repository: https://github.com/postgres/postgres
- Revision: `126976a8ffcfa47f40d3df09243ebf4606001912`
- Authors: PostgreSQL Global Development Group
- License: PostgreSQL License; retained notices govern
- Snapshot: `quarantine/sources/adaptive-batching/postgresql/COPYRIGHT` (1,198 bytes)
- SHA-256: `3d6af92ff8a4c2cdf69afb1cf44edea727922f5cd0cf8b5f72b11cdecac8fdfd`
- Source: https://raw.githubusercontent.com/postgres/postgres/126976a8ffcfa47f40d3df09243ebf4606001912/COPYRIGHT

```sh
mkdir -p 'quarantine/sources/adaptive-batching/postgresql'
curl -fL 'https://raw.githubusercontent.com/postgres/postgres/126976a8ffcfa47f40d3df09243ebf4606001912/COPYRIGHT' -o 'quarantine/sources/adaptive-batching/postgresql/COPYRIGHT'
printf '%s  %s\n' '3d6af92ff8a4c2cdf69afb1cf44edea727922f5cd0cf8b5f72b11cdecac8fdfd' 'quarantine/sources/adaptive-batching/postgresql/COPYRIGHT' | shasum -a 256 -c -
```

## postgresql: src/backend/access/transam/xlog.c

- Repository: https://github.com/postgres/postgres
- Revision: `126976a8ffcfa47f40d3df09243ebf4606001912`
- Authors: PostgreSQL Global Development Group
- License: PostgreSQL License; retained notices govern
- Snapshot: `quarantine/sources/adaptive-batching/postgresql/src/backend/access/transam/xlog.c` (353,358 bytes)
- SHA-256: `0d39e5891c67fc19dd2fef8635e3bc87a31a052f0c42ed2b54eb3fe6157806ce`
- Source: https://raw.githubusercontent.com/postgres/postgres/126976a8ffcfa47f40d3df09243ebf4606001912/src/backend/access/transam/xlog.c

```sh
mkdir -p 'quarantine/sources/adaptive-batching/postgresql/src/backend/access/transam'
curl -fL 'https://raw.githubusercontent.com/postgres/postgres/126976a8ffcfa47f40d3df09243ebf4606001912/src/backend/access/transam/xlog.c' -o 'quarantine/sources/adaptive-batching/postgresql/src/backend/access/transam/xlog.c'
printf '%s  %s\n' '0d39e5891c67fc19dd2fef8635e3bc87a31a052f0c42ed2b54eb3fe6157806ce' 'quarantine/sources/adaptive-batching/postgresql/src/backend/access/transam/xlog.c' | shasum -a 256 -c -
```

## postgresql: src/backend/storage/lmgr/lwlock.c

- Repository: https://github.com/postgres/postgres
- Revision: `126976a8ffcfa47f40d3df09243ebf4606001912`
- Authors: PostgreSQL Global Development Group
- License: PostgreSQL License; retained notices govern
- Snapshot: `quarantine/sources/adaptive-batching/postgresql/src/backend/storage/lmgr/lwlock.c` (54,976 bytes)
- SHA-256: `8880afe5fbe1a64225742900f9f002d0c28e2ea461fc7396c0517960dde15338`
- Source: https://raw.githubusercontent.com/postgres/postgres/126976a8ffcfa47f40d3df09243ebf4606001912/src/backend/storage/lmgr/lwlock.c

```sh
mkdir -p 'quarantine/sources/adaptive-batching/postgresql/src/backend/storage/lmgr'
curl -fL 'https://raw.githubusercontent.com/postgres/postgres/126976a8ffcfa47f40d3df09243ebf4606001912/src/backend/storage/lmgr/lwlock.c' -o 'quarantine/sources/adaptive-batching/postgresql/src/backend/storage/lmgr/lwlock.c'
printf '%s  %s\n' '8880afe5fbe1a64225742900f9f002d0c28e2ea461fc7396c0517960dde15338' 'quarantine/sources/adaptive-batching/postgresql/src/backend/storage/lmgr/lwlock.c' | shasum -a 256 -c -
```

## postgresql: src/backend/utils/misc/guc_tables.c

- Repository: https://github.com/postgres/postgres
- Revision: `126976a8ffcfa47f40d3df09243ebf4606001912`
- Authors: PostgreSQL Global Development Group
- License: PostgreSQL License; retained notices govern
- Snapshot: `quarantine/sources/adaptive-batching/postgresql/src/backend/utils/misc/guc_tables.c` (26,135 bytes)
- SHA-256: `23906c4ba2e43015bf0178d7e1bbf4544d17f91858b79b7dd769f8a9f6ee3bdd`
- Source: https://raw.githubusercontent.com/postgres/postgres/126976a8ffcfa47f40d3df09243ebf4606001912/src/backend/utils/misc/guc_tables.c

```sh
mkdir -p 'quarantine/sources/adaptive-batching/postgresql/src/backend/utils/misc'
curl -fL 'https://raw.githubusercontent.com/postgres/postgres/126976a8ffcfa47f40d3df09243ebf4606001912/src/backend/utils/misc/guc_tables.c' -o 'quarantine/sources/adaptive-batching/postgresql/src/backend/utils/misc/guc_tables.c'
printf '%s  %s\n' '23906c4ba2e43015bf0178d7e1bbf4544d17f91858b79b7dd769f8a9f6ee3bdd' 'quarantine/sources/adaptive-batching/postgresql/src/backend/utils/misc/guc_tables.c' | shasum -a 256 -c -
```

## rocksdb: LICENSE.Apache

- Repository: https://github.com/facebook/rocksdb
- Revision: `ea313605367281013c2c81158069e88411516357`
- Authors: RocksDB contributors
- License: Apache-2.0 OR GPL-2.0 per retained root licenses; retained notices govern
- Snapshot: `quarantine/sources/adaptive-batching/rocksdb/LICENSE.Apache` (11,358 bytes)
- SHA-256: `cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30`
- Source: https://raw.githubusercontent.com/facebook/rocksdb/ea313605367281013c2c81158069e88411516357/LICENSE.Apache

```sh
mkdir -p 'quarantine/sources/adaptive-batching/rocksdb'
curl -fL 'https://raw.githubusercontent.com/facebook/rocksdb/ea313605367281013c2c81158069e88411516357/LICENSE.Apache' -o 'quarantine/sources/adaptive-batching/rocksdb/LICENSE.Apache'
printf '%s  %s\n' 'cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30' 'quarantine/sources/adaptive-batching/rocksdb/LICENSE.Apache' | shasum -a 256 -c -
```

## rocksdb: LICENSE.leveldb

- Repository: https://github.com/facebook/rocksdb
- Revision: `ea313605367281013c2c81158069e88411516357`
- Authors: RocksDB contributors
- License: Apache-2.0 OR GPL-2.0 per retained root licenses; retained notices govern
- Snapshot: `quarantine/sources/adaptive-batching/rocksdb/LICENSE.leveldb` (1,572 bytes)
- SHA-256: `b568f1f37a5a1d37a3e981e7c8b7fc4856ca33b2ca0a45bc8e847aaaf7afe485`
- Source: https://raw.githubusercontent.com/facebook/rocksdb/ea313605367281013c2c81158069e88411516357/LICENSE.leveldb

```sh
mkdir -p 'quarantine/sources/adaptive-batching/rocksdb'
curl -fL 'https://raw.githubusercontent.com/facebook/rocksdb/ea313605367281013c2c81158069e88411516357/LICENSE.leveldb' -o 'quarantine/sources/adaptive-batching/rocksdb/LICENSE.leveldb'
printf '%s  %s\n' 'b568f1f37a5a1d37a3e981e7c8b7fc4856ca33b2ca0a45bc8e847aaaf7afe485' 'quarantine/sources/adaptive-batching/rocksdb/LICENSE.leveldb' | shasum -a 256 -c -
```

## rocksdb: COPYING

- Repository: https://github.com/facebook/rocksdb
- Revision: `ea313605367281013c2c81158069e88411516357`
- Authors: RocksDB contributors
- License: Apache-2.0 OR GPL-2.0 per retained root licenses; retained notices govern
- Snapshot: `quarantine/sources/adaptive-batching/rocksdb/COPYING` (18,092 bytes)
- SHA-256: `8177f97513213526df2cf6184d8ff986c675afb514d4e68a404010521b880643`
- Source: https://raw.githubusercontent.com/facebook/rocksdb/ea313605367281013c2c81158069e88411516357/COPYING

```sh
mkdir -p 'quarantine/sources/adaptive-batching/rocksdb'
curl -fL 'https://raw.githubusercontent.com/facebook/rocksdb/ea313605367281013c2c81158069e88411516357/COPYING' -o 'quarantine/sources/adaptive-batching/rocksdb/COPYING'
printf '%s  %s\n' '8177f97513213526df2cf6184d8ff986c675afb514d4e68a404010521b880643' 'quarantine/sources/adaptive-batching/rocksdb/COPYING' | shasum -a 256 -c -
```

## rocksdb: db/write_thread.cc

- Repository: https://github.com/facebook/rocksdb
- Revision: `ea313605367281013c2c81158069e88411516357`
- Authors: RocksDB contributors
- License: Apache-2.0 OR GPL-2.0 per retained root licenses; retained notices govern
- Snapshot: `quarantine/sources/adaptive-batching/rocksdb/db/write_thread.cc` (36,295 bytes)
- SHA-256: `c66f450d80a8c2f39f8e358d5f81c152220a25d91d1270bb07eadfb6e18d80db`
- Source: https://raw.githubusercontent.com/facebook/rocksdb/ea313605367281013c2c81158069e88411516357/db/write_thread.cc

```sh
mkdir -p 'quarantine/sources/adaptive-batching/rocksdb/db'
curl -fL 'https://raw.githubusercontent.com/facebook/rocksdb/ea313605367281013c2c81158069e88411516357/db/write_thread.cc' -o 'quarantine/sources/adaptive-batching/rocksdb/db/write_thread.cc'
printf '%s  %s\n' 'c66f450d80a8c2f39f8e358d5f81c152220a25d91d1270bb07eadfb6e18d80db' 'quarantine/sources/adaptive-batching/rocksdb/db/write_thread.cc' | shasum -a 256 -c -
```

## rocksdb: db/write_thread.h

- Repository: https://github.com/facebook/rocksdb
- Revision: `ea313605367281013c2c81158069e88411516357`
- Authors: RocksDB contributors
- License: Apache-2.0 OR GPL-2.0 per retained root licenses; retained notices govern
- Snapshot: `quarantine/sources/adaptive-batching/rocksdb/db/write_thread.h` (20,178 bytes)
- SHA-256: `ffd8b38db25f8b03a1b39b63a7d012f1f2bc5095061f42d66d7df090c0820dfa`
- Source: https://raw.githubusercontent.com/facebook/rocksdb/ea313605367281013c2c81158069e88411516357/db/write_thread.h

```sh
mkdir -p 'quarantine/sources/adaptive-batching/rocksdb/db'
curl -fL 'https://raw.githubusercontent.com/facebook/rocksdb/ea313605367281013c2c81158069e88411516357/db/write_thread.h' -o 'quarantine/sources/adaptive-batching/rocksdb/db/write_thread.h'
printf '%s  %s\n' 'ffd8b38db25f8b03a1b39b63a7d012f1f2bc5095061f42d66d7df090c0820dfa' 'quarantine/sources/adaptive-batching/rocksdb/db/write_thread.h' | shasum -a 256 -c -
```

## rocksdb: include/rocksdb/options.h

- Repository: https://github.com/facebook/rocksdb
- Revision: `ea313605367281013c2c81158069e88411516357`
- Authors: RocksDB contributors
- License: Apache-2.0 OR GPL-2.0 per retained root licenses; retained notices govern
- Snapshot: `quarantine/sources/adaptive-batching/rocksdb/include/rocksdb/options.h` (164,349 bytes)
- SHA-256: `ace3658a2101349a6478a5dc094c4695e2af88e3833642ac7829d9e1b9e88ead`
- Source: https://raw.githubusercontent.com/facebook/rocksdb/ea313605367281013c2c81158069e88411516357/include/rocksdb/options.h

```sh
mkdir -p 'quarantine/sources/adaptive-batching/rocksdb/include/rocksdb'
curl -fL 'https://raw.githubusercontent.com/facebook/rocksdb/ea313605367281013c2c81158069e88411516357/include/rocksdb/options.h' -o 'quarantine/sources/adaptive-batching/rocksdb/include/rocksdb/options.h'
printf '%s  %s\n' 'ace3658a2101349a6478a5dc094c4695e2af88e3833642ac7829d9e1b9e88ead' 'quarantine/sources/adaptive-batching/rocksdb/include/rocksdb/options.h' | shasum -a 256 -c -
```

## oceanbase: LICENSE

- Repository: https://github.com/oceanbase/oceanbase
- Revision: `5e8a63d29dae9fde5e962b8fcc7b167d438fc343`
- Authors: OceanBase contributors
- License: Apache-2.0 (retained LICENSE and source notices); retained notices govern
- Snapshot: `quarantine/sources/adaptive-batching/oceanbase/LICENSE` (11,356 bytes)
- SHA-256: `43070e2d4e532684de521b885f385d0841030efa2b1a20bafb76133a5e1379c1`
- Source: https://raw.githubusercontent.com/oceanbase/oceanbase/5e8a63d29dae9fde5e962b8fcc7b167d438fc343/LICENSE

```sh
mkdir -p 'quarantine/sources/adaptive-batching/oceanbase'
curl -fL 'https://raw.githubusercontent.com/oceanbase/oceanbase/5e8a63d29dae9fde5e962b8fcc7b167d438fc343/LICENSE' -o 'quarantine/sources/adaptive-batching/oceanbase/LICENSE'
printf '%s  %s\n' '43070e2d4e532684de521b885f385d0841030efa2b1a20bafb76133a5e1379c1' 'quarantine/sources/adaptive-batching/oceanbase/LICENSE' | shasum -a 256 -c -
```

## oceanbase: src/logservice/palf/log_sliding_window.cpp

- Repository: https://github.com/oceanbase/oceanbase
- Revision: `5e8a63d29dae9fde5e962b8fcc7b167d438fc343`
- Authors: OceanBase contributors
- License: Apache-2.0 (retained LICENSE and source notices); retained notices govern
- Snapshot: `quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf/log_sliding_window.cpp` (215,430 bytes)
- SHA-256: `0b03396b878505fefb1999af7caa496f5d802d10430eb778adf6f71f1110715e`
- Source: https://raw.githubusercontent.com/oceanbase/oceanbase/5e8a63d29dae9fde5e962b8fcc7b167d438fc343/src/logservice/palf/log_sliding_window.cpp

```sh
mkdir -p 'quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf'
curl -fL 'https://raw.githubusercontent.com/oceanbase/oceanbase/5e8a63d29dae9fde5e962b8fcc7b167d438fc343/src/logservice/palf/log_sliding_window.cpp' -o 'quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf/log_sliding_window.cpp'
printf '%s  %s\n' '0b03396b878505fefb1999af7caa496f5d802d10430eb778adf6f71f1110715e' 'quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf/log_sliding_window.cpp' | shasum -a 256 -c -
```

## oceanbase: src/logservice/palf/log_sliding_window.h

- Repository: https://github.com/oceanbase/oceanbase
- Revision: `5e8a63d29dae9fde5e962b8fcc7b167d438fc343`
- Authors: OceanBase contributors
- License: Apache-2.0 (retained LICENSE and source notices); retained notices govern
- Snapshot: `quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf/log_sliding_window.h` (27,531 bytes)
- SHA-256: `d1bea991d1f3678f82e25ca4c8ee488aed890acb1df66b6e781a75eaf24209ed`
- Source: https://raw.githubusercontent.com/oceanbase/oceanbase/5e8a63d29dae9fde5e962b8fcc7b167d438fc343/src/logservice/palf/log_sliding_window.h

```sh
mkdir -p 'quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf'
curl -fL 'https://raw.githubusercontent.com/oceanbase/oceanbase/5e8a63d29dae9fde5e962b8fcc7b167d438fc343/src/logservice/palf/log_sliding_window.h' -o 'quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf/log_sliding_window.h'
printf '%s  %s\n' 'd1bea991d1f3678f82e25ca4c8ee488aed890acb1df66b6e781a75eaf24209ed' 'quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf/log_sliding_window.h' | shasum -a 256 -c -
```

## oceanbase: src/logservice/palf/log_group_buffer.cpp

- Repository: https://github.com/oceanbase/oceanbase
- Revision: `5e8a63d29dae9fde5e962b8fcc7b167d438fc343`
- Authors: OceanBase contributors
- License: Apache-2.0 (retained LICENSE and source notices); retained notices govern
- Snapshot: `quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf/log_group_buffer.cpp` (21,449 bytes)
- SHA-256: `94b244336b6ab0a554b62f0ae572fa27bd4f897bf90d38ffb1737820d077bae1`
- Source: https://raw.githubusercontent.com/oceanbase/oceanbase/5e8a63d29dae9fde5e962b8fcc7b167d438fc343/src/logservice/palf/log_group_buffer.cpp

```sh
mkdir -p 'quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf'
curl -fL 'https://raw.githubusercontent.com/oceanbase/oceanbase/5e8a63d29dae9fde5e962b8fcc7b167d438fc343/src/logservice/palf/log_group_buffer.cpp' -o 'quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf/log_group_buffer.cpp'
printf '%s  %s\n' '94b244336b6ab0a554b62f0ae572fa27bd4f897bf90d38ffb1737820d077bae1' 'quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf/log_group_buffer.cpp' | shasum -a 256 -c -
```

## oceanbase: src/logservice/palf/log_io_worker.cpp

- Repository: https://github.com/oceanbase/oceanbase
- Revision: `5e8a63d29dae9fde5e962b8fcc7b167d438fc343`
- Authors: OceanBase contributors
- License: Apache-2.0 (retained LICENSE and source notices); retained notices govern
- Snapshot: `quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf/log_io_worker.cpp` (20,217 bytes)
- SHA-256: `5c99b77bfa2212472d3c3ec7f23b2433c35ec67dc56a1a7bda403cc8c975d236`
- Source: https://raw.githubusercontent.com/oceanbase/oceanbase/5e8a63d29dae9fde5e962b8fcc7b167d438fc343/src/logservice/palf/log_io_worker.cpp

```sh
mkdir -p 'quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf'
curl -fL 'https://raw.githubusercontent.com/oceanbase/oceanbase/5e8a63d29dae9fde5e962b8fcc7b167d438fc343/src/logservice/palf/log_io_worker.cpp' -o 'quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf/log_io_worker.cpp'
printf '%s  %s\n' '5c99b77bfa2212472d3c3ec7f23b2433c35ec67dc56a1a7bda403cc8c975d236' 'quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf/log_io_worker.cpp' | shasum -a 256 -c -
```

## oceanbase: src/logservice/palf/log_io_worker.h

- Repository: https://github.com/oceanbase/oceanbase
- Revision: `5e8a63d29dae9fde5e962b8fcc7b167d438fc343`
- Authors: OceanBase contributors
- License: Apache-2.0 (retained LICENSE and source notices); retained notices govern
- Snapshot: `quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf/log_io_worker.h` (5,690 bytes)
- SHA-256: `5258e20aa422916573906d6bc3bc023be7c0b7420baeff88e4f1b25810be8485`
- Source: https://raw.githubusercontent.com/oceanbase/oceanbase/5e8a63d29dae9fde5e962b8fcc7b167d438fc343/src/logservice/palf/log_io_worker.h

```sh
mkdir -p 'quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf'
curl -fL 'https://raw.githubusercontent.com/oceanbase/oceanbase/5e8a63d29dae9fde5e962b8fcc7b167d438fc343/src/logservice/palf/log_io_worker.h' -o 'quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf/log_io_worker.h'
printf '%s  %s\n' '5258e20aa422916573906d6bc3bc023be7c0b7420baeff88e4f1b25810be8485' 'quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf/log_io_worker.h' | shasum -a 256 -c -
```

## oceanbase: src/logservice/palf/log_define.h

- Repository: https://github.com/oceanbase/oceanbase
- Revision: `5e8a63d29dae9fde5e962b8fcc7b167d438fc343`
- Authors: OceanBase contributors
- License: Apache-2.0 (retained LICENSE and source notices); retained notices govern
- Snapshot: `quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf/log_define.h` (15,636 bytes)
- SHA-256: `ec580261db1936940505cace2a3f4f6246dc67a889c6cc1a763bd825325ebb28`
- Source: https://raw.githubusercontent.com/oceanbase/oceanbase/5e8a63d29dae9fde5e962b8fcc7b167d438fc343/src/logservice/palf/log_define.h

```sh
mkdir -p 'quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf'
curl -fL 'https://raw.githubusercontent.com/oceanbase/oceanbase/5e8a63d29dae9fde5e962b8fcc7b167d438fc343/src/logservice/palf/log_define.h' -o 'quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf/log_define.h'
printf '%s  %s\n' 'ec580261db1936940505cace2a3f4f6246dc67a889c6cc1a763bd825325ebb28' 'quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf/log_define.h' | shasum -a 256 -c -
```

## Additional originals and caller paths

### stout.pdf

John C. McCullough; John Dunagan; Alec Wolman; Alex C. Snoeren. 2010 USENIX Annual Technical Conference. Stout: An Adaptive Interface to Scalable Cloud Storage. Original research paper; original copyright/permission notice governs; no software license inferred.

- URL: https://www.microsoft.com/en-us/research/wp-content/uploads/2016/12/usenix-2010-stout.pdf
- Original: `quarantine/sources/adaptive-batching/database-literature/stout.pdf` (1,221,978 bytes)
- SHA-256: `e7f82c5bac9e50409c67c998866dcff6bcc538a453d25e24735aa1b3aabcbb3c`

```sh
mkdir -p 'quarantine/sources/adaptive-batching/database-literature'
curl -fL 'https://www.microsoft.com/en-us/research/wp-content/uploads/2016/12/usenix-2010-stout.pdf' -o 'quarantine/sources/adaptive-batching/database-literature/stout.pdf'
printf '%s  %s\n' 'e7f82c5bac9e50409c67c998866dcff6bcc538a453d25e24735aa1b3aabcbb3c' 'quarantine/sources/adaptive-batching/database-literature/stout.pdf' | shasum -a 256 -c -
```

### log_loop_thread.cpp

OceanBase contributors. Revision `5e8a63d29dae9fde5e962b8fcc7b167d438fc343`, Apache-2.0; source and root notices retained.

- URL: https://raw.githubusercontent.com/oceanbase/oceanbase/5e8a63d29dae9fde5e962b8fcc7b167d438fc343/src/logservice/palf/log_loop_thread.cpp
- Original: `quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf/log_loop_thread.cpp` (6,005 bytes)
- SHA-256: `7b4cfc528dd0658ab172705a4cc80c3f857e1d7402b21fd83ff31953ed7a84b7`

```sh
mkdir -p 'quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf'
curl -fL 'https://raw.githubusercontent.com/oceanbase/oceanbase/5e8a63d29dae9fde5e962b8fcc7b167d438fc343/src/logservice/palf/log_loop_thread.cpp' -o 'quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf/log_loop_thread.cpp'
printf '%s  %s\n' '7b4cfc528dd0658ab172705a4cc80c3f857e1d7402b21fd83ff31953ed7a84b7' 'quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf/log_loop_thread.cpp' | shasum -a 256 -c -
```

### palf_handle_impl.cpp

OceanBase contributors. Revision `5e8a63d29dae9fde5e962b8fcc7b167d438fc343`, Apache-2.0; source and root notices retained.

- URL: https://raw.githubusercontent.com/oceanbase/oceanbase/5e8a63d29dae9fde5e962b8fcc7b167d438fc343/src/logservice/palf/palf_handle_impl.cpp
- Original: `quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf/palf_handle_impl.cpp` (262,461 bytes)
- SHA-256: `7608f9730699f52633f182e86b75973b900ae291a7fd34b77f033426997f35c3`

```sh
mkdir -p 'quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf'
curl -fL 'https://raw.githubusercontent.com/oceanbase/oceanbase/5e8a63d29dae9fde5e962b8fcc7b167d438fc343/src/logservice/palf/palf_handle_impl.cpp' -o 'quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf/palf_handle_impl.cpp'
printf '%s  %s\n' '7608f9730699f52633f182e86b75973b900ae291a7fd34b77f033426997f35c3' 'quarantine/sources/adaptive-batching/oceanbase/src/logservice/palf/palf_handle_impl.cpp' | shasum -a 256 -c -
```

### postgresql18-wal.html

PostgreSQL Global Development Group. PostgreSQL 18 documentation, observed 2026-10-23; mutable publisher page, separately versioned from acquired development source. PostgreSQL documentation copyright/license applies.

- URL: https://www.postgresql.org/docs/18/wal-configuration.html
- Original: `quarantine/sources/adaptive-batching/database-literature/postgresql18-wal.html` (37,576 bytes)
- SHA-256: `731aeb515577e12531d70716ff702497d5bfdde8f62348cde702252013b725db`

```sh
mkdir -p 'quarantine/sources/adaptive-batching/database-literature'
curl -fL 'https://www.postgresql.org/docs/18/wal-configuration.html' -o 'quarantine/sources/adaptive-batching/database-literature/postgresql18-wal.html'
printf '%s  %s\n' '731aeb515577e12531d70716ff702497d5bfdde8f62348cde702252013b725db' 'quarantine/sources/adaptive-batching/database-literature/postgresql18-wal.html' | shasum -a 256 -c -
```

## Existing acquisitions reused without duplicate copies

Original restoration commands and rights notices remain in [durable log manifest](../survey/durable-logs/references-manifest.md), [durable bibliography](../survey/durable-logs/bibliography.tsv), and [first reference manifest](external-manifest.md). This lane read Aether §4.1, PALF §6 and §8.2 plus client-model context, and LevelDB `DBImpl::BuildBatchGroup`. Earlier reading status in the durable bibliography records the earlier acquisition checkpoint; the PALF reading here is new.

| Work | Citation and source | Existing original | SHA-256 |
| --- | --- | --- | --- |
| Aether: A Scalable Approach to Logging | Ryan Johnson; Ippokratis Pandis; Radu Stoica; Manos Athanassoulis; Anastasia Ailamaki. 2010. PVLDB 3(1–2). https://www.vldb.org/pvldb/vol3/R61.pdf | `quarantine/archives/survey-durable/documents/aether.pdf` | `9c0b5aa69388abd7b381edb969334e1deb646240e19c54e1983a250db8b05c5e` |
| PALF: Replicated Write-Ahead Logging for Distributed Databases | Fusheng Han; Hao Liu; Bin Chen; Debin Jia; Jianfeng Zhou; Xuwang Teng; Chuanhui Yang; Huafeng Xi; Wei Tian; Shuning Tao; Sen Wang; Quanqing Xu; Zhenkun Yang. 2024. PVLDB 17(12). https://www.vldb.org/pvldb/vol17/p3745-xu.pdf | `quarantine/archives/survey-durable/documents/palf.pdf` | `f09909ce10bf2bab81899e0ec09461f42b24f5bff41953c9d570b66948de48b2` |
| LevelDB source | Google/LevelDB contributors. Revision `bb74ef739973a70ca9f0d90788a1da89b788c0ad`. BSD-3-Clause. https://github.com/google/leveldb/blob/bb74ef739973a70ca9f0d90788a1da89b788c0ad/db/db_impl.cc | `quarantine/sources/external/leveldb/db/db_impl.cc` | `8ec9ebbd813dfbb4186993154f1d7c392b62926f3b36f356ffd26f3fd0006a81` |

## Reading and acquisition bounds

Selected source sections read: PostgreSQL `XLogFlush` and associated lock-wait comments; RocksDB `EnterAsBatchGroupLeader`, `AwaitState`, and batching/wait option comments; OceanBase `check_and_switch_freeze_mode`, feedback/periodic freeze routines, append counter increments, loop scheduling and handle forwarding; Stout §3.1–3.2 and §4.3, with evaluation/hiccup context. Acquired ancillary functions/files outside these sections are not represented as fully reviewed. No source was compiled or performance experiment reproduced. Derived Stout text can be recreated with `pdftotext -layout <original.pdf> <output.txt>`; original is the source of truth.

Stout publisher PDF URLs returned HTTP 403 to the direct downloader; the author-hosted Microsoft Research original above succeeded. Author/publisher publication pages expose paper/slides, but no corresponding public Stout implementation was identified in this bounded search. No missing original is claimed acquired. The printed Stout gain equation and its accompanying upper-bound prose disagree; no correction is inferred or implemented. Database papers/code are references for study only.
