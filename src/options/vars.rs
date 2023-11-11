use std::ffi::OsString;

// General variables

/// Environment variable used to colour files, both by their filesystem type
/// (symlink, socket, directory) and their file name or extension (image,
/// video, archive);
pub static LS_COLORS: &str = "LS_COLORS";

/// Environment variable used to override the width of the terminal, in
/// characters.
pub static COLUMNS: &str = "COLUMNS";

/// Environment variable used to datetime format.
pub static TIME_STYLE: &str = "TIME_STYLE";

/// Environment variable used to disable colors.
/// See: <https://no-color.org/>
pub static NO_COLOR: &str = "NO_COLOR";

// exa-specific variables

/// Environment variable used to colour exa’s interface when colours are
/// enabled. This includes all the colours that `LS_COLORS` would recognise,
/// overriding them if necessary. It can also contain exa-specific codes.
pub static EXA_COLORS: &str = "EXA_COLORS";
pub static EZA_COLORS: &str = "EZA_COLORS";

/// Environment variable used to switch on strict argument checking, such as
/// complaining if an argument was specified twice, or if two conflict.
/// This is meant to be so you don’t accidentally introduce the wrong
/// behaviour in a script, rather than for general command-line use.
/// Any non-empty value will turn strict mode on.
pub static EXA_STRICT: &str = "EXA_STRICT";
pub static EZA_STRICT: &str = "EZA_STRICT";

/// Environment variable used to make exa print out debugging information as
/// it runs. Any non-empty value will turn debug mode on.
pub static EXA_DEBUG: &str = "EXA_DEBUG";
pub static EZA_DEBUG: &str = "EZA_DEBUG";

/// Environment variable used to limit the grid-details view
/// (`--grid --long`) so it’s only activated if there’s at least the given
/// number of rows of output.
pub static EXA_GRID_ROWS: &str = "EXA_GRID_ROWS";
pub static EZA_GRID_ROWS: &str = "EZA_GRID_ROWS";

/// Environment variable used to specify how many spaces to print between an
/// icon and its file name. Different terminals display icons differently,
/// with 1 space bringing them too close together or 2 spaces putting them too
/// far apart, so this may be necessary depending on how they are shown.
pub static EXA_ICON_SPACING: &str = "EXA_ICON_SPACING";
pub static EZA_ICON_SPACING: &str = "EZA_ICON_SPACING";

pub static EXA_OVERRIDE_GIT: &str = "EXA_OVERRIDE_GIT";
pub static EZA_OVERRIDE_GIT: &str = "EZA_OVERRIDE_GIT";

/// Enviroment variable used to set the minimum luminance in `color_scale`. It's value
/// can be between -100 and 100
pub static EXA_MIN_LUMINANCE: &str = "EXA_MIN_LUMINANCE";
pub static EZA_MIN_LUMINANCE: &str = "EZA_MIN_LUMINANCE";

/// Environment variable used to automate the same behavior as `--icons=auto` if set.
/// Any explicit use of `--icons=WHEN` overrides this behavior.
pub static EZA_ICONS_AUTO: &str = "EZA_ICONS_AUTO";

pub static EZA_STDIN_SEPARATOR: &str = "EZA_STDIN_SEPARATOR";

/// Environment variable used to choose how windows attributes are displayed.
/// Short will display a single character for each set attribute, long will
/// display a comma separated list of descriptions.
pub static EZA_WINDOWS_ATTRIBUTES: &str = "EZA_WINDOWS_ATTRIBUTES";

/// Mockable wrapper for `std::env::var_os`.
pub trait Vars {
    fn get(&self, name: &'static str) -> Option<OsString>;

    /// Get the variable `name` and if not set get the variable `fallback`.
    fn get_with_fallback(&self, name: &'static str, fallback: &'static str) -> Option<OsString> {
        self.get(name).or_else(|| self.get(fallback))
    }

    /// Get the source of the value.  If the variable `name` is set return
    /// `Some(name)` else if the variable `fallback` is set return
    /// `Some(fallback)` else `None`.
    fn source(&self, name: &'static str, fallback: &'static str) -> Option<&'static str> {
        match self.get(name) {
            Some(_) if !name.is_empty() => Some(name),
            _ => self.get(fallback).and(Some(fallback)),
        }
    }
}

