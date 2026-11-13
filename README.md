# ai-xvt-base

An AI-ready decompile of Star Wars: X-Wing vs. TIE Fighter (1997) and
its Balance of Power expansion.

This repository is the decompile of X-Wing vs. TIE Fighter, the space
combat game LucasArts released in 1997. It is rebuilt from the 2 game
programs: `Z_XVT__.EXE` and `z_bop__.exe` (Balance of Power). AI-ready
means the code is written to be read by AI. Each function, one small
piece of the program, and each variable gets a plain name chosen by
reading what it does, so a person or an AI model working in the code can
follow it.

The end of a name will say how sure that reading is. A name with
nothing added is a confident one. A name ending in `_Likely` is
probably right. A name ending in `_Guess` is the least sure, so check
it against the code before relying on it. Variable names will carry
the same endings.

## Why another decompile?

This repo changes nothing. It has no improvements, no new features and
no bug fixes: each function does what the original does, bugs included.
That makes it a fixed starting point for other work, such as a port to
a new system, a mod, a study of how the game works, or a check on
someone else's rebuild.

It is also built for AI models to work in. The names say what each
piece does, so a model helping with one of those projects can read and
change the code without first having to work out what it means.

The MIT license suits a base like this. It lets anyone reuse the code
on one condition: keep its copyright and license notice. A companion
project, such as a port or a mod, can carry this code inside its own
work, under its own license, and take each new function as it arrives
here.

## Status

Each rebuilt function arrives here once it passes its checks against
the original program. The first 1,146 arrived on 13 November 2026: 605
from X-Wing vs. TIE Fighter and 541 from Balance of Power. Each is in
its own file under `src/`, named by its address, with a short record
under `provenance/` of which checks it passed and which tools made it.
None has a plain name yet. 5 keep a name that came with the
decompiler's output, such as `timeGetTime`, and the other 1,141 still
carry its stand-in names, `FUN_` and the address, until a later pass
names them. A list of every named function, with a link to its file,
is in `src/README.md`.

On the same day, 1,120 of these functions also arrived in Rust, under
`rust/`: 588 from X-Wing vs. TIE Fighter and 532 from Balance of
Power. An AI model converted each one from its rebuilt C with no
fixes, so the Rust keeps the original's bugs too. Each of the two
folders is a Rust library set up to build for 32-bit x86 Windows only,
the machine the original ran on. The Rust still uses the stand-in
names, which a later pass will replace.

## The legal side

Decompiling a commercial game is a legal gray area. The game itself
still belongs to its copyright holder: Totally Games made it and
LucasArts published it, and its rights now sit with Lucasfilm, part of
The Walt Disney Company. Star Wars and X-Wing are trademarks of
Lucasfilm. This project is not affiliated with or endorsed by any of
these companies.

The source code here is released under the MIT license (see `LICENSE`)
so that anyone can use it in their own work quickly. That license
covers only what is written in this repository. It grants no rights in
the game itself: its story, art, sound, music, names and trademarks.

- **You need your own copy of the game.** No game files are included:
  no art, sound, missions or programs.
- **It is provided as is,** with no warranty of any kind, as the MIT
  license says.
