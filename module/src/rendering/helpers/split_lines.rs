use handlebars::handlebars_helper;

handlebars_helper!(split_lines: |paragraph: String| paragraph.split('\n').collect::<Vec<&str>>());
