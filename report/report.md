---
title: Longest path in a DAG
subtitle: Assignment of Graph Theory and Algorithms
author: Alessandro Bregoli
bibliography: biblio.bib
header-includes:
  - \usepackage{algpseudocode, algorithm}
...


# Introduction

The goal of this report is to study the time complexity of the single source shortest/longest path
for a Directed Acyclic Graph. After a short introduction to the notation in Section \ref{notation}, 
I describe the single source longest simple path problem in Section \ref{problem} with partucluar
attention to the solution algorithm and its computational complexity. Finally, in Section
\ref{implementation}, I present my Rust implementation of the algorithm.

# Notation {#notation}


A **directed graph** $G=(V,E)$ is composed by a finite set of **nodes** $V$ and a finite set of
**edges** $E$ such that $E \subseteq [V]^2$ (@cormen). A graph is weighted if there is a function 
$w : E\rightarrow \mathbb{R}$. Given two nodes $x,y \in V$  we say that $x$ is
**adjacent** to $y$ if $\{x,y\} \in E$. This relation isn't symmetric. A **path** is a sequence of
adjacent vertices $\langle v_1,v_2,...,v_k\rangle : v_i \in V \land \{v_i, v_{i+1}\}\in E$. The
**weight** of a path is equal to the number of edges in the case of a non-weighted graph while it is
equal to the sum of the weights of the arcs in the case of a weighted graph. 
A path is **simple** if all vertices in the path are distinct. A path for a **cycle** if 
$v_0 =v_k$ and the path contains at least one edge. A Directed Acyclic Graph (**DAG**) is a
directed graph without cycles of any length. A **shortest simple path** is a simple path of minimal
weight. Conversely, a **longest simple path** is a simple path of maximum weight.

# Single source longest simple path problem {#problem}

The single source shortest path problem can be solved in polynomial time for each graph. On the
other hand, the single source longest simple path is NP-hard for a generic graph. However, if we
only want to solve the problem for directed acyclic graphs we can modify the algorithm for finding
the single source shortest path in a convenient way and identify the single source simple longest
path in linear time.

## The algorithm

In @cormen the authors say that both the single source shortest path and the single source longest
path for a directed acyclic graph are based on two components: a relax procedure and a topological
sort algorithm. The only difference between the two algorithms is the definition of the relax
function.

- **Relax procedure for the single source shortest path algorithm**. The process of relaxing an
  edge $(u,v)$ consists of testing whether we can improve the shortest path to $v$ found, so that
  far by going through $u$ and, if so, updating the shortest path.
- **Relax procedure for the single source longest simple path algorithm**. The process of relaxing
  an edge $(u,v)$ consists of testing whether we can improve the longest path to $v$ found, so that
  far by going through $u$ and, if so, updating the longest path. (Algorithm \ref{algorithm_relax})

\begin{algorithm}
  \caption{RELAX function}
  \label{algorithm_relax}
  \begin{algorithmic}[1]
    \Function{relax}{$v,w \in G.V$, $w$: edge weight}
      \If{$v$.longest\_path < $u$.longest\_path + $w$($u$,$v$)}
        \State $v$.longest\_path = $u$.longest\_path + $w$($u$,$v$)
        \State $v$.predecessor = $u$
      \EndIf
    \EndFunction
  \end{algorithmic}
\end{algorithm}


The **topological sort algorithm** is an algorithm witch find an order among the nodes such that,
  if there is and edge $(u,v)$ then $u$ appear before $v$ in the topological ordering. As shown in
    \ref{algorithm_topological_sort}) , the topological ordering is based on the DFS-visit and
    inherits its computational complexity.   


\begin{algorithm}
  \caption{TOPOLOGICAL SORT}
  \label{algorithm_topological_sort}
  \begin{algorithmic}[1]
  \Function{topological\_sort}{$G$}
    \State topological\_order = \Call{list()}{}
    \For{each $u \in G.V$}
      \State $u.color = WHITE$
      \State $u.predecessor = NIL$
    \EndFor
    \For{each $u \in G.V$}
      \If{$u.color == WHITE$}
        \State \Call{DFS-VISIT}{$G$, $u$, topological\_order}
      \EndIf
    \EndFor
    \State \Return topological\_order
  \EndFunction
  
  \item[]
  
  \Function{DFS-VISIT}{$G$, $u$, topological\_order}
    \State u.color = GRAY

    \For{each $v\in G.Adj[u]$}
      \If{$v.color == WHITE$}
        \State $v.predecessor = u$
        DFS-VISIT($G$, $v$, topological\_order)
      \EndIf
    \EndFor
    \State $u.color = BLACK$
    \State topological\_order.prepend($u$)
  \EndFunction
  \end{algorithmic}
\end{algorithm}


