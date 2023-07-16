use std::collections::HashSet;
use std::fmt::{Display, Error, Formatter};
use petgraph::{Direction as GraphDirection};
use petgraph::graph::NodeIndex;
use crate::path::{Direction, NodeType, Path};

const STARTING_ROOM: (isize, isize) = (0, 0);

const WALL: &str = "⬜️";
const DOOR: &str = "🚪";
const ROOM: &str = "◾️";
const USER: &str = "🧍";

#[derive(Debug)]
pub struct Map {
    rooms: HashSet<(isize, isize)>,
    doors: HashSet<((isize, isize), (isize, isize))>,
}

impl Map {
    pub fn new(path: &Path) -> Self {
        let mut map = Map {
            rooms: HashSet::from([STARTING_ROOM]),
            doors: HashSet::new(),
        };
        map.trace_path(path, NodeIndex::new(0), STARTING_ROOM);
        map
    }

    fn trace_path(&mut self, path: &Path, path_index: NodeIndex, position: (isize, isize)) {
        print!("Trace index: {:5}\r", path_index.index());

        let node = &path[path_index];
        let outputs = path.neighbors_directed(path_index, GraphDirection::Outgoing);

        if let NodeType::Direction(direction) = node {
            let to = match direction {
                Direction::North => (position.0, position.1 - 1),
                Direction::South => (position.0, position.1 + 1),
                Direction::East => (position.0 + 1, position.1),
                Direction::West => (position.0 - 1, position.1),
            };
            self.add_room(position, to);

            for output in outputs {
                self.trace_path(path, output, to);
            }
        } else {
            for output in outputs {
                self.trace_path(path, output, position);
            }
        }
    }

    fn add_room(&mut self, from: (isize, isize), to: (isize, isize)) {
        self.rooms.insert(to);
        self.doors.insert((from, to));
        self.doors.insert((to, from));
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let min_x = self.rooms.iter().map(|room| room.0).min().unwrap();
        let max_x = self.rooms.iter().map(|room| room.0).max().unwrap();
        let min_y = self.rooms.iter().map(|room| room.1).min().unwrap();
        let max_y = self.rooms.iter().map(|room| room.1).max().unwrap();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let room = (x, y);

                let is_room = self.rooms.contains(&room);
                let has_door_north = self.doors.contains(&(room, (x, y - 1)));
                if is_room && has_door_north {
                    write!(f, "{}{}", WALL, DOOR)?;
                } else {
                    write!(f, "{}{}", WALL, WALL)?;
                }
            }
            write!(f, "{}", WALL)?;
            writeln!(f)?;
            for x in min_x..=max_x {
                let room = (x, y);
                let has_door_west = self.doors.contains(&(room, (x - 1, y)));
                if room == STARTING_ROOM {
                    if has_door_west {
                        write!(f, "{}{}", DOOR, USER)?;
                    } else {
                        write!(f, "{}{}", WALL, USER)?;
                    }
                } else if self.rooms.contains(&room) {
                    if has_door_west {
                        write!(f, "{}{}", DOOR, ROOM)?;
                    } else {
                        write!(f, "{}{}", WALL, ROOM)?;
                    }
                } else {
                    write!(f, "{}{}", WALL, WALL)?;
                }
            }
            write!(f, "{}", WALL)?;
            writeln!(f)?;
        }
        for _ in min_x..=max_x {
            write!(f, "{}{}", WALL, WALL)?;
        }
        writeln!(f, "{}", WALL)?;

        Ok(())
    }
}