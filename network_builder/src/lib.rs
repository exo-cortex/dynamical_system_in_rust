pub mod edge;
pub mod network;
pub mod network_builder;

pub enum SelectGroup {
    AllGroups,
    SingleGroup(usize),
    NotGroup(usize),
}

fn on_ring(a: isize, b: usize) -> usize {
    (((a % b as isize) + b as isize) % b as isize) as usize
}
