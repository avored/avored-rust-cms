use handlebars::{Handlebars, Helper, Context, RenderContext, RenderError, Output, JsonRender};
use r_i18n::{I18nConfig, I18n};


pub struct AvoRedViewProvider {
    pub handlebars: Handlebars<'static>
}

impl AvoRedViewProvider {
    pub fn register() -> AvoRedViewProvider {
        let mut handlebars = Handlebars::new();


        handlebars
            .register_templates_directory(".hbs", "./views")
            .expect("handlebars cant register the views path");

        handlebars.register_helper("translate_key", Box::new(translate_key));
        handlebars.register_helper("get_validation_message", Box::new(get_validation_message));

        AvoRedViewProvider {
            handlebars,
        }
    }
}


fn translate(key: String) -> String {
    let config: I18nConfig = I18nConfig {
        locales: &["en", "fr"],
        directory: "locales",
    };
    let r_i18n: I18n = I18n::configure(&config);

    let translated_text = match r_i18n.translations.get("en") {
        Some(language_json) => {
            if language_json.has_key(&key) {
                language_json[&key].to_string()
            } else {
                String::from(&key)
            }
        }
        None => String::from(&key),
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

    let tran = translate(String::from(param_value));

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