// Test impl that just returns the value it has.
#[cfg(test)]
impl Vars for Option<OsString> {
    fn get(&self, _name: &'static str) -> Option<OsString> {
        self.clone()
    }
}

#[cfg(test)]
#[allow(dead_code)]
pub struct MockVars {
    pub columns: OsString,
    pub colors: OsString,
    pub no_colors: OsString,
    pub strict: OsString,
    pub debug: OsString,
    pub grid_rows: OsString,
    pub icon_spacing: OsString,
    pub luminance: OsString,
    pub icons: OsString,
    pub time: OsString,
}

#[cfg(test)]
#[allow(dead_code)]
impl Vars for MockVars {
    fn get(&self, name: &'static str) -> Option<OsString> {
        match name {
            "EXA_STRICT" | "EZA_STRICT" if !self.strict.is_empty() => Some(self.strict.clone()),
            "EZA_COLORS" | "LS_COLORS" | "EXA_COLORS" if !self.colors.is_empty() => {
                Some(self.colors.clone())
            }
            "EXA_DEBUG" | "EZA_DEBUG" if !self.debug.is_empty() => Some(self.debug.clone()),
            "EXA_GRID_ROWS" | "EZA_GRID_ROWS" if !self.grid_rows.is_empty() => {
                Some(self.grid_rows.clone())
            }
            "EXA_ICON_SPACING" | "EZA_ICON_SPACING" if !self.icon_spacing.is_empty() => {
                Some(self.icon_spacing.clone())
            }
            "EXA_MIN_LUMINANCE" | "EZA_MIN_LUMINANCE" if !self.luminance.is_empty() => {
                Some(self.luminance.clone())
            }
            "EZA_ICONS_AUTO" if !self.icons.is_empty() => Some(self.icons.clone()),
            "COLUMNS" if !self.columns.is_empty() => Some(self.columns.clone()),
            "NO_COLOR" if !self.no_colors.is_empty() => Some(self.no_colors.clone()),
            "TIME_STYLE" if !self.time.is_empty() => Some(self.time.clone()),
            _ => None,
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
impl MockVars {
    pub fn set(&mut self, var: &'static str, value: &OsString) {
        match var {
            "EXA_STRICT" | "EZA_STRICT" => self.strict = value.clone(),
            "EZA_COLORS" | "LS_COLORS" | "EXA_COLORS" => self.colors = value.clone(),
            "EXA_DEBUG" | "EZA_DEBUG" => self.debug = value.clone(),
            "EXA_GRID_ROWS" | "EZA_GRID_ROWS" => self.grid_rows = value.clone(),
            "EXA_ICON_SPACING" | "EZA_ICON_SPACING" => self.icon_spacing = value.clone(),
            "EXA_MIN_LUMINANCE" | "EZA_MIN_LUMINANCE" => self.luminance = value.clone(),
            "EZA_ICONS_AUTO" => self.icons = value.clone(),
            "COLUMNS" => self.columns = value.clone(),
            "NO_COLOR" => self.no_colors = value.clone(),
            "TIME_STYLE" => self.time = value.clone(),
            _ => (),
        };
        ()
    }

    pub fn default() -> Self {
        Self {
            columns: OsString::new(),
            colors: OsString::new(),
            no_colors: OsString::new(),
            strict: OsString::new(),
            grid_rows: OsString::new(),
            debug: OsString::new(),
            luminance: OsString::new(),
            icon_spacing: OsString::new(),
            icons: OsString::new(),
            time: OsString::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn set_test() {
        let mut vars = MockVars {
            ..MockVars::default()
        };

        vars.set(TIME_STYLE, &OsString::from("iso"));
        assert_eq!(vars.get(TIME_STYLE), Some(OsString::from("iso")));
    }
}
