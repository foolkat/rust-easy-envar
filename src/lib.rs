/// Defines environment variables.
/// 
/// ***
/// # Examples
/// 
/// ```rust,no_run
/// // build.rs
/// use easy_envar::Envar;
/// 
/// fn main() {
///     easy_envar::init().unwrap();
/// 
///     let env_vars = [
///         Envar::String("HOST"),
///         Envar::U16("PORT"),
///         Envar::U32("DATA"),
///         Envar::Bool("SECURE"),
///     ];
/// }
/// ```
#[derive(Debug)]
pub enum Envar<'a> {
    /// A boolean type environment variable.
    /// 
    /// ***
    /// # Examples
    /// 
    /// ```rust
    /// use easy_envar::Envar;
    /// 
    /// let env_var = Envar::Bool("VAR_NAME");
    /// ```
    Bool(&'a str),

    /// A string type environment variable.
    /// 
    /// ***
    /// # Examples
    /// 
    /// ```rust
    /// use easy_envar::Envar;
    /// 
    /// let env_var = Envar::String("VAR_NAME");
    /// ```
    String(&'a str),

    /// A u16 type environment variable.
    /// 
    /// ***
    /// # Examples
    /// 
    /// ```rust
    /// use easy_envar::Envar;
    /// 
    /// let env_var = Envar::U16("VAR_NAME");
    /// ```
    U16(&'a str),

    /// A u32 type environment variable.
    /// 
    /// ***
    /// # Examples
    /// 
    /// ```rust
    /// use easy_envar::Envar;
    /// 
    /// let env_var = Envar::U32("VAR_NAME");
    /// ```
    U32(&'a str),
}


/// Represents an environment variable whose value has already been loaded.
///
/// ***
/// # Examples
///
/// ```rust,no_run
/// use easy_envar::{Envar, LoadedEnvar};
///
/// let env_var: Envar = Envar::String("VAR_NAME");
///
/// let loaded_env_var: LoadedEnvar = env_var.load().unwrap();
/// ```
#[derive(Debug)]
pub enum LoadedEnvar<'a> {
    /// A loaded `String` environment variable.
    ///
    /// The first field is the environment variable name.
    /// The second field is the string value that was loaded.
    String(&'a str, String),

    /// A loaded `bool` environment variable.
    ///
    /// The first field is the environment variable name.
    /// The second field is the boolean value that was loaded.
    Bool(&'a str, bool),

    /// A loaded `u16` environment variable.
    ///
    /// The first field is the environment variable name.
    /// The second field is the `u16` value that was loaded.
    U16(&'a str, u16),

    /// A loaded `u32` environment variable.
    ///
    /// The first field is the environment variable name.
    /// The second field is the `u32` value that was loaded.
    U32(&'a str, u32),
}


impl<'a> Envar<'a> {
    /// Loads the environment variable's value from the system environment,
    /// then attempts to parse it into the corresponding data type.
    ///
    /// ***
    /// # Returns
    ///
    /// - `Ok(LoadedEnvar)`: if the value is successfully retrieved and parsed.
    /// - `Err(..)`: if the environment variable is missing or the value is invalid for the expected type.
    ///
    /// ***
    /// # Examples
    ///
    /// ```rust,no_run
    /// // build.rs
    /// use easy_envar::Envar;
    ///
    /// fn main() {
    ///     easy_envar::init().unwrap();
    ///
    ///     let env_var = Envar::String("VAR_NAME");
    ///
    ///     env_var.load().unwrap();
    /// }
    /// ```
    pub fn load(&self) -> Result<LoadedEnvar, Box<dyn std::error::Error>> {
        let key = match self {
            Envar::String(key) |
            Envar::Bool(key) |
            Envar::U16(key) |
            Envar::U32(key) => *key,
        };

        let raw = std::env::var(key)?;

        match self {
            Envar::String(_) => {
                let val = raw;
                Ok(LoadedEnvar::String(key, val))
            },
            Envar::Bool(_) => {
                let val = raw.parse::<bool>()?;
                Ok(LoadedEnvar::Bool(key, val))
            },
            Envar::U16(_) => {
                let val = raw.parse::<u16>()?;
                Ok(LoadedEnvar::U16(key, val))
            },
            Envar::U32(_) => {
                let val = raw.parse::<u32>()?;
                Ok(LoadedEnvar::U32(key, val))
            }
        }
    }
}


impl<'a> LoadedEnvar<'a> {
    /// Exports this loaded environment variable as a Cargo build directive (`cargo::rustc-env`).
    ///
    /// When invoked in a build script, this method prints a line that instructs
    /// Cargo to set the specified environment variable for subsequent compiler
    /// invocations. This is useful for scenarios where you need to make certain
    /// environment variables available at compile time (e.g., via `env!` in your code).
    ///
    /// ***
    /// # Examples
    ///
    /// ```rust
    /// // build.rs
    /// use easy_envar::LoadedEnvar;
    /// 
    /// fn main() {
    ///     let loaded_var = LoadedEnvar::String("VAR_NAME", "some_value".to_string());
    ///     loaded_var.export();
    /// }
    /// ```
    /// 
    /// ***
    /// 
    /// ```rust,ignore
    /// # use easy_envar::LoadedEnvar;
    /// # let loaded_var = LoadedEnvar::String("VAR_NAME", "some_value".to_string());
    /// # loaded_var.export();
    /// 
    /// // main.rs
    /// 
    /// fn main() {
    ///     let value = std::env!("VAR_NAME");
    ///     assert_eq!(value, "some_value".to_string());
    /// }
    /// ```
    pub fn export(&self) {
        let (key, val) = match self {
            LoadedEnvar::String(key, val) => (*key, val.clone()),
            LoadedEnvar::Bool(key, val)   => (*key, val.to_string()),
            LoadedEnvar::U16(key, val)    => (*key, val.to_string()),
            LoadedEnvar::U32(key, val) => (*key, val.to_string()),
        };
        println!("cargo:rustc-env={}={}", key, val);
    }
}


/// Loads the `.env` file from the root directory of your project.
/// 
/// This function simply calls `dotenvy::dotenv()`.
///
/// ***
/// # Examples
///
/// ```rust,no_run
/// // build.rs
///
/// fn main() {
///     easy_envar::init().unwrap();
/// }
/// ```
pub fn init() -> Result<std::path::PathBuf, dotenvy::Error> {
    dotenvy::dotenv()
}