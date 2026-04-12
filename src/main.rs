use leptos::prelude::*;
use rhai::{Dynamic, Engine, Scope, packages::Package};

#[component]
pub fn LuaAlternative() -> impl IntoView {
    let (value, set_value) = signal(String::new());
    let (code, set_code) = signal(String::new());
    let (text, set_text) = signal(String::new());

    let run_script = {
        move |_| {
            let engine = Engine::new();

            let mut scope = Scope::new();

            match engine.compile(code.get()) {
                Ok(ast) => {
                    match engine.call_fn::<Dynamic>(&mut scope, &ast, "process", (text.get(),)) {
                        Ok(res) => {
                            if let Ok(r) = serde_json::to_string(&res) {
                                *set_value.write() = r;
                            }
                        }
                        Err(e) => {
                            *set_value.write() = format!("Error: {}", e);
                        }
                    }
                }
                Err(e) => {
                    *set_value.write() = format!("Error: {}", e);
                }
            }
        }
    };

    view! {
        <div class="min-h-screen bg-slate-900 text-slate-100 p-4 md:p-8 font-sans">
                <div class="max-w-5xl mx-auto space-y-6">

                    <div class="flex items-center justify-between border-b border-slate-700 pb-4">
                        <h1 class="text-xl font-bold tracking-tight text-orange-500">"Rhai Playground"</h1>
                        <button
                            on:click=run_script
                            class="bg-orange-600 hover:bg-orange-500 text-white px-6 py-2 rounded-md font-semibold transition-all shadow-lg active:scale-95"
                        >
                            "▶ Запустить"
                        </button>
                    </div>

                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                        <div class="flex flex-direction-col space-y-2">
                            <label class="text-sm font-medium text-slate-400">"Скрипт (Rhai)"</label>
                            <textarea
                                class="w-full h-[400px] bg-slate-800 border border-slate-700 rounded-lg p-4 font-mono text-sm focus:ring-2 focus:ring-orange-500 focus:outline-none resize-none"
                                prop:value=move || code.get()
                                on:input:target=move |ev| *set_code.write() = ev.target().value()
                            >
                                {move || code.get()}
                            </textarea>
                        </div>

                        <div class="space-y-6">
                            <div class="space-y-2">
                                <label class="text-sm font-medium text-slate-400">"Входные данные (Text)"</label>
                                <textarea
                                    class="w-full h-32 bg-slate-800 border border-slate-700 rounded-lg p-3 font-mono text-sm focus:ring-2 focus:ring-blue-500 focus:outline-none"
                                    prop:value=move || text.get()
                                    on:input:target=move |ev| *set_text.write() = ev.target().value()
                                >
                                    {move || text.get()}
                                </textarea>
                            </div>

                            <div class="space-y-2">
                                <label class="text-sm font-medium text-slate-400">"Результат выполнения"</label>
                                <div class="w-full min-h-[210px] bg-black/50 border border-slate-700 rounded-lg p-4 font-mono text-green-400 overflow-auto shadow-inner">
                                    <span class="text-slate-500 mr-2">">"</span>
                                    {move || value.get()}
                                </div>
                            </div>
                        </div>
                    </div>

                    
                    <div class="text-[10px] text-slate-600 text-right uppercase tracking-widest">
                        "Powered by Rust & Leptos"
                    </div>
                </div>
            </div>
    }
}

fn main() {
    leptos::mount::mount_to_body(LuaAlternative)
}
