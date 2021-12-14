pub mod tvix {
    pub mod evaluator {
        include!(concat!(env!("OUT_DIR"), "/tvix.proto.evaluator.v1.rs"));
    }
}

pub mod yzix {
    pub mod runwork {
        include!(concat!(env!("OUT_DIR"), "/yzix.proto.runwork.v1.rs"));
    }
    pub mod store {
        include!(concat!(env!("OUT_DIR"), "/yzix.proto.store.v1.rs"));
    }
}
