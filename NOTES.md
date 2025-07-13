https://github.com/BartMassey/ws5

## The Genesis

* Next-door neighbor writes some Python. Estimates
  a year of runtime.
  
* I estimate I can do the same thing in one minute.

* A week or more of work I don't have time for ensues.

## 5×5 Word Squares

* Magic

      aback
      belle
      alloy
      close
      keyed

* "Double"

      aback
      radon
      erode
      agree
      sends

* Transpositions

      areas
      barge
      adorn
      coded
      knees

## Searching For All Solutions

* Finding all of these is "complete state-space search". In
  practice, depth-first search from partial to complete
  solutions.
  
* Dumb: start with a word

      aback
      .....
      .....
      .....
      .....

  add another word

      aback
      abaft
      .....
      .....
      .....

  when full

      aback
      abaft
      abase
      abash
      abate

  note failure and try fixing the last line

      aback
      abaft
      abase
      abash
      .....

      aback
      abaft
      abase
      abash
      abbey

* Naïve: Notice when stuck and retry

      aback
      abaft
      .....
      .....
      .....

  Nope. Look for the first word that could work.

      aback
      baron
      .....
      .....
      .....

  Backtrack when stuck. Can work, but maybe a day.

* Basic: Try alternating vertical and horizontal words.
  
* Standard: Try the hardest-to-fill position first.

      aback
      ...on
      ...de
      ...ee
      ...ds

* The rest is implementation

## Implementation

* Recursive depth-first search is fairly standard.

* Really hard to debug this kind of code. Use lots of unit
  tests.

* Write auxiliary tools to help. Maybe even in Python!

* Runtimes are ridiculous. Use a small example to debug and
  measure performance. I didn't this time. I'm dumb.

* Profiling can help speed things up. Use a profiler.

* Early pruning is essential.

* Want to keep memory use small because caches and
  allocator.  Use do-undo. Use iterators.

* Code should pay for itself. No big complexity for tiny
  (less than 2-3×) gains. Problem is "exponential", so
  claiming one-time linear speedups is boring unless
  really large.

* Rust "functional programming" is a thing. Iterators
  are key to performance.

## Epilogue

I got my code to run in one minute on my laptop. I was
pretty proud of it, and gave a talk to the PDX Rust Meetup a
few hours later.

During the talk Jim Blandy thought up a better algorithm,
and quickly constructed a solver
the night after I gave a talk about this. It is a clever
approach using prefix tries rather than state-space
search. (He is clearly smarter than I am.)

Jim's implementation runs in 4 seconds on my
desktop. <https://github.com/jimblandy/jimb-ws5>
