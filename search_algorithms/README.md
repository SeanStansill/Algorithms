# Search Algorithms

In this small project we will look at two common search algorithms; linear search and binary search. The program will ask you to choose which method you would like to use and choose the size of list you'd like to test (n). The program will then create a list of the integers from 1 to n and use a random number generator to choose a number to search for (this kind of sorted list is well suited to binary search). It will then use the specified algorithm to search for that integer. This will be put into a loop and search for 1000 random integers. The program will return the average time and number of loop elements it took to find the element. To run the program do `cargo run`.

## Linear Search

Linear search is the brute force method. On average, this is the slowest method of searching a data structure (array, list, etc). It loops through every element in a list until it finds a match. In pseudo code this looks like

For a list $L$ of length $n$ with elements $L_0$, ..., $L_{n-1}$, and search parameter $S$

1) Let $i$ equal 0
2) if the $i$-th element of $L$ is equal to $S$, finish search successfully. Else increase $i$ by 1
4) if $i$ is less than $n$, go to step 2. Else $S$ is not in $L$.

## Binary Search

Binary search is an optimized search algorithm which can be applied to sorted data. Instead of complexity of $O(n)$, this algorithm has complexity of $O(\log(n))$. This speed-up is gained by starting each comparison in the middle of the array, this then allows half of the posibilities to be discarded at each step. Linear search only discards a single element at each step. For the same definitions as above, the pseudocode procedure is

1) Let $i$ equal 0 and $j$ equal $n-1$
2) If i > j the search is unsuccessful
3) Set $k$ equal to the floor of $\frac{i+j}{2}$
4) If $L_k < S$ set $i$ equal to $k+1$ and go to step 2
5) If $L_k > S$ set $i$ equal to $k-1$ and go to step 2
6) If $L_k = S$, the search is complete
7) If $L_k \neq S$, the search fails and $S$ is not in $L$

Step 2 is error handling. It will fail for $n<0$. In step 3, the operator `floor` rounds down the division of an odd number (for example `floor(5/2)` returns 2), we could have equally chosen the ceiling.

This algorithm can be adapted to non-integer numbers, strings sorted alphabetically, and other ordered data types.
