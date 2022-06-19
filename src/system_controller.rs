use std::io::Error;
use system_shutdown::reboot;

pub struct SystemController {}

impl SystemController {
    pub fn new() -> Self { SystemController{} }

    pub fn reboot(&self) -> Result<(), Error> { reboot() }
}