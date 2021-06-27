/**
defines a search space decorator trait.
A search space decorator allows 
*/
pub trait SearchSpaceDecorator<S> {
    /** gets the underlying search space */
    fn unwrap(&self) -> &S;
}