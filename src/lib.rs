pub mod tvix {
    pub mod proto {
        pub mod evaluator {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/tvix.proto.evaluator.v1.rs"));
            }
        }
    }
}

pub mod yzix {
    pub mod proto {
        pub mod done {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/yzix.proto.done.v1.rs"));
            }
        }
        pub mod hashty {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/yzix.proto.hashty.v1.rs"));
            }
        }
        pub mod tasks {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/yzix.proto.tasks.v1.rs"));
            }
        }
    }
}
