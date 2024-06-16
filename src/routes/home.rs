use dioxus::prelude::*;

use super::Route;

#[component]
pub fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog moment"
        }
        div {
            h1 { 
                class: "w-full flex flex-row justify-center items-center",

                "High-Five counter: {count}"
            },

            button {
                class: "bg-teal-400",

                onclick: move |_| count += 1, 
                
                "Up high!" 
            },

            button {
                class: "bg-pink-600",

                onclick: move |_| count -= 1, 
                
                "Down low!" 
            }

			button {
				class: "bg-red-500",
                
				onmousedown: move |ev| {
					ev.stop_propagation();

					tracing::info!("Button Pressed - {:#?}", ev);
				},

				"print something"
			}

            ul {
                for i in 1..=10 {
                    li { "hello, world! {i}" }
                }
            }
        }
    }
}
