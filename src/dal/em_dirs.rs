
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use directories::ProjectDirs;

pub struct EmDirs {

	pub proj_dirs: ProjectDirs,
	pub working_directory: PathBuf

}

impl EmDirs {

	pub fn new(app_name: &str) -> EmDirs {

		if let Some(proj_dirs) = ProjectDirs::from("com", "EmmaLabs", app_name) {
			let companion_dirs = EmDirs {

				proj_dirs: proj_dirs,
				working_directory: env::current_dir().unwrap()

			};
			companion_dirs.init_dirs();
			return companion_dirs;
		}
		panic!("{:?}", "Unable to locate local directories");

	}

	pub fn default() -> EmDirs {

		return Self::new("App");

	}

	pub fn get_data_dir_path(&self, relative_path: &str) -> String {

		format!("{}/{}", self.proj_dirs.data_dir().to_str().unwrap(), relative_path)

	}

	pub fn get_cache_dir_path(&self, relative_path: &str) -> String {

		format!("{}/{}", self.proj_dirs.cache_dir().to_str().unwrap(), relative_path)

	}

	fn init_dirs(&self) {

		Self::init_dir(&self.proj_dirs.cache_dir());
		Self::init_dir(&self.proj_dirs.config_dir());
		Self::init_dir(&self.proj_dirs.data_dir());

		Self::init_dir(&Path::new(&self.get_cache_dir_path("/packages")));

	}

	fn init_dir(path: &Path) {

		if path.exists() {
			return;
		}
		fs::create_dir_all(path).unwrap();

	}

}