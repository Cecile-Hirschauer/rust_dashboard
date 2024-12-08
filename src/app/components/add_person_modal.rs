use crate::app::components::{ Toast, ToastMessage, ToastMessageType };
use crate::app::models::AddPersonRequest;
use crate::app::server_functions::persons::add_person;
use leptos::*;
use validator::Validate;

#[component]
pub fn AddPersonModal(
    set_if_show_modal: WriteSignal<bool>,
    set_if_show_added: WriteSignal<bool>,
    set_toast_message: WriteSignal<ToastMessage>
) -> impl IntoView {
    const INPUT_STYLE: &str =
        "w-full h-12 bg-[#333333] border-none rounded \
        px-6 py-4 text-white mt-4 \
        outline-none focus:outline-none";

    const CANCEL_BUTTON_STYLE: &str =
        "bg-[#555555] px-8 py-2 rounded text-white \
        mr-3 hover:bg-[#666666]";

    const ADD_BUTTON_STYLE: &str =
        "bg-[#7734e7] px-8 py-2 rounded text-white \
        hover:bg-[#8448e9]";

    // field values
    let (person_name, set_person_name) = create_signal(String::new());
    let (person_title, set_person_title) = create_signal(String::new());
    let (person_level, set_person_level) = create_signal(String::new());
    let (compensation, set_compensation) = create_signal(String::new());

    // for error message(s)
    let (error_message, set_error_message) = create_signal(String::new());
    let (if_error, set_if_error) = create_signal(false);

    let on_close = move |_| set_if_show_modal(false);

    let on_click = move |_| {
        let add_person_request = AddPersonRequest::new(
            person_name(),
            person_title(),
            person_level(),
            compensation().parse::<i32>().expect("Numbers only")
        );

        let is_valid = add_person_request.validate();

        match is_valid {
            Ok(_) => {
                spawn_local(async move {
                    let add_result = add_person(add_person_request).await;

                    // we get the result back and do something with it
                    match add_result {
                        Ok(_added_person) => {
                            set_if_show_modal(false);

                            set_toast_message(
                                ToastMessage::create(ToastMessageType::NewMemberAdded)
                            );

                            // setting this to true to make the toast
                            // for "new member added" appear
                            set_if_show_added(true);
                        }
                        Err(e) => println!("Error adding: {:?}", e),
                    }
                });
            }
            Err(_) => {
                set_if_error(true);
                set_error_message(String::from("All fields are required"))
            }
        }
    };

    view! {
        <div class="fixed inset-0 flex items-center justify-center bg-black/50">
            <div class="bg-[#222222] rounded-lg w-full max-w-[36rem]">
                <div class="px-6 py-5">
                    <h2 class="text-white text-xl">"Add New Employee"</h2>
                    
                    <input
                        type="text"
                        placeholder="Name"
                        class=INPUT_STYLE
                        value=person_name
                        on:input=move |event| {
                            set_person_name(event_target_value(&event));
                        }
                    />
                    <input
                        type="text"
                        placeholder="Title"
                        class=INPUT_STYLE
                        value=person_title
                        on:input=move |event| {
                            set_person_title(event_target_value(&event));
                        }
                    />
                    <input
                        type="text"
                        placeholder="Level"
                        class=INPUT_STYLE
                        value=person_level
                        on:input=move |event| {
                            set_person_level(event_target_value(&event));
                        }
                    />
                    <input
                        type="text"
                        placeholder="Compensation"
                        class=INPUT_STYLE
                        value=compensation
                        on:input=move |event| {
                            set_compensation(event_target_value(&event));
                        }
                    />

                    <div class="flex justify-end mt-8 space-x-3">
                        <button on:click=on_close class=CANCEL_BUTTON_STYLE>
                            "Cancel"
                        </button>
                        <button on:click=on_click class=ADD_BUTTON_STYLE>
                            "Add"
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
