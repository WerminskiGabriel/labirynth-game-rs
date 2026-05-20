use crate::directions::Directions;

#[derive(Clone, Debug,PartialEq)]
pub enum CellStep{
    Unvisited,
    Direction(Directions),
    Finish,
}