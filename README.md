# Minisynth
This project was developed with Rust for the Hive Sound Synthesis Rush Project.

## What is it?
Our objective was to basically develop a minimal audio synthesizer tool, called `minisynth`. A Command line tool that will parse a specific music description file format, and produce synthesized audio from it. 

## How do I run it?
If you want to try out this synthesizer, once you cloned this repository, the executable should already be ready as the file `minisynth`. There are a bunch of audio description files you can choose from in `audio_samples`. Simply invoke the tool with the command:
```
./minisynth file
```

We hope you have as much fun as we did!

### The Music Description Format
The `minisynth` tool accepts the following file format (The file may be named with a `.synth` suffix, but really, any file naming works fine too):
* The file is text based
* Any empty line or line starting with a # character is ignored as comment
* The first non-comment line must be in the format `tempo <N>`, where `<N>` is an integer, representing the tempo of the piece in beats per minute
* The next non-comment line must be tracks followed by a comma separated list of instruments: `tracks <instrument>[,<instrument>,...]`. Each entry in this list represent a track, numbered from 1 to the total number of tracks. 
* All remaining non-comment lines must be in the following format: `<track>:<notes>`, where `<track>` is the track number, and `<notes>` represents notes to be added to the given track.

### The Notes Format
The `<notes>` part of each line is parsed for all substrings matching the following pattern: `<pitch>[<alteration>][<octave>][/<duration>]`.
* `<pitch>` is any lowercase letter from **a** to **g**, representing the usual pitch names in Western notation, or the letter **r** to represent a rest (silence)
* `<alteration>` is optional, and can either be **#** or **b**, indicating that the note should be sharp or flat (note that these are the hash symbol and the lowercase letter B, representable in ASCII, not the sharp and flat musical symbols available in Unicode)
* `<octave>` is an optional integer from **0** to **9** representing the octave of the note (using the standard scientific pitch notation, so that middle C is **c4**)
* `<duration>` is optional and preceded with a **/** when present: it is a decimal number, possibly fractional, representing the duration of the note in beats

More details about the notes format can be found in the attached [pdf](rushes-sound-synthesis.en.pdf) in this repository.

### The Audio synthesizer
Our `minisynth` tool generates 4 basic 

