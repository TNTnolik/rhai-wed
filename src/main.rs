use leptos::{prelude::*};
use rhai::{Engine, Scope};

#[component]
pub fn LuaAlternative() -> impl IntoView {
    let (value, set_value) = signal(0i64);
    let (code, set_code) = signal(String::new());

    let run_script = {
        move |_| {
            let engine = Engine::new();
            let mut scope = Scope::new();
    
            // Пример простого скрипта "let x = 10; x * 2"
            let script = code.get();
    
            if let Ok(result) = engine.eval_with_scope::<i64>(&mut scope, &script) {
                *set_value.write() = result;
            }
        }
    };

    view! {
        <button on:click=run_script> "Запустить Rhai" </button>
        <textarea 
            prop:value=move || code.get()
            on:input:target=move |ev| *set_code.write()=ev.target().value()
            >{code}</textarea>
        <p>"Результат: " {move || value.get()}</p>
    }
}

fn main() {
    leptos::mount::mount_to_body(LuaAlternative)
}