use regex::Regex;
use regex_static::once_cell::sync::Lazy;

// Matches an hazard declared in the `Hazard::HazardName` form.
pub(crate) static HAZARD_RE: Lazy<Regex> = regex_static::lazy_regex!(r"Hazard::(\w+)");

// Matches the arguments (content between round brackests) of a function call.
//
// Given for example `DeviceAction::with_hazards(toggle_config, toggle, &[Hazard::FireHazard, Hazard::PowerSurge])`
// it will match `toggle_config, toggle, &[Hazard::FireHazard, Hazard::PowerSurge]`.
pub(crate) static ARGS_RE: Lazy<Regex> = regex_static::lazy_regex!(r"\((?s)(.*?)\)");

// Regex that that matches a method invokation
// that has one of the following forms:
//
// - `expression(content).`
// - `expression(content)?.`
#[inline(always)]
pub(crate) fn method_re(expression: &str) -> Option<Regex> {
    Regex::new(&format!(r"(?s){}\((.*?)\)\s*\??\s*\.", expression)).ok()
}
