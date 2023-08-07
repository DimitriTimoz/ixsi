use crate::prelude::*;

pub struct Db<T> {
    config: IxsiConfig,
    /// The database containing all the tastes of the peers and interests points of the user.
    my_tastes: T,
    /// Filters received from seeders to find common points of interest.
    seeder_filters: RwLock<BTreeMap<PeerId, ()>>,
    /// Peers we send filters to find common points of interest.
    leechers: RwLock<BTreeSet<PeerId>>,
}

impl<T> Db<T> {
    pub fn new(config: IxsiConfig, my_tastes: T) -> Self {
        Self {
            config,
            my_tastes,
            leechers: RwLock::new(BTreeSet::new()),
            seeder_filters: RwLock::new(BTreeMap::new()),
        }
    }
}

/*
    
*/