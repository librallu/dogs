/**
defines a decision tree structure. It allows to reduce the memory consumption and time to clone
information required to retrieve the solution.
*/
pub mod decision_tree;

/**
implements a lazy_clonable structure. While the object is not explicitely used, the cloned version
keeps a reference towards its "parent".
*/
pub mod lazy_clonable;

/**
Implmements a sparse set. Allows fast operations but consumes more memory than a standard bit-set.
*/
pub mod sparse_set;