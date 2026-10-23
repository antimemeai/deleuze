#!/usr/bin/env python3
"""Rebuild the navigation catalog from lane-owned research bibliographies.

Rows are citation entries, not deduplicated papers or a claim that each was read.
The original lane bibliography remains authoritative for additional provenance.
"""
import csv
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
INPUTS = {
    "durable-logs": "durable-logs/bibliography.tsv",
    "fast-capture": "fast-capture/bibliography.tsv",
    "fast-capture-recent": "fast-capture/recent-bibliography.tsv",
    "world-state": "world-state/bibliography.tsv",
    "verifiability": "verifiability/bibliography.tsv",
    "integrity": "integrity/bibliography.tsv",
    "measurement": "measurement/bibliography.tsv",
}
ALIASES = {
    "id": ("id",), "title": ("title",), "authors": ("authors",),
    "year": ("year",), "venue": ("venue",), "doi": ("doi", "DOI"),
    "primary_url": ("primary_url", "primaryURL", "url"),
    "local_original": ("local_original", "localOriginal", "original_path", "original"),
    "sha256": ("sha256",),
    "acquisition": ("acquisition", "acquisition_status", "access"),
    "reading_status": ("reading_status", "readingStatus"),
    "notes": ("notes", "reading_notes", "edition_note"),
}


def main():
    rows = []
    for lane, relative in INPUTS.items():
        path = ROOT / "papers/survey" / relative
        if not path.is_file():
            raise SystemExit(f"Missing bibliography: {path}")
        with path.open(newline="") as source:
            reader = csv.DictReader(source, delimiter="\t")
            if "id" not in (reader.fieldnames or ()):
                raise SystemExit(f"Missing id column: {path}")
            for original in reader:
                row = {"lane": lane, "bibliography": str(path.relative_to(ROOT))}
                for field, candidates in ALIASES.items():
                    values = [original[key] for key in candidates if original.get(key)]
                    row[field] = "; ".join(values) if field == "notes" else next(iter(values), "")
                if not row["id"] or not row["title"]:
                    raise SystemExit(f"Incomplete citation in {path}: {row['id']}")
                rows.append(row)
    target = ROOT / "papers/survey/catalog.tsv"
    with target.open("w", newline="") as output:
        writer = csv.DictWriter(output, fieldnames=["lane", "bibliography", *ALIASES],
                                delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    for lane, count in Counter(row["lane"] for row in rows).items():
        print(f"{lane}: {count} entries")
    print(f"Total: {len(rows)} entries (duplicates between lanes retained)")


if __name__ == "__main__":
    main()
