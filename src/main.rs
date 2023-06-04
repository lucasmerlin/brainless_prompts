#![allow(non_snake_case)]

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch(App);
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

fn make_prompt() -> String {
    let NOUNS: &str = include_str!("./nouns.txt").trim();
    let ADJECTIVES: &str = include_str!("./adjectives.txt").trim();
    let VERBS: &str = include_str!("./verbs.txt").trim();
    let PREPOSITIONS: &str = include_str!("prepositions.txt").trim();

    let split_nouns: Vec<&str> = NOUNS.split("\n").collect();
    let split_adjectives: Vec<&str> = ADJECTIVES.split("\n").collect();
    let split_verbs: Vec<&str> = VERBS.split("\n").collect();
    let split_prepositions: Vec<&str> = PREPOSITIONS.split("\n").collect();

    let mut rng = rand::thread_rng();

    let noun_1 = split_nouns.choose(&mut rng).unwrap().to_lowercase();
    let noun_2 = split_nouns.choose(&mut rng).unwrap().to_lowercase();
    let adjective_1 = split_adjectives.choose(&mut rng).unwrap().to_lowercase();
    let prep = split_prepositions.choose(&mut rng).unwrap().to_lowercase();

    format!(
        "{} {} {} {} {} {} {}",
        make_article(&adjective_1),
        adjective_1,
        noun_1,
        split_verbs.choose(&mut rng).unwrap().to_lowercase(),
        prep,
        make_article(&noun_2).to_lowercase(),
        noun_2
    )
}

fn make_article(inputstring: &String) -> String {
    let first_char = inputstring.chars().nth(0).unwrap();
    let vocal_vec = vec!['a', 'e', 'i', 'o', 'u'];

    if vocal_vec.contains(&first_char) {
        "An".to_string()
    } else {
        "A".to_string()
    }
}

fn make_random_color() -> (String, String) {
    let COLORS: &str = include_str!("./colors.txt").trim();
    let mut split: Vec<&str> = COLORS.split("\n").collect();
    let mut rng = rand::thread_rng();
    let first_color_index = rng.gen_range(0..split.len());
    let first_color = split.remove(first_color_index);
    let second_color = rng.gen_range(0..split.len());

    return (first_color.to_string(), split[second_color].to_string());
}

fn App(cx: Scope) -> Element {
    let mut all_colors = use_state(cx, || make_random_color());

    let mut state = use_state(cx, || make_prompt());

    let mut copied = use_state(cx, || false);

    let color_0 = &all_colors.0;
    let color_1 = &all_colors.1;

    let copied_clone = copied.clone();

    let copied_class = if **copied { "copied" } else { "not-copied" };

    cx.render(rsx! {

        style {
            include_str!("style.css")
        }

        div {
            class: "container",
                background: "linear-gradient({color_0}, {color_1})",


            div {
                class: "prompt {copied_class}",
                state.as_str()
            }
            button {
                class:"button",
                onclick: move |_| {
                    state.set(make_prompt());
                    all_colors.set(make_random_color());
                },
                "Generate new prompt"
            }
            button {
                class:"button",
                onclick: move |_| {
                    let prompt = state.as_str();
                    let mut clipboard = web_sys::window().unwrap().navigator().clipboard().unwrap();
                    let copied_clone = copied_clone.clone();
                    let copied_clone_2 = copied_clone.clone();
                    let closure = Closure::new(move |_| {
                        copied_clone.set(true);
                    });
                    clipboard.write_text(&prompt).then(&closure);
                    closure.forget();

                    let f = Closure::wrap(Box::new(move || {
                        copied_clone_2.set(false);
                    }) as Box<dyn FnMut()>);

                    web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(
                        f.as_ref().unchecked_ref(),
                        500,
                    ).unwrap();

                    f.forget();
                },
                "Copy prompt"
            }
        }

    })
}
