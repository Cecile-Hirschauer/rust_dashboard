use leptos::*;
use crate::app::components::Header;

#[component]
pub fn TeamPage() -> impl IntoView {
    view! {
        <body class="bg-gray-900 overflow-hidden text-gray-100">
            <div class="w-full max-w-[64rem] mx-auto items-center flex align-center justify-center">
                <Header />
            "Team Page here"
            </div>
        </body>
    }
}
