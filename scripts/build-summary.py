#!/usr/bin/env python3
"""
Build a sector-aggregated public summary from a full per-target scan JSON.

Usage:
    python3 scripts/build-summary.py \
        --input  path/to/nz-ci-scan-YYYYMMDD.json \
        --output results/YYYY-MM-DD/summary.json

The full per-target JSON produced by `nzism-pqc-audit scan --output-json` is
assumed to be private (it names individual entities). This script emits a
sanitised, sector-level-only summary that's safe to publish.

The output JSON schema is stable across scan dates so consumers can diff
summaries over time.
"""
from __future__ import annotations
import argparse
import json
import sys
from collections import Counter, defaultdict
from datetime import datetime
from pathlib import Path


def pqc(r: dict) -> bool:
    t = r.get("tls") or {}
    return bool(t.get("pqc_supported"))


def cdn(r: dict) -> bool:
    c = r.get("cdn") or {}
    return bool(c.get("detected"))


def cdn_provider(r: dict) -> str | None:
    c = r.get("cdn") or {}
    return c.get("provider") if c.get("detected") else None


def err(r: dict) -> bool:
    t = r.get("tls") or {}
    return r.get("error") is not None or not t.get("connected", True)


def build_summary(records: list[dict], scan_date: str, tool_version: str) -> dict:
    total = len(records)
    successful = sum(1 for r in records if not err(r))
    errors = total - successful

    pqc_total = sum(1 for r in records if pqc(r))
    pqc_cdn = sum(1 for r in records if pqc(r) and cdn(r))
    pqc_origin = sum(1 for r in records if pqc(r) and not cdn(r))
    nopqc_cdn = sum(1 for r in records if not pqc(r) and cdn(r) and not err(r))
    nopqc_origin = sum(1 for r in records if not pqc(r) and not cdn(r) and not err(r))
    behind_cdn = sum(1 for r in records if cdn(r))
    tls13 = sum(
        1 for r in records
        if (r.get("tls") or {}).get("tls_version") == "TLSv1_3"
    )

    sectors: dict[str, dict[str, int]] = defaultdict(
        lambda: {"total": 0, "pqc": 0, "pqc_cdn": 0, "pqc_origin": 0, "errors": 0}
    )
    for r in records:
        s = r.get("sector") or "Unknown"
        sectors[s]["total"] += 1
        if err(r):
            sectors[s]["errors"] += 1
            continue
        if pqc(r):
            sectors[s]["pqc"] += 1
            if cdn(r):
                sectors[s]["pqc_cdn"] += 1
            else:
                sectors[s]["pqc_origin"] += 1

    cdn_counts: Counter[str] = Counter()
    cdn_pqc_counts: Counter[str] = Counter()
    for r in records:
        p = cdn_provider(r)
        if p is None:
            continue
        cdn_counts[p] += 1
        if pqc(r):
            cdn_pqc_counts[p] += 1

    kex_counts = Counter(
        (r.get("tls") or {}).get("key_exchange") for r in records if not err(r)
    )
    tls_counts = Counter(
        (r.get("tls") or {}).get("tls_version") for r in records if not err(r)
    )

    def pct(n: int, d: int) -> float:
        return round(n / d, 4) if d else 0.0

    return {
        "schema_version": 1,
        "scan_date": scan_date,
        "tool_version": tool_version,
        "targets_scanned": total,
        "successful": successful,
        "errors": errors,
        "totals": {
            "pqc_negotiated": pqc_total,
            "pqc_rate": pct(pqc_total, successful),
            "tls_1_3": tls13,
            "tls_1_3_rate": pct(tls13, successful),
            "behind_cdn": behind_cdn,
            "cdn_rate": pct(behind_cdn, total),
        },
        "split": {
            "pqc_via_cdn": pqc_cdn,
            "pqc_via_origin": pqc_origin,
            "no_pqc_behind_cdn": nopqc_cdn,
            "no_pqc_origin": nopqc_origin,
        },
        "sectors": dict(sorted(sectors.items())),
        "cdn_providers": {
            p: {"fronted": cdn_counts[p], "pqc": cdn_pqc_counts[p]}
            for p in sorted(cdn_counts)
        },
        "key_exchange_groups": dict(kex_counts.most_common()),
        "tls_versions": dict(tls_counts.most_common()),
    }


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--input", required=True, help="Full per-target scan JSON")
    ap.add_argument("--output", required=True, help="Sector-aggregated summary JSON")
    ap.add_argument(
        "--scan-date",
        default=None,
        help="ISO date for the scan (default: from filename or today)",
    )
    ap.add_argument(
        "--tool-version", default="0.1.0", help="Tool version producing the scan"
    )
    args = ap.parse_args()

    src = Path(args.input)
    if not src.exists():
        print(f"error: input not found: {src}", file=sys.stderr)
        return 2

    records = json.loads(src.read_text())
    if not isinstance(records, list):
        print("error: expected a JSON array of scan records", file=sys.stderr)
        return 2

    scan_date = args.scan_date or datetime.now().strftime("%Y-%m-%d")
    summary = build_summary(records, scan_date, args.tool_version)

    out = Path(args.output)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(summary, indent=2) + "\n")
    print(f"wrote {out}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
