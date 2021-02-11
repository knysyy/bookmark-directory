use structopt::{clap, StructOpt};

use crate::{
    errors::CommandError,
    models::{bookmark, result::CommandResult},
    types::CliResult,
    utils::database::establish_connection,
};

#[derive(Debug, StructOpt)]
#[structopt(name = "del", about = "delete directory bookmrak")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Delete {
    #[structopt(index = 1)]
    key: String,
}

impl Delete {
    pub fn run(&self) -> CliResult {
        let conn = establish_connection()?;
        match bookmark::get_bookmark(&conn, &self.key) {
            Ok(bookmark) => {
                bookmark::delete_bookmark(&conn, &self.key)?;
                Ok(CommandResult::Deleted(bookmark.key, bookmark.path))
            },
            Err(diesel::NotFound) => Err(CommandError::KeyNotFoundError(self.key.clone())),
            Err(err) => Err(CommandError::DieselError(err)),
        }
    }
}
