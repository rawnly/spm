/// Creates a fuzzy matcher and scorer variable for use with `inquire::Select`.
/// Spaces in the input are stripped so "my proj" matches "my-project".
///
/// Usage: `fuzzy_scorer!(scorer_name, Type);`
#[macro_export]
macro_rules! fuzzy_scorer {
    ($name:ident, $T:ty) => {
        let matcher =
            ::std::cell::RefCell::new(::frizbee::Matcher::new("", &::frizbee::Config::default()));
        let $name: ::inquire::type_aliases::Scorer<$T> = &|input, _, str_val, _| {
            if input.is_empty() {
                return Some(0);
            }
            let needle = input.replace(' ', "");
            let mut m = matcher.borrow_mut();
            m.set_needle(&needle);
            m.smith_waterman_one(str_val.as_bytes(), 0, true)
                .map(|r| r.score as i64)
        };
    };
}
