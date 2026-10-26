# ai-xvt-base

An AI-ready decompile of Star Wars: X-Wing vs. TIE Fighter (1997) and
its Balance of Power expansion.

A decompile is source code rebuilt from a program that was only ever
shipped compiled, the form a computer runs and a person cannot read.
This repository holds one for X-Wing vs. TIE Fighter, the space combat
game LucasArts released in 1997. It is rebuilt from the 2 game programs
in the GOG release: `Z_XVT__.EXE`, the game, and `z_bop__.exe`, Balance
of Power. AI-ready means the code is written to be read. Each function,
one small piece of the program, and each variable gets a plain name
chosen by reading what it does, so a person or an AI model working in
the code can follow it.

## Why another decompile?

This one changes nothing. It has no improvements, no new features and
no bug fixes: each function does what the original does, bugs included.
That makes it a fixed starting point for other work, such as a port to
a new system, a mod, a study of how the game works, or a check on
someone else's rebuild.

## Status

Each rebuilt function arrives here once it passes its checks against
the original program. Until the first one does, this repository holds
only this page and its license.

## The legal side

Decompiling a commercial game is a legal gray area. The game itself
still belongs to its copyright holder: LucasArts published it, and its
rights now sit with Lucasfilm, part of The Walt Disney Company. Star
Wars and X-Wing are trademarks of Lucasfilm. This project is not
affiliated with or endorsed by either company.

The source code here is released under the MIT license (see `LICENSE`)
so that anyone can use it in their own work quickly. That license
covers only what is written in this repository. It grants no rights in
the game itself: its story, art, sound, music, names and trademarks.

- **You need your own copy of the game.** No game files are included:
  no art, sound, missions or programs. The rebuild is checked against
  the GOG release.
- **It is provided as is,** with no warranty of any kind, as the MIT
  license says.
