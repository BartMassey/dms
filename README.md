# ws5: 5×5 word squares
Bart Massey 2025

Construct all five-letter-by-five-letter word squares from a
given dictionary under various constraints.

## Fancy Word Squares

Here is an example of a 5×5 canonical double [word
square](https://en.wikipedia.org/wiki/Word_square):

    mores
    uvula
    siren
    enact
    delta

This is a *word square*, since every row and column is a word
from the dictionary used in construction.

This is a *double* word square, since no word is used
more than once.

This is a *canonical* word square, since row one is a
lexically-earlier word than column one. (The *transpose* of
any word square — flipping it across the diagonal — is also
a word square. We count these as uninterestingly different
and take the canonical one as the representative.)

## Usage

Build and run this Rust program with `cargo run --release`.

Use `cargo run --release -- --help` to get help with
possible arguments.

By default, the program uses `usa_5.txt` as the dictionary,
runs silently, and saves results into `squares.json`. These
things can be changed with program arguments.

`analyze.py` checks and cleans up a `squares.json` file.

## Time and Resources

As of this writing, on my home box, this program completes
in about 1.5 minutes.

Times on my home box (AMD Ryzen 9 3900X):

* Naïve algorithm: did not finish.
* First most-constrained search: 60m35s.
* Added hit cache for cross-fit and count cache for
  best-pos: 15m00s.
* Added de-doubling (doubly checks): 15m00s.
* Added de-transposition (canonicity checks): 14m08s.
* Added split dictionary for checks: 4m52s.
* Added split dictionary for search
  * Full: 1m56s.
  * Double canonical: 1m25s.

On my laptop (Intel Lunar Lake Core Ultra 7 258V) the
runtime of the current version is 56s.

Generating full squares is about 80M nodes. Double-canonical
is about 60M.

## Profiling

This crate is set up to profile with `cargo flamegraph
--release`. You'll want to run `systctl.sh` on Linux to get
access to programming features.

There are a couple of flame graphs in the `artifacts/`
folder.

## Acknowledgments

Thanks to Gene Welborn for introducing the problem and
providing a reference dictionary.

Thanks to the author of the `caches` lib crate for nice
caches.

## License

This work is made available under the "MIT License". See the
file `LICENSE.txt` in this distribution for license terms.
