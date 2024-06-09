#!/usr/bin/env python3
# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import sys
import subprocess

SYMBOLS = [
    "ICU4XDataProvider_create_compiled",
    "ICU4XDataProvider_destroy",
    "ICU4XGraphemeClusterSegmenter_create",
    "ICU4XGraphemeClusterSegmenter_destroy",
    # "ICU4XGraphemeClusterSegmenter_segment_utf8",
    # "ICU4XGraphemeClusterBreakIteratorUtf8_next",
    # "ICU4XGraphemeClusterBreakIteratorUtf8_destroy",
    "ICU4XGraphemeClusterSegmenter_segment_utf16",
    "ICU4XGraphemeClusterBreakIteratorUtf16_next",
    "ICU4XGraphemeClusterBreakIteratorUtf16_destroy",
]

def main():
    new_argv = []
    is_export = False
    for arg in sys.argv[1:]:
        if is_export:
            if not arg.startswith("ICU4X") or arg in SYMBOLS:
                new_argv += ["--export", arg]
            is_export = False
        elif arg == "--export":
            is_export = True
        else:
            new_argv += [arg]
            is_export = False
    result = subprocess.run(["lld-16"] + new_argv, stdout=sys.stdout, stderr=sys.stderr)
    return result.returncode

if __name__ == "__main__":
    sys.exit(main())
