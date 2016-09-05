//! The `conductor repo` subcommand.

use command_runner::CommandRunner;
use project::Project;
use util::Error;

/// We implement `conductor repo` with a trait so we put it in its own module.
pub trait CommandRepo {
    /// List all the repositories associated with a project.
    fn repo_list<CR>(&self, runner: &CR) -> Result<(), Error>
        where CR: CommandRunner;

    /// Clone the specified repository.
    fn repo_clone<CR>(&self, runner: &CR, alias: &str) -> Result<(), Error>
        where CR: CommandRunner;
}


impl CommandRepo for Project {
    fn repo_list<CR>(&self, _runner: &CR) -> Result<(), Error>
        where CR: CommandRunner
    {
        for repo in self.repos().iter() {
            println!("{:15} {}", repo.alias(), repo.git_url());
            if repo.is_cloned(self) {
                let path = try!(self.src_dir().join(repo.rel_path())
                    .strip_prefix(self.root_dir())).to_owned();
                println!("  Cloned at {}", path.display());
            }
        }
        Ok(())
    }

    fn repo_clone<CR>(&self, runner: &CR, alias: &str) -> Result<(), Error>
        where CR: CommandRunner
    {
        let repo = try!(self.repos().find_by_alias(alias).ok_or_else(|| {
            err!("Could not find a repo with short alias \"{}\"", alias)
        }));
        if !repo.is_cloned(self) {
            try!(repo.clone_source(runner, self));
        }
        Ok(())
    }
}

// No tests because this is a very thin wrapper over `Repos` and `Repo`.