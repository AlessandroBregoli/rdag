# rdag

*rdag* is a didactic implementation of the shortest/longest path algorithm for directed acyclic
graphs. 

## Quick Start

If you want to use *rdag* from windows or from linux the easiest way is to download the compiled
version. 

### File format

*rdag* is only capable of load a network in edge list format:

**net5.el**
```
0   1   3.0
0   2   2.0
0   3   4.0
0   4   10.5
1   3   3.5
2   3   0.2
2   4   6.0
3   4   1.5
```

The node labels (first two columns) must be a contiguous set of integers starting from 0. The edge
weight (third column) is represented by a float number.

### Topological Sort


The program *rdag* allows to load this file and find a topological order for the nodes
using the following arguments:

- *-n 5*: number of nodes in the network
- *-p net5.el*: path to the network file
- *topological*: subcommand to return a topologically ordered list of nodes

```
$ rdag -n 5 -p net5.el topological
Output: 0 2 1 3 4
```

### Shortest/Longest path


*rdag* is also capable of computing the single source shortest/longest path.
The required parameters to accomplish these tasks are:

- *-n 5*: number of nodes in the network
- *-p net5.el*: path to the network file
- *SLpath*: subcommand to compute the single source shortest/longest path
- *-s 0*: source node
- *-l*: this is an optional parameter. If present, the program computes the shortest path.
  Otherwise, it computes the longest path


```
$ rdag -n 5 -p net5.el SLpath -s 0
Shortest path from 0:
Node: 0 	-	 Predecessor: None 	-	 Distance from source: 0
Node: 1 	-	 Predecessor: 0 	-	 Distance from source: 3
Node: 2 	-	 Predecessor: 0 	-	 Distance from source: 2
Node: 3 	-	 Predecessor: 2 	-	 Distance from source: 2.2
Node: 4 	-	 Predecessor: 3 	-	 Distance from source: 3.7

$ rdag -n 5 -p net5.el SLpath -l -s 0
Longest path from 0:
Node: 0 	-	 Predecessor: None 	-	 Distance from source: 0
Node: 1 	-	 Predecessor: 0 	-	 Distance from source: 3
Node: 2 	-	 Predecessor: 0 	-	 Distance from source: 2
Node: 3 	-	 Predecessor: 1 	-	 Distance from source: 6.5
Node: 4 	-	 Predecessor: 0 	-	 Distance from source: 10.5
```

## Build from source

To build *rdag* from source [cargo](https://www.rust-lang.org/tools/install) is
required. 

Download or clone the current repository. Then execute the following command:

```
cargo build --release
```
The compiled program will be available at: *target/release/rdag*.


