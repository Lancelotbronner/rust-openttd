use std::path::{Path, PathBuf};

pub struct Paths {
	///< Base directory for all subdirectories.
	pub base: PathBuf,
	///< Base directory for all savegames.
	pub save: PathBuf,
	///< Subdirectory of save for autosaves.
	pub autosaves: PathBuf,
	///< Base directory for all scenarios.
	pub scenario: PathBuf,
	///< Subdirectory of scenario for heightmaps.
	pub heightmap: PathBuf,
	///< Subdirectory for all base data (base sets, intro game).
	pub baseset: PathBuf,
	///< Subdirectory for all NewGRFs.
	pub newgrf: PathBuf,
	///< Subdirectory for all translation files.
	pub lang: PathBuf,
	///< Subdirectory for all %AI files.
	pub ai: PathBuf,
	///< Subdirectory for all %AI libraries.
	pub ai_lib: PathBuf,
	///< Subdirectory for all game scripts.
	pub gs: PathBuf,
	///< Subdirectory for all GS libraries.
	pub gs_lib: PathBuf,
	///< Subdirectory for all screenshots.
	pub screenshot: PathBuf,
	///< Subdirectory for all social integration plugins.
	pub social_integration: PathBuf,
	///< Subdirectory for documentation.
	pub docs: PathBuf,
}

impl Paths {
	pub fn new(base: PathBuf) -> Self {
		let save = base.join("save");
		let autosaves = save.join("autosave");
		let scenario = base.join("scenario");
		let heightmap = scenario.join("heightmap");
		let baseset = base.join("baseset");
		let newgrf = base.join("newgrf");
		let lang = base.join("lang");
		let ai = base.join("ai");
		let ai_lib = ai.join("library");
		let gs = base.join("game");
		let gs_lib = gs.join("library");
		let screenshot = base.join("screenshot");
		let social_integration = base.join("social_integration");
		let docs = base.join("docs");
		Self {
			base,
			save,
			autosaves,
			scenario,
			heightmap,
			baseset,
			newgrf,
			lang,
			ai,
			ai_lib,
			gs,
			gs_lib,
			screenshot,
			social_integration,
			docs
		}
	}
}

pub fn determine_base_paths(exe: &Path) {

}
/*
/**
 * Determine the base (personal dir and game data dir) paths
 * @param exe the path to the executable
 */
