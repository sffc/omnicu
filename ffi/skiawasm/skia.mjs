// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import { ICU4XDataProvider } from "./lib/ICU4XDataProvider.js";
import { ICU4XBidi } from "./lib/ICU4XBidi.js";
import { readFileSync } from "fs";

const DATA = readFileSync("icu4x_data_skiawasm.postcard");

const SAMPLE_TEXT = [
    "א",
    "ב",
    "ג",
    "a",
    "b",
    "c",
].join();

async function main() {
    const provider = ICU4XDataProvider.create_from_byte_slice(DATA);
    const bidi = ICU4XBidi.create(provider);
    const bidiInfo = bidi.for_text(SAMPLE_TEXT);

    console.log("Number of paragraphs:", bidiInfo.paragraph_count());

    const paragraph = bidiInfo.paragraph_at(0);
    const reordered = paragraph.reorder_line(paragraph.range_start(), paragraph.range_end());

    console.log("Input:", SAMPLE_TEXT);
    console.log("Reordered:", reordered);
}

main();