\begin{algorithm}
  \caption{Single source shortest path}
  \label{algorithm_shortest_path}
  \begin{algorithmic}[1]
    \Function{dag-longest-paths}{$G$, $s$}
    \State T = \Call{topological\_sort}{$G$} \label{lst:ts}
    \For{$v \in G\setminus s$} \label{lst:start_init}
      \State $v$.longest\_path = $-\infty$
    \EndFor
    \For{$v \in G$}
      \State $v$.predecessor = NILL
    \EndFor 
    \State $s$.longest\_path = 0 \label{lst:end_init}
    \For{$u \in T$} \label{lst:start_search}
      \For{$v \in G.Adj[u]$}
        \State \Call{relax}{$u$,$v$,$w$}
      \EndFor
    \EndFor \label{lst:end_search}
    
    \EndFunction
  \end{algorithmic}
\end{algorithm}

By exploiting the topological ordering it is possible to define the **single source longest path**
**algorithm** as shown in Algorithm \ref{algorithm_shortest_path}.
The rationale behind this algorithm is based on the fact that there are no cycles in a DAG.
Consequently if the dag contains a path from vertex $u$ to vertex $v$, then $u$ precedes $v$ in the
topological sort. For this reason it is sufficient to make just one pass over the vertices in the
topologically sorted order.

## Time Complexity

The goal of this subsection is to show the linear complexity of the single source longest path
algorithm. In order to achieve this goal we can split the algorithm in its main components
described in Algorithm \ref{algorithm_relax}, Algorithm \ref{algorithm_topological_sort} and 
Algorithm \ref{algorithm_shortest_path}.


Analyzing the RELAX function in Algorithm \ref{algorithm_relax} we can say that the time complexity
is: $O(1)$.^[This is true only if the access to the weight $w(u,v)$ is $O(1)$. For this reason the
selection of an appropriate data structure is crucial.]


It is well known from the literature that a DFS search has a time complexity of $O(V+E)$. Since the 
TOPOLOGICAL SORT presented in Algorithm \ref{algorithm_topological_sort}  is basically a DFS
also its time complexity is $O(V+E)$.


In order to find the time complexity of the Single source shortest path presented in Algorithm
\ref{algorithm_shortest_path}  we need to split the code as follow:

- **Line \ref{lst:ts}**: it calls the TOPOLOGICAL SORT algorithm with time complexity $O(V+E)$
- **Lines  \ref{lst:start_init} - \ref{lst:end_init}**: these lines initialize the data structures for
  each node with a time complexity of $O(V)$.
- **Lines \ref{lst:start_search} - \ref{lst:end_search}**: in this portion of code the algorithm
  pass through each node and each vertex exactly once. For this reason the time complexity is
  $O(V+E)$

Combining the complexity of each piece of code we have that the complexity of the algorithm is: 
$$O(V + E)$$

# Implementation {#implementation}

The previous section shows that the loongest single path algorithm has a complexity of $V(V+)$.
However this is true only if we use the correct data structure to represent the graph. For example
if we use an adjacency matrix both the Algorithm \ref{algorithm_topological_sort} and the nested
for of the Algorithm \ref{algorithm_shortest_path} become $O(V^2)$. For this reason I decided to
use a adjacency list implemented with a linked list.


The programming language that I decided to use is 
Rust^[\url{https://www.rust-lang.org}] mainly becouse of its high performance.
The implemented algorithm is available on 
github^[\url{https://github.com/AlessandroBregoli/rdag}] and is
capable of:

- Load the structure of a network from an edge list file where each line is structured as follow:
  Source, Destination, weight.
- Return a topological order of the nodes for the loaded network.
- Compute the single source  shortest/logest path given a network and a source node.

## Demonstration

![Directed acyclic graph\label{fig:dag}](network.pdf)

In Figure \ref{fig:dag} there is the dag that we will use in this section.
First of all we need a file containing the edge list representation of the network: Listing
\ref{el}.



```{#el caption="Edge List (net5.el)"}
                        0   1   3.0
                        0   2   2.0
                        0   3   4.0
                        0   4   10.5
                        1   3   3.5
                        2   3   0.2
                        2   4   6.0
                        3   4   1.5
```

The program *rdag* allow to load this file and find a topological order for the nodes (Listing
\ref{topological}) using the following arguments:

- *-n 5*: number of nodes in the network
- *-p net5.el*: path to the network file
- *topological*: subcommand to return a topologically ordered list of nodes

```{#topological caption="rdag - topological sort"}
$ rdag -n 5 -p net5.el topological
Output: 0 2 1 3 4
```

*rdag* is also capable of computing the single source shortest/longest path (Listing \ref{SLpath}.
The required parameters to accomplish these tasks are:

- *-n 5*: number of nodes in the network
- *-p net5.el*: path to the network file
- *SLpath*: subcommand to compute the single source shortest/longest path
- *-s 0*: source node
- *-l*: this is an optional parameter. If present the program computes the shortest path.
  Otherwise it computes the longest path

\pagebreak

~~~{#SLpath caption="rdag - shortest/longest path"}
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
~~~

# Conclusion

In this assignment I studied the time complexity of the single source shortest path algorithm, both
form a theoretical and practical point of view. In fact, while the theoretical implementation is
proved to has a time complexity fo $O(V+E)$, in practice a poor choiche of the data structure can
drammatically worsen the complexity of the algorithm.

# Bibliography