void DetermineBasePaths(std::string_view exe)
{
	std::string tmp;
	const std::string homedir = GetHomeDir();
#ifdef USE_XDG
	if (auto xdg_data_home = GetEnv("XDG_DATA_HOME"); xdg_data_home.has_value()) {
		tmp = *xdg_data_home;
		tmp += PATHSEP;
		tmp += PERSONAL_DIR[0] == '.' ? &PERSONAL_DIR[1] : PERSONAL_DIR;
		AppendPathSeparator(tmp);
		_searchpaths[Searchpath::PersonalDirXdg] = tmp;

		tmp += "content_download";
		AppendPathSeparator(tmp);
		_searchpaths[Searchpath::AutodownloadPersonalDirXdg] = tmp;
	} else if (!homedir.empty()) {
		tmp = homedir;
		tmp += PATHSEP ".local" PATHSEP "share" PATHSEP;
		tmp += PERSONAL_DIR[0] == '.' ? &PERSONAL_DIR[1] : PERSONAL_DIR;
		AppendPathSeparator(tmp);
		_searchpaths[Searchpath::PersonalDirXdg] = tmp;

		tmp += "content_download";
		AppendPathSeparator(tmp);
		_searchpaths[Searchpath::AutodownloadPersonalDirXdg] = tmp;
	} else {
		_searchpaths[Searchpath::PersonalDirXdg].clear();
		_searchpaths[Searchpath::AutodownloadPersonalDirXdg].clear();
	}
#endif

#if !defined(WITH_PERSONAL_DIR)
	_searchpaths[Searchpath::PersonalDir].clear();
#else
	if (!homedir.empty()) {
		tmp = std::move(homedir);
		tmp += PATHSEP;
		tmp += PERSONAL_DIR;
		AppendPathSeparator(tmp);
		_searchpaths[Searchpath::PersonalDir] = tmp;

		tmp += "content_download";
		AppendPathSeparator(tmp);
		_searchpaths[Searchpath::AutodownloadPersonalDir] = tmp;
	} else {
		_searchpaths[Searchpath::PersonalDir].clear();
		_searchpaths[Searchpath::AutodownloadPersonalDir].clear();
	}
#endif

#if defined(WITH_SHARED_DIR)
	tmp = SHARED_DIR;
	AppendPathSeparator(tmp);
	_searchpaths[Searchpath::SharedDir] = tmp;
#else
	_searchpaths[Searchpath::SharedDir].clear();
#endif

	char cwd[MAX_PATH];
	if (getcwd(cwd, MAX_PATH) == nullptr) *cwd = '\0';

	if (_config_file.empty()) {
		/* Get the path to working directory of OpenTTD. */
		tmp = cwd;
		AppendPathSeparator(tmp);
		_searchpaths[Searchpath::WorkingDir] = tmp;

		_do_scan_working_directory = DoScanWorkingDirectory();
	} else {
		/* Use the folder of the config file as working directory. */
		size_t end = _config_file.find_last_of(PATHSEPCHAR);
		if (end == std::string::npos) {
			/* _config_file is not in a folder, so use current directory. */
			tmp = cwd;
		} else {
			tmp = FS2OTTD(std::filesystem::weakly_canonical(std::filesystem::path(OTTD2FS(_config_file))).parent_path().native());
		}
		AppendPathSeparator(tmp);
		_searchpaths[Searchpath::WorkingDir] = tmp;
	}

	/* Change the working directory to that one of the executable */
	if (ChangeWorkingDirectoryToExecutable(exe)) {
		char buf[MAX_PATH];
		if (getcwd(buf, lengthof(buf)) == nullptr) {
			tmp.clear();
		} else {
			tmp = buf;
		}
		AppendPathSeparator(tmp);
		_searchpaths[Searchpath::BinaryDir] = tmp;
	} else {
		_searchpaths[Searchpath::BinaryDir].clear();
	}

	if (cwd[0] != '\0') {
		/* Go back to the current working directory. */
		if (chdir(cwd) != 0) {
			Debug(misc, 0, "Failed to return to working directory!");
		}
	}

#if !defined(GLOBAL_DATA_DIR)
	_searchpaths[Searchpath::InstallationDir].clear();
#else
	tmp = GLOBAL_DATA_DIR;
	AppendPathSeparator(tmp);
	_searchpaths[Searchpath::InstallationDir] = std::move(tmp);
#endif
#ifdef WITH_COCOA
extern void CocoaSetApplicationBundleDir();
	CocoaSetApplicationBundleDir();
#else
	_searchpaths[Searchpath::ApplicationBundleDir].clear();
#endif

	/* Look for Atari release of Transport Tycoon Deluxe for original data files */
	std::string config_file_path;
	const std::string atari_ini_filename = "Atari/Transport Tycoon Deluxe/installpath.ini";

	_searchpaths[Searchpath::TransportTycoonDeluxeDir].clear();

#ifdef WITH_COCOA
extern std::string CocoaGetAppSupportDir();
	config_file_path = CocoaGetAppSupportDir();

	if (!config_file_path.empty()) {
		AppendPathSeparator(config_file_path);
		config_file_path += atari_ini_filename;
	}
#else
	config_file_path = GetHomeDir();

	if (!config_file_path.empty()) {
		AppendPathSeparator(config_file_path);
		config_file_path += ".local/share/";
		config_file_path += atari_ini_filename;
	}
#endif

	if (!config_file_path.empty()) {
		size_t installpath_len;
		std::unique_ptr<char[]> installpath = ReadFileToMem(config_file_path, installpath_len, MAX_PATH);

		if (installpath != nullptr && installpath_len > 0) {
			std::string ttd_path = installpath.get();
			AppendPathSeparator(ttd_path);

#ifdef WITH_COCOA
			/* The path provided is to the TTD.app/Contents/MacOS folder */
			ttd_path += "../Resources/";
#endif

			ttd_path += "CD";
			AppendPathSeparator(ttd_path);

			if (FileExists(ttd_path)) _searchpaths[Searchpath::TransportTycoonDeluxeDir] = ttd_path;
		}
	}
}
 */

/**
 * Acquire the base paths (personal dir and game data dir),
 * fill all other paths (save dir, autosave dir etc) and
 * make the save and scenario directories.
 * @param exe the path from the current path to the executable
 * @param only_local_path Whether we shouldn't fill searchpaths with global folders.
 */
pub fn determine_paths(exe: &Path, only_local_path: bool) {

}
/*

void determine_paths(std::string_view exe, bool only_local_path)
{
	DetermineBasePaths(exe);
	FillValidSearchPaths(only_local_path);

#ifdef USE_XDG
	std::string config_home;
	std::string homedir = GetHomeDir();
	if (auto xdg_config_home = GetEnv("XDG_CONFIG_HOME"); xdg_config_home.has_value()) {
		config_home = *xdg_config_home;
		config_home += PATHSEP;
		config_home += PERSONAL_DIR[0] == '.' ? &PERSONAL_DIR[1] : PERSONAL_DIR;
	} else if (!homedir.empty()) {
		/* Defaults to ~/.config */
		config_home = std::move(homedir);
		config_home += PATHSEP ".config" PATHSEP;
		config_home += PERSONAL_DIR[0] == '.' ? &PERSONAL_DIR[1] : PERSONAL_DIR;
	}
	AppendPathSeparator(config_home);
