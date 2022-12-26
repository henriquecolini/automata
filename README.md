# automata

[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)

This repository contains code that parses, emulates, does general conversions, and builds visualizations of finite automaton. Namely, it is able to parse a [Formal Language Theory](https://en.wikipedia.org/wiki/Formal_language) [Regular Expression](https://en.wikipedia.org/wiki/Regular_expression#Formal_language_theory) into machine-readable structures, convert them into [NFA-εs](https://en.wikipedia.org/wiki/Nondeterministic_finite_automaton#NFA_with_%CE%B5-moves) and [DFAs](https://en.wikipedia.org/wiki/Deterministic_finite_automaton), and finally generate [DOT language](https://graphviz.org/doc/info/lang.html) files out of those, for Graphviz visualization.

This software was created for educational purposes, and it follows closely the mathematical definitions one might see on an introductory computer theory class. It was not developed for practical use (i.e. as a general text processor) and does not follow the POSIX standard you're likely familiar with. It was built with performance in mind, but certainly not as a primary objective; there are multiple aspects that could be improved.

The project was written entirely in the Rust programming language, as its strong typing, performance and and memory safety features were deemed important. I do admit that I'm still a beginner with the language, therefore I'm sure there are issues and obvious mistakes in the code, so feel free to point them out and contribute. I will take a good look at any pull requests.
