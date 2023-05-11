use super::{
    BinField, BugsField, ExportsField, ManField, PackageJsonPerson, RepositoryField, TypeEnum,
};
use serde::Deserialize;
use std::{collections::HashMap, error::Error, fs, io::BufReader, path::Path};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    /// The name is what your thing is called.
    ///
    /// Some rules:
    /// - The name must be less than or equal to 214 characters. This includes the scope for scoped packages.
    /// - The name can’t start with a dot or an underscore.
    /// - New packages must not have uppercase letters in the name.
    /// - The name ends up being part of a URL, an argument on the command line, and a folder
    /// name. Therefore, the name can’t contain any non-URL-safe characters.
    pub name: Option<String>,
    /// Version must be parseable by `node-semver`, which is bundled with npm as a dependency. (`npm install semver` to use it yourself.)
    pub version: Option<String>,
    /// Put a description in it. It’s a string. This helps people discover your package, as it’s listed in `npm search`.
    pub description: Option<String>,
    /// Put keywords in it. It’s an array of strings. This helps people discover your package as it’s listed in `npm search`.
    pub keywords: Option<Vec<String>>,
    /// The url to the project homepage.
    pub homepage: Option<String>,
    /// The url to your project’s issue tracker and / or the email address to which issues should be reported. These are helpful for people who encounter issues with your package.
    pub bugs: Option<BugsField>,
    /// You should specify a license for your package so that people know how they are permitted to use it, and any restrictions you’re placing on it.
    pub license: Option<String>,
    /// Specify the place where your code lives. This is helpful for people who want to contribute. If the git repo is on GitHub, then the `npm docs` command will be able to find you.
    ///
    /// For GitHub, GitHub gist, Bitbucket, or GitLab repositories you can use the same shortcut syntax you use for npm install:
    pub repository: Option<RepositoryField>,
    pub scripts: Option<HashMap<String, String>>,
    /// If you set `"private": true` in your package.json, then npm will refuse to publish it.
    pub private: Option<bool>,
    /// The “author” is one person.
    pub author: Option<PackageJsonPerson>,
    /// “contributors” is an array of people.
    pub contributors: Option<Vec<PackageJsonPerson>>,
    /// The optional `files` field is an array of file patterns that describes the entries to be included when your package is installed as a dependency. File patterns follow a similar syntax to `.gitignore`, but reversed: including a file, directory, or glob pattern (`*`, `**\/*`, and such) will make it so that file is included in the tarball when it’s packed. Omitting the field will make it default to `["*"]`, which means it will include all files.
    pub files: Option<Vec<String>>,
    /// The main field is a module ID that is the primary entry point to your program. That is, if your package is named `foo`, and a user installs it, and then does `require("foo")`, then your main module’s exports object will be returned.
    ///
    /// This should be a module ID relative to the root of your package folder.
    ///
    /// For most modules, it makes the most sense to have a main script and often not much else.
    pub main: Option<String>,
    /// If your module is meant to be used client-side the browser field should be used instead of the main field. This is helpful to hint users that it might rely on primitives that aren’t available in Node.js modules. (e.g. window)
    pub browser: Option<String>,
    /// A map of command name to local file name. On install, npm will symlink that file into `prefix/bin` for global installs, or `./node_modules/.bin/` for local installs.
    pub bin: Option<BinField>,
    /// Specify either a single file or an array of filenames to put in place for the `man` program to find.
    pub man: Option<ManField>,
    /// Dependencies are specified in a simple object that maps a package name to a version range. The version range is a string which has one or more space-separated descriptors. Dependencies can also be identified with a tarball or git URL.
    pub dependencies: Option<HashMap<String, String>>,
    /// If someone is planning on downloading and using your module in their program, then they probably don’t want or need to download and build the external test or documentation framework that you use.
    ///
    /// In this case, it’s best to map these additional items in a `devDependencies` object.
    pub dev_dependencies: Option<HashMap<String, String>>,
    /// If a dependency can be used, but you would like npm to proceed if it cannot be found or fails to install, then you may put it in the `optionalDependencies` object. This is a map of package name to version or url, just like the `dependencies` object. The difference is that build failures do not cause installation to fail.
    pub optional_dependencies: Option<HashMap<String, String>>,
    /// In some cases, you want to express the compatibility of your package with a host tool or library, while not necessarily doing a `require` of this host. This is usually referred to as a plugin. Notably, your module may be exposing a specific interface, expected and specified by the host documentation.
    pub peer_dependencies: Option<HashMap<String, String>>,
    /// TypeScript typings, typically ending by .d.ts
    pub types: Option<String>,
    pub typings: Option<String>,
    /// Non-Standard Node.js alternate entry-point to main.
    ///
    /// An initial implementation for supporting CJS packages (from main), and use module for ESM modules.
    pub module: Option<String>,
    /// Make main entry-point be loaded as an ESM module, support "export" syntax instead of "require"
    ///
    /// Docs:
    /// - https://nodejs.org/docs/latest-v14.x/api/esm.html#esm_package_json_type_field
    ///
    /// @default 'commonjs'
    ///
    /// @since Node.js v14
    #[serde(default = "TypeEnum::default")]
    pub type_: TypeEnum,

    /// Alternate and extensible alternative to "main" entry point.
    ///
    /// When using `{type: "module"}`, any ESM module file MUST end with `.mjs` extension.
    ///
    /// Docs:
    ///
    /// - <https://nodejs.org/docs/latest-v14.x/api/esm.html#esm_exports_sugar>
    ///
    /// @default 'commonjs'
    ///
    /// @since Node.js v14
    pub exports: Option<ExportsField>,
    pub workspaces: Option<Vec<String>>,
}

impl Default for PackageJson {
    /// default returns a pkg.
    fn default() -> Self {
        PackageJson {
            name: None,
            version: None,
            description: None,
            keywords: None,
            homepage: None,
            bugs: None,
            license: None,
            repository: None,
            scripts: None,
            private: None,
            author: None,
            contributors: None,
            files: None,
            main: None,
            browser: None,
            bin: None,
            man: None,
            dependencies: None,
            dev_dependencies: None,
            optional_dependencies: None,
            peer_dependencies: None,
            types: None,
            typings: None,
            module: None,
            type_: TypeEnum::Commonjs,
            exports: None,
            workspaces: None,
        }
    }
}

impl PackageJson {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let file = fs::File::open(path)?;

        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `PackageJson`.
        let pkg_json: PackageJson = serde_json::from_reader(reader)?;

        Ok(pkg_json)
    }
}
