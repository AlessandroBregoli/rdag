---
title: Longest path in a DAG
subtitle: Assignment of Graph Theory and Algorithms
author: Alessandro Bregoli
...



# Notation

A **directed graph** $G=(V,E)$ is composed by a finite set of **nodes** $V$ and a finite set of
**edges** $E$ such that $E \subseteq [V]^2$. A graph is weighted if there is a function 
$w : E\rightarrow \mathbb{R}$. Given two nodes $x,y \in V$  we say that $x$ is
**adjacent** to $y$ if $\{x,y\} \in E$. This relation isn't symmetric. A **path** is a sequence of
adjacent vertices $\langle v_1,v_2,...,v_k\rangle : v_i \in V \land \{v_i, v_{i+1}\}\in E$. The
**weight** of a path is equal to the number of edges in the case of a non-weighted graph while it is
equal to the sum of the weights of the arcs in the case of a weighted graph. 
A path is **simple** if all vertices in the path are distinct. A path for a **cycle** if 
$v_0 =v_k$ and the path contains at least one edge. A Directed Acyclic Graph (**DAG**) is a
directed graph without cycles of any length. A **shortest simple path** is a simple path of minimal
weight. Conversely, a **longest simple path** is a simple path of maximum weight.

# Single-source longest simple path problem





