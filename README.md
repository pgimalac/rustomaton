# A Rust automaton manipulation library project.

## Fonctionnalities
The library allows to build automatons (DFA and NFA) and regexes.

It also provides lots of classic algorithms over theses structures and allows to convert from one to another.

## Algorithms implemented
- union of two automatons
- intersection of two automatons
- equality of two automatons
- concatenation of two automatons
- complementary of an automaton
- minimisation of an automaton
- Kleene closure of an automaton
- determinization of an automaton
- completed automaton
- accessible automaton
- co-accessible automaton
- trimmed automaton
- reversed automaton

## Displayal
Regexes can be displayed as Strings but the "simplify" function is not incredible so it generates stupidly long regexes.

Automatons can be exported to [.dot files](https://en.wikipedia.org/wiki/DOT_(graph_description_language)).

## Bugs
This library hasn't been tested intensively so I wouldn't recommand using it for something too serious.

If you notice a bug or anything weird don't hesitate to open an issue or a pull request on [the github page](https://github.com/pgimalac/rustomaton).

Lots of links in the doc are broken as I didn't spent much time debugging it.
