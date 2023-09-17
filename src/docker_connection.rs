extern crate rs_docker;
extern crate lazy_static;

use std::sync::Mutex;
use lazy_static::lazy_static;
use rs_docker::{Docker};

lazy_static! {
    pub static ref DOCKER_CONNECTION: Mutex<Docker> = {
        let mut connection = Docker
            ::connect("unix:///var/run/docker.sock")
            .expect("test");
        Mutex::new(connection)
    };
}

