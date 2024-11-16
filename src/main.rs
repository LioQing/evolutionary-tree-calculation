use std::collections::BinaryHeap;

use gloo::file::callbacks::FileReader;
use ordered_float::NotNan;
use web_sys::{DragEvent, Event, HtmlInputElement};
use yew::html::TargetCast;
use yew::{html, Callback, Component, Context, Html};

mod domain;

/// The details of a file.
pub struct FileDetails {
    name: String,
    data: String,
}

/// The messages that the application can receive.
pub enum Msg {
    /// A file has been loaded.
    Loaded(FileDetails),

    /// A file has been received.
    File(web_sys::File),

    /// An error has occurred.
    Err(String),

    /// No message.
    None,
}

/// The main application component.
pub struct App {
    reader: Option<FileReader>,
    file: Option<FileDetails>,
    results: Option<BinaryHeap<(NotNan<f64>, String)>>,
    error: Option<String>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            reader: None,
            file: None,
            results: None,
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file) => {
                self.reader = None;
                match domain::solve(&file.data) {
                    Ok(results) => {
                        self.results = Some(results);
                        self.error = None;
                    },
                    Err(err) => {
                        self.results = None;
                        self.error = Some(err);
                    }
                }
                self.file = Some(file);
                true
            }
            Msg::File(file) => {
                let link = ctx.link().clone();
                let name = file.name().clone();

                let task = gloo::file::callbacks::read_as_text(
                    &gloo::file::File::from(file),
                    move |res| link.send_message(Msg::Loaded(FileDetails {
                        data: res.expect("failed to read file"),
                        name,
                    })),
                );
                
                self.reader = Some(task);
                self.error = None;
                true
            }
            Msg::Err(err) => {
                self.error = Some(err);
                true
            }
            Msg::None => {
                self.reader = None;
                self.file = None;
                self.results = None;
                self.error = None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Callback to prevent the default drag behavior.
        let noop_drag = Callback::from(|e: DragEvent| {
            e.prevent_default();
        });

        // Callbacks for the file input.
        let ondrop = ctx.link().callback(|e: DragEvent| {
            e.prevent_default();
            match e.data_transfer() {
                None => return Msg::Err("No data received".to_string()),
                Some(data_transfer) => match data_transfer.files() {
                    None => return Msg::Err("No files received".to_string()),
                    Some(files) => match files.get(0) {
                        None => return Msg::Err("No file received".to_string()),
                        Some(file) => Msg::File(file),
                    },
                },
            }
        });

        // Callback for the file input.
        let onchange = ctx.link().callback(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            match input.files() {
                None => Msg::Err("No files received".to_string()),
                Some(files) => match files.get(0) {
                    None => Msg::Err("No file received".to_string()),
                    Some(file) => Msg::File(file),
                },
            }
        });

        html! {
            <div class="md:container md:mx-auto md:px-36 px-10 py-16 text-center">
                <p class="text-5xl font-bold my-8">{ "Evolutionary Tree Calculation" }</p>
                <div class="flex items-center justify-center w-full">
                    <label
                        for="dropzone-file"
                        class="flex flex-col items-center justify-center w-full h-48
                            rounded-lg cursor-pointer bg-gray-100 hover:bg-gray-200
                            rounded-lg border-2 border-gray-700 shadow-md"
                    >
                        <div class="flex flex-col items-center justify-center pt-5 pb-6">
                            <svg
                                class="w-14 h-14 text-gray-700"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                            >
                                <path
                                    stroke="none"
                                    fill="currentColor"
                                    d="M13,20H6a1,1,0,0,1-1-1V5A1,1,0,0,1,6,4h5V7a3,3,
                                        0,0,0,3,3h3v2a1,1,0,0,0,2,0V9s0,0,0-.06a1.31,1.31,0,0,
                                        0-.06-.27l0-.09a1.07,1.07,0,0,0-.19-.28h0l-6-6h0a1.07,
                                        1.07,0,0,0-.28-.19.32.32,0,0,0-.09,0A.88.88,0,0,0,
                                        12.05,2H6A3,3,0,0,0,3,5V19a3,3,0,0,0,3,3h7a1,1,0,0,0,
                                        0-2ZM13,5.41,15.59,8H14a1,1,0,0,1-1-1ZM8,8a1,1,0,0,0,
                                        0,2H9A1,1,0,0,0,9,8Zm6,4H8a1,1,0,0,0,0,2h6a1,1,0,0,0,
                                        0-2Zm6.71,5.29-2-2a1,1,0,0,0-.33-.21,1,1,0,0,0-.76,0,
                                        1,1,0,0,0-.33.21l-2,2a1,1,0,0,0,1.42,1.42l.29-.3V21a1,
                                        1,0,0,0,2,0V18.41l.29.3a1,1,0,0,0,1.42,0A1,1,0,0,0,
                                        20.71,17.29ZM12,18a1,1,0,0,0,0-2H8a1,1,0,0,0,0,2Z"
                                />
                            </svg>
                            <p class="mv-2 text-l text-gray-700">
                                {"Click to upload or drag and drop"}
                            </p>
                            <p class="text-m text-gray-700">
                                {"any JSON file"}
                            </p>
                        </div>
                        <input
                            id="dropzone-file"
                            type="file"
                            class="hidden"
                            accept=".txt,.json,*"
                            ondrop={&ondrop}
                            onchange={&onchange}
                            ondragover={&noop_drag}
                            ondragenter={&noop_drag}
                        />
                    </label>
                </div>
                { for self.error.as_ref().map(|err| html! {
                    <div
                        class="flex items-center justify-center w-full mt-8 bg-red-100 p-4
                            shadow-md rounded-lg"
                    >
                        <p class="text-red-500">{format!("Error: {err}")}</p>
                    </div>
                })}
                { for self.file.as_ref().map(|file| html! {
                    <div
                        class="flex items-center justify-center w-full mt-8 bg-gray-100 p-4
                            shadow-md rounded-lg"
                    >
                        <p>{format!("Loaded: {}", file.name)}</p>
                    </div>
                })}
                <div
                    class="relative flex flex-col w-full h-full bg-white
                        shadow-md rounded-lg bg-clip-border mt-8"
                >
                    <table class="w-full text-left table-auto min-w-max">
                        <thead>
                            <tr>
                                <th
                                    class="p-4 transition-colors border-b border-slate-300
                                        bg-slate-50"
                                >
                                    <p
                                        class="flex items-center justify-between gap-2
                                            font-normal leading-none text-slate-800"
                                    >
                                        {"Name"}
                                    </p>
                                </th>
                                <th
                                    class="p-4 transition-colors border-b border-slate-300
                                        bg-slate-50"
                                >
                                    <p
                                        class="flex items-center justify-between
                                            gap-2 font-normal leading-none text-slate-800"
                                    >
                                        {"Value"}
                                    </p>
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            { for self.results
                                .as_ref()
                                .map(|results| results
                                    .iter()
                                    .map(|(value, name)| html! {
                                        <tr class="hover:bg-slate-50">
                                            <td class="p-4 border border-slate-200">
                                                <p class="block text text-slate-800">
                                                    {name}
                                                </p>
                                            </td>
                                            <td class="p-4 border border-slate-200">
                                                <p class="block text text-slate-800">
                                                    {value.into_inner()}
                                                </p>
                                            </td>
                                        </tr>
                                    })
                                    .collect::<Html>()
                                )
                            }
                        </tbody>
                    </table>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}