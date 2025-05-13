#!/usr/bin/env python3

import os
import sys
import re

def convert(note, octave):
    notes = ["c", "db", "d", "eb", "e", "f", "gb", "g", "ab", "a", "bb", "b"]
    return (int(octave), notes.index(note.lower()))

def main():
    dir = sys.argv[1]
    instrument = dir.split("/")[-1]
    pattern = re.compile("^([A-Za-z]+)([0-9])$")
    result = {}
    
    for entry in os.listdir(dir):
        name, ext = entry.split(".")
        match = pattern.fullmatch(name)
        assert match is not None
        note = match.group(1)
        octave = match.group(2)
        idx = convert(note, octave)
        result[idx] = entry
    
    for key in sorted(result):
        print(f"files.push(asset!(\"/assets/instruments/{instrument}/{result[key]}\"));")

if __name__ == "__main__":
    main()
