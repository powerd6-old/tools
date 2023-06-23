mod split_lines;

pub(crate) fn setup_helpers(
    mut handlebars: handlebars::Handlebars<'_>,
) -> handlebars::Handlebars<'_> {
    handlebars.register_helper("splitLines", Box::new(split_lines::split_lines));
    handlebars
}
