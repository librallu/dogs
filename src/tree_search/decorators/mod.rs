/** helper code for the decorators */
pub mod helper;

/** g-cost dominance decorator */
pub mod gcost_dominance;

/** provides various search statistics */
pub mod stats;

/** implements limited discrepancy search based algorithms */
pub mod lds;

/** generic pruning mechanism */
pub mod pruning;

/** generic dual bound report */
pub mod bounding;

/** guide with bound mixing
estimates the average value of the bound and the guide, and sums them, taking into account
the distance from the root.
*/
pub mod guide_with_bound;