"""Read-only, same-size PNG comparison. Never resizes or alters either image.

Requires Pillow supplied by the caller's Python environment. Does not install it.
--require-exact is the strict pixel-identity gate; a report alone is not a pass.
"""
import argparse
import hashlib
import json
from pathlib import Path

try:
    from PIL import Image, ImageChops
except ImportError as error:
    raise SystemExit("Pillow is required in the selected Python environment; no tools were installed.") from error


def fingerprint(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def difference(reference, candidate):
    diff = ImageChops.difference(reference, candidate)
    histogram = diff.histogram()
    count = reference.width * reference.height
    total = sum((i % 256) * n for i, n in enumerate(histogram))
    red, green, blue = diff.split()
    maximum = ImageChops.lighter(ImageChops.lighter(red, green), blue)
    equal = maximum.histogram()[0]
    return {
        "exact_equal_pixels": equal,
        "total_pixels": count,
        "exact_equal_fraction": equal / count,
        "mean_absolute_channel_difference": total / (count * 3),
        "exact_match": equal == count,
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("reference", type=Path)
    parser.add_argument("candidate", type=Path)
    parser.add_argument("--report", type=Path, required=True)
    parser.add_argument("--require-exact", action="store_true")
    args = parser.parse_args()
    if args.report.exists():
        raise SystemExit("Report exists; preserve evidence and choose a new report filename.")
    with Image.open(args.reference) as source, Image.open(args.candidate) as actual:
        if source.size != actual.size:
            raise SystemExit(f"Size mismatch: reference {source.size}, candidate {actual.size}. No normalization performed.")
        if source.width * source.height > 16_000_000:
            raise SystemExit("Comparison exceeds the 16-megapixel input bound.")
        reference, candidate = source.convert("RGB"), actual.convert("RGB")
    report = {
        "schema_version": 1,
        "reference_sha256": fingerprint(args.reference),
        "candidate_sha256": fingerprint(args.candidate),
        "candidate": args.candidate.name,
        "dimensions": list(reference.size),
        "method": "RGB absolute difference, no alignment, scaling, exclusions or image edits",
        "limitation": "Includes scene-state differences. Equality measures pixels, not rules or interaction correctness.",
        "whole_frame": difference(reference, candidate),
    }
    if reference.size == (2000, 1135):
        regions = {
            "board": (0, 0, 1549, 1025),
            "sidebar": (1549, 0, 2000, 1135),
            "bottom": (0, 1025, 1549, 1135),
            "left_tools": (0, 0, 80, 310),
            "activity": (1549, 0, 2000, 280),
            "chat": (1549, 283, 2000, 507),
        }
        report["regions"] = {
            name: difference(reference.crop(rect), candidate.crop(rect))
            for name, rect in regions.items()
        }
    args.report.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(report["whole_frame"]))
    if args.require_exact and not report["whole_frame"]["exact_match"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
