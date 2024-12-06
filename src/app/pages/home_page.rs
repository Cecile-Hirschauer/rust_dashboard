use leptos::*;
use crate::app::components::Header;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <body class="bg-gray-900 overflow-hidden">
            <div class="w-full max-w-[64rem] mx-auto items-center flex align-center justify-center">
            "home Page here"
                <Header />
            </div>
        </body>
    }
}
