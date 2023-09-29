use crate::error::Result as AvoRedResult;
use handlebars::{Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError};
use r_i18n::{I18n, I18nConfig};
pub struct AvoRedViewProvider {
    pub handlebars: Handlebars<'static>,
}

impl AvoRedViewProvider {
    pub fn register() -> AvoRedResult<AvoRedViewProvider> {
        let mut handlebars = Handlebars::new();

        handlebars
            .register_templates_directory(".hbs", "./views")
            .expect("handlebars cant register the views path");

        handlebars.register_helper("is_vector_contains", Box::new(is_vector_contains));
        handlebars.register_helper("translate_key", Box::new(translate_key));
        handlebars.register_helper("get_validation_message", Box::new(get_validation_message));

        Ok(AvoRedViewProvider { handlebars })
    }
}

pub fn translate(key: &str) -> String {
    let config: I18nConfig = I18nConfig {
        locales: &["en", "fr"],
        directory: "locales",
    };
    let r_i18n: I18n = I18n::configure(&config);

    let translated_text = match r_i18n.translations.get("en") {
        Some(language_json) => {
            if language_json.has_key(key) {
                language_json[key].to_string()
            } else {
                String::from(key)
            }
        }
        None => String::from(key),
    };
    translated_text
}

fn translate_key(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    // get parameter from helper or throw an error
    let param = h
        .param(0)
        .ok_or(RenderError::new(&format!("param 0")))
        .unwrap();
    let param_value: String = param.value().render();

    let tran = translate(param_value.as_str());

    write!(out, "{}", tran).unwrap();
    Ok(())
}

fn get_validation_message(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or(RenderError::new(&format!("param 0")))
        .unwrap();
    let param_value: String = param.value().render();
    write!(
        out,
        "{}",
        String::from(format!("Hello Validation {}", param_value))
    )
    .unwrap();

    Ok(())
}

fn is_vector_contains(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let vector = h
        .param(0)
        .ok_or(RenderError::new(&format!("param 0")))
        .unwrap();
    let vector_value = match vector.value().as_array() {
        Some(arr) => arr.clone(),
        None => Vec::new(),
    };

    let find_key = h
        .param(1)
        .ok_or(RenderError::new(&format!("param 1")))
        .unwrap();
    let find_key_value = find_key.value().render();

    let mut find_key_result = "";

    if vector_value.into_iter().any(|i| {
        // Try to fix this trim issue
        let index = i.to_string();
        let trim_str = index.trim_matches('"');

        trim_str.eq(&find_key_value)
    }) {
        // this value wont be used anyhow yet.
        find_key_result = "found";
    }

    write!(out, "{}", find_key_result).unwrap();
    Ok(())
}
