# graph_match

[![Build Status](https://travis-ci.org/charlieegan3/graph_match.svg?branch=master)](https://travis-ci.org/charlieegan3/graph_match) [Cargo Crate](https://crates.io/crates/graph_match)

A library for matching patterns in directed graphs. The library implements 
'queries' for graphs as graphs that represent in part the nodes in the graph 
to be matched.

The API for this functionality is still changing and the project is in early
development. Integration tests can be found 
[here](https://github.com/charlieegan3/graph-match/blob/master/tests/lib.rs) - 
these give a reasonable overview of the functionality.

## Features
There are two broad classes of query implemented, these can be described
loosely as:
* _Given a graph, does this subgraph exist within it?_
* _Given a node in a graph, what is the subgraph that can be reached by 
following it's directed edges?_

## Motivations
As part of my honors project I needed to answer the above questions for a graph
data structure. In my case, the graph was a 
[dependency graph](https://en.wikipedia.org/wiki/Dependency_grammar). I opted 
to use Neo4j for the purpose, the Cypher query language is really nice to 
write queries in. This introduces temporary persistence for graphs during
analysis, and that slows things down quite a bit. Running a Neo4j container
also uses more RAM than I'd like. So, in my efforts to get 
[standpoint](https://github.com/charlieegan3/standpoint) up and running on
a cheap cloud instance, this project, that implements the features of Cypher
that I use, was created.


## Graph Assumptions
This project has been written for a tool that I'm working on where it's 
possible to make a number of assumptions on the type of graphs used. I'm 
working with dependency parse graphs and have made the following assumptions:

* Graphs are directed
* Graphs are acyclic