#endif

	for (Searchpath sp : _valid_searchpaths) {
		if (sp == Searchpath::WorkingDir && !_do_scan_working_directory) continue;
		Debug(misc, 3, "{} added as search path", _searchpaths[sp]);
	}

	std::string config_dir;
	if (!_config_file.empty()) {
		config_dir = _searchpaths[Searchpath::WorkingDir];
	} else {
		std::string personal_dir = FioFindFullPath(Subdirectory::Base, "openttd.cfg");
		if (!personal_dir.empty()) {
			auto end = personal_dir.find_last_of(PATHSEPCHAR);
			if (end != std::string::npos) personal_dir.erase(end + 1);
			config_dir = std::move(personal_dir);
		} else {
#ifdef USE_XDG
			/* No previous configuration file found. Use the configuration folder from XDG. */
			config_dir = config_home;
#else
			static const Searchpath new_openttd_cfg_order[] = {
					Searchpath::PersonalDir, Searchpath::BinaryDir, Searchpath::WorkingDir, Searchpath::SharedDir, Searchpath::InstallationDir
				};

			config_dir.clear();
			for (const auto &searchpath : new_openttd_cfg_order) {
				if (IsValidSearchPath(searchpath)) {
					config_dir = _searchpaths[searchpath];
					break;
				}
			}
#endif
		}
		_config_file = config_dir + "openttd.cfg";
	}

	Debug(misc, 1, "{} found as config directory", config_dir);

	_highscore_file = config_dir + "hs.dat";
	extern std::string _hotkeys_file;
	_hotkeys_file = config_dir + "hotkeys.cfg";
	extern std::string _windows_file;
	_windows_file = config_dir + "windows.cfg";
	extern std::string _private_file;
	_private_file = config_dir + "private.cfg";
	extern std::string _secrets_file;
	_secrets_file = config_dir + "secrets.cfg";
	extern std::string _favs_file;
	_favs_file = config_dir + "favs.cfg";

#ifdef USE_XDG
	if (config_dir == config_home) {
		/* We are using the XDG configuration home for the config file,
		 * then store the rest in the XDG data home folder. */
		_personal_dir = _searchpaths[Searchpath::PersonalDirXdg];
		if (only_local_path) {
			/* In case of XDG and we only want local paths and we detected that
			 * the user either manually indicated the XDG path or didn't use
			 * "-c" option, we change the working-dir to the XDG personal-dir,
			 * as this is most likely what the user is expecting. */
			_searchpaths[Searchpath::WorkingDir] = _searchpaths[Searchpath::PersonalDirXdg];
		}
	} else
#endif
	{
		_personal_dir = config_dir;
	}

	/* Make the necessary folders */
	FioCreateDirectory(config_dir);
#if defined(WITH_PERSONAL_DIR)
	FioCreateDirectory(_personal_dir);
#endif

	Debug(misc, 1, "{} found as personal directory", _personal_dir);

	static const Subdirectory default_subdirs[] = {
		Subdirectory::Save, Subdirectory::Autosave, Subdirectory::Scenario, Subdirectory::Heightmap, Subdirectory::Baseset, Subdirectory::NewGrf, Subdirectory::Ai, Subdirectory::AiLibrary, Subdirectory::Gs, Subdirectory::GsLibrary, Subdirectory::Screenshot, Subdirectory::SocialIntegration
	};

	for (const auto &default_subdir : default_subdirs) {
		FioCreateDirectory(fmt::format("{}{}", _personal_dir, _subdirs[default_subdir]));
	}

	/* If we have network we make a directory for the autodownloading of content */
	_searchpaths[Searchpath::AutodownloadDir] = _personal_dir + "content_download" PATHSEP;
	Debug(misc, 3, "{} added as search path", _searchpaths[Searchpath::AutodownloadDir]);
	FioCreateDirectory(_searchpaths[Searchpath::AutodownloadDir]);
	FillValidSearchPaths(only_local_path);

	/* Create the directory for each of the types of content */
	const Subdirectory subdirs[] = { Subdirectory::Scenario, Subdirectory::Heightmap, Subdirectory::Baseset, Subdirectory::NewGrf, Subdirectory::Ai, Subdirectory::AiLibrary, Subdirectory::Gs, Subdirectory::GsLibrary, Subdirectory::SocialIntegration };
	for (const auto &subdir : subdirs) {
		FioCreateDirectory(FioGetDirectory(Searchpath::AutodownloadDir, subdir));
	}

	extern std::string _log_file;
	_log_file = _personal_dir + "openttd.log";
}
 */