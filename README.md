# Minisynth
```
         ___                                                         |
____|\_____/_____________________|_______________________|___________|___|___..
____|/___ _)__________@______|___|________|______________|__|___@____|___|___||
___/|____====____|___|_______|___|____@___|___@__@__@____|__|__|____@____|___||
__|_/_\___/|_____|___|___@___|__O____|____|__|__|__|____@___|__|________@____||
___\|/___'-|_____|___|__|____|_______|____|__|__|__|________|__|_____________||
    /      |    O     \ |            |       |  |  |             
                       \|                    `--3--'
```

This project was developed with Rust for the Hive Sound Synthesis Rush Project.

## What is it?
Our objective was to basically develop a minimal audio synthesizer tool, called `minisynth`. A Command line tool that will parse a specific music description file format, and produce synthesized audio from it. 

## How do I run it?
If you want to try out this synthesizer, once you cloned this repository, the executable should already be ready as the file `minisynth`. There are a bunch of audio description files you can choose from in `audio_samples`. Simply invoke the tool with the command:
```
./minisynth file
```

We hope you have as much fun as we did!

## The Music Description Format
The `minisynth` tool accepts the following file format (The file may be named with a `.synth` suffix, but really, any file naming works fine too):
* The file is text based
* Any empty line or line starting with a # character is ignored as comment
* The first non-comment line must be in the format `tempo <N>`, where `<N>` is an integer, representing the tempo of the piece in beats per minute
* The next non-comment line must be tracks followed by a comma separated list of instruments: `tracks <instrument>[,<instrument>,...]`. Each entry in this list represent a track, numbered from 1 to the total number of tracks. 
* All remaining non-comment lines must be in the following format: `<track>:<notes>`, where `<track>` is the track number, and `<notes>` represents notes to be added to the given track.

## The Notes Format
The `<notes>` part of each line is parsed for all substrings matching the following pattern: `<pitch>[<alteration>][<octave>][/<duration>]`.
* `<pitch>` is any lowercase letter from **a** to **g**, representing the usual pitch names in Western notation, or the letter **r** to represent a rest (silence)
* `<alteration>` is optional, and can either be **#** or **b**, indicating that the note should be sharp or flat (note that these are the hash symbol and the lowercase letter B, representable in ASCII, not the sharp and flat musical symbols available in Unicode)
* `<octave>` is an optional integer from **0** to **9** representing the octave of the note (using the standard scientific pitch notation, so that middle C is **c4**)
* `<duration>` is optional and preceded with a **/** when present: it is a decimal number, possibly fractional, representing the duration of the note in beats

More details about the notes format can be found in the attached [pdf](rushes-sound-synthesis.en.pdf) in this repository.

## The Audio synthesizer
Each **track** generates notes according to the instrument assigned, as indicated by the **tracks** line in the document. Our tool supports the following wave form generation:
• sine waves
• saw waves
• square waves
• triangle waves
The possible values for `<instrument>` are therefore **sine, saw, square and triangle**. We use the standard modern tuning of A4 playing at 440 Hz.

## The Tech
We used Rust for this project, a fascinatingly cool language that's been getting a lot of attention lately. This was my first real foray into programming with Rust, motivated by my curiousity about this language. It's fast and safe, two words not often put together for a single language.

It deals with a lot of the memory safety problems that you have in `C`, which leaves all the memory management up to the coder. Yet it's faster than languages that are memory safe like `python` or `ruby` (those are still great languages. This is because it doesn't use garbage collectors, which are slow run time programs that manage memory for you. Instead, Rust introduces the concept of `ownership`, which is a set of rules that the compile checks. If any rules are violated, the program won't compile. This means memory management happens at compile time. So with the expense of more expensive compile time, the language is fast and safe at run time.

Rust also gives a wonderful developer experience. Since the rules are so complex and elaborate, a lot of effort has been put into making VERY helpful compiler errors. It's almost like having a pair programmer, showing you where your mistakes are. I haven't worked with Rust much, but I'm already beginning to love it.

Finally, for the audio synthesis, we use the `rodio` library, which is a basic audio playback library that allows use to create the necessary audio samples, and access the output device. We also got a lot of help from this [Article](https://thewolfsound.com/sound-synthesis/wavetable-synth-in-rust/) by Jan Wilczek, that helped us create a basic Wave table (I would have preferred to refer to them as arrays, but table is fine too). This formed the basic building block for our synthesizer tool.

