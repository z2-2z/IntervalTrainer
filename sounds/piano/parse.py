#!/usr/bin/env python3

import json
import base64

def main():
    with open("acoustic_grand_piano-mp3.json") as f:
        data = json.load(f)
        
    for key, text in data.items():
        encoding, enc = text.split(",")
        assert encoding == "data:audio/mp3;base64"
        audio = base64.b64decode(enc.encode("ascii"))
        
        with open(f"{key}.mp3", "wb") as f:
            f.write(audio)

if __name__ == "__main__":
    main()
