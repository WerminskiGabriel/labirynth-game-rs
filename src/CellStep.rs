use crate::directions::Directions;

#[derive(Clone, Copy, Debug,PartialEq)]
pub enum CellStep{
    Unvisited,
    Direction(Directions),
    Finish,
}