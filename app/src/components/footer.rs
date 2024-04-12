use leptos::*;

#[component]
pub fn HomeFooter() -> impl IntoView {
    view! {
        <footer class="mt-auto w-full max-w-[85rem] py-10 px-4 sm:px-6 lg:px-8 mx-auto">
            <div class="text-center">
                <div class="mt-3">
                    <p class="text-[#CED4DA]">
                        "Made with ❤️ using "
                        <a
                            class="font-semibold text-blue-500 hover:text-blue-400"
                            href="https://www.rust-lang.org/"
                        >
                            "Rust"
                        </a> ", "
                        <a
                            class="font-semibold text-blue-500 hover:text-blue-400"
                            href="https://www.leptos.dev/"
                        >
                            "Leptos "
                        </a> "& "
                        <a
                            class="font-semibold text-blue-500 hover:text-blue-400"
                            href="https://preline.co/"
                        >
                            "Preline"
                        </a> "."
                    </p>
                    <p class="text-[#CED4DA]">"© Itehax. 2023 "</p>

                    <a
                        class="flex justify-center items-center space-x-2 text-blue-500 hover:text-blue-400 text-base mt-4"
                        href="/feed.xml"
                        rel="external"
                    >
                        <span class="text-[#CED4DA]">"Rss"</span>
                        <svg
                            viewBox="0 0 48 48"
                            class="h-6 w-6"
                            xmlns="http://www.w3.org/2000/svg"
                            fill="currentColor"
                        >
                            <path d="M0 0h48v48H0z" fill="none"></path>
                            <circle cx="12.36" cy="35.64" r="4.36"></circle>
                            <path d="M8 8.89v5.66c14.06 0 25.46 11.4 25.46 25.46h5.66C39.11 22.82 25.18 8.89 8 8.89zM8 20.2v5.66c7.81 0 14.14 6.34 14.14 14.14h5.66c0-10.93-8.87-19.8-19.8-19.8z"></path>
                        </svg>
                    </a>

                </div>
            </div>
        </footer>
    }
}
