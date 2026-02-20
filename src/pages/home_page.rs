use leptos::prelude::*;



/*
#[server]
pub async fn setup_grpc() -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::infra::grpc::misc::{SetupRequest, misc_client::MiscClient};

        let mut client: MiscClient<tonic::transport::Channel> =
            MiscClient::connect("http://127.0.0.1:3000")
                .await
                .map_err(|e| ServerFnError::new(format!("Connection failed: {}", e)))?;

        let request = tonic::Request::new(SetupRequest {  });

        let response = client
            .setup(request)
            .await
            .map_err(|e| ServerFnError::new(format!("gRPC call failed: {}", e)))?;

        Ok(response.into_inner().status)
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = name;
        Err(ServerFnError::new(
            "Server function body should not be called on client",
        ))
    }
}


#[server]
pub async fn health_check_grpc() -> Result<bool, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use crate::infra::grpc::misc::misc_client::MiscClient;
        use crate::infra::grpc::misc::HealthCheckRequest;

        let mut client: MiscClient<tonic::transport::Channel> =
            MiscClient::connect("http://127.0.0.1:3000")
                .await
                .map_err(|e| ServerFnError::new(format!("Connection failed: {}", e)))?;

        let request = tonic::Request::new(HealthCheckRequest {  });

        let response = client
            .health_check(request)
            .await
            .map_err(|e| ServerFnError::new(format!("gRPC call failed: {}", e)))?;

        Ok(response.into_inner().status)
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = name;
        Err(ServerFnError::new(
            "Server function body should not be called on client",
        ))
    }
}



#[server]
pub async fn greet_grpc(name: String) -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        // use crate::infra::grpc::helloworld::greeter_client::GreeterClient;
        // use crate::infra::grpc::helloworld::HelloRequest;

        // let mut client: GreeterClient<tonic::transport::Channel> =
        //     GreeterClient::connect("http://127.0.0.1:3000")
        //         .await
        //         .map_err(|e| ServerFnError::new(format!("Connection failed: {}", e)))?;

        // let request = tonic::Request::new(HelloRequest { name });

        // let response = client
        //     .say_hello(request)
        //     .await
        //     .map_err(|e| ServerFnError::new(format!("gRPC call failed: {}", e)))?;

        // Ok(response.into_inner().message)
    }
    #[cfg(not(feature = "ssr"))]
    {
        // let _ = name;
        Err(ServerFnError::new(
            "Server function body should not be called on client",
        ))
    }
}

*/

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="relative sm:flex sm:justify-center sm:items-center min-h-screen bg-center bg-gray-100 selection:bg-primary-500 selection:text-white">
                <div class="max-w-7xl mx-auto p-6 lg:p-8">
                    <div class="flex justify-center">
                        <img src="/public/images/avored.svg" class="w-20 h-20" alt="Avored Rust Cms"/>
                    </div>
                    <div class="mt-16">
                        <div class="flex justify-center">
                            <div class="text-gray-600">
                                "To Access demo admin please use the email:" 
                                <span class="text-red-500 font-semibold">
                                    "demo@avored.com"
                                </span>
                                "and password:" 
                                <span class="text-red-500 font-semibold">
                                    "admin123"
                                </span>
                            </div>
                        </div>
                    </div>
                    <div class="mt-16">
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 lg:gap-8">
                            <a
                                href="/login"
                                class="scale-100 p-6 bg-white rounded-lg shadow-2xl shadow-gray-500/20 flex focus:outline-primary-500"
                            >
                            <div>
                                    <div
                                        class="h-16 w-16 bg-primary-50 flex items-center justify-center rounded-full">
                                        <i
                                            class="w-7 h-7 stroke-primary-500"
                                            data-feather="framer"
                                        ></i>
                                    </div>

                                    <h2 class="mt-6 text-xl font-semibold text-gray-900">
                                        "Administrator"
                                    </h2>

                                    <p class="mt-4 text-gray-500 text-sm leading-relaxed">
                                        "An app administrator plays a pivotal role in ensuring the
                                        smooth operation of the avored cms. They are responsible for
                                        managing user access, content updates and overseeing
                                        updates. The administrator acts as the gatekeeper,
                                        safeguarding the app's content and user experience while
                                        facilitating its growth and improvement. In essence, they
                                        are the backbone of a well-maintained and secure avored rust
                                        cms ecosystem. AvoRed"
                                    </p>
                                </div>
                                <div class="self-center pl-5">
                                    <i
                                        class="w-6 h-6 stroke-primary-500"
                                        data-feather="chevrons-right"
                                    ></i>
                                </div>
                            </a>
                            <a
                                href="https://github.com/avored/avored-rust-cms"
                                class="scale-100 p-6 bg-white  rounded-lg shadow-2xl flex focus:outline-primary-500"
                            >
                                <div>
                                    <div
                                        class="h-16 w-16 bg-primary-50 flex items-center justify-center rounded-full">
                                        <i
                                            class="w-7 h-7 stroke-primary-500"
                                            data-feather="github"
                                        ></i>
                                    </div>

                                    <h2 class="mt-6 text-xl font-semibold text-gray-900">
                                        "Github"
                                    </h2>

                                    <p class="mt-4 text-gray-500 text-sm leading-relaxed">
                                        "GitHub Star: When you \"star\" a GitHub repository, it
                                        signifies your interest in that project. Starring a
                                        repository also sends a signal to the repository owner that
                                        you appreciate their work or find it valuable. Community
                                        Help: The GitHub community is known for its collaborative
                                        nature. Developers frequently help each other by
                                        contributing to open-source projects, reporting issues,
                                        suggesting improvements, and even submitting pull requests
                                        to fix bugs or add features. This collective effort is
                                        essential for the growth and success of open-source projects
                                        hosted on GitHub."
                                    </p>
                                </div>

                                <div class="self-center pl-5">
                                    <i
                                        class="w-6 h-6 stroke-primary-500"
                                        data-feather="chevrons-right"
                                    ></i>
                                </div>
                            </a>
                        </div>
                    </div>
                    <div class="flex justify-center mt-16 px-0 sm:items-center sm:justify-between">
                        <div class="text-center text-sm text-gray-500  sm:text-left">
                            <div class="flex items-center gap-4">
                                <a
                                    href="https://github.com/sponsors/indpurvesh"
                                    class="group inline-flex items-center hover:text-gray-700 dark:hover:text-white focus:rounded-sm focus:outline-primary-500"
                                >
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        strokeWidth="1.5"
                                        class="-mt-px mr-1 w-5 h-5 stroke-gray-400 dark:stroke-gray-600 group-hover:stroke-gray-600 dark:group-hover:stroke-gray-400"
                                    >
                                        <path
                                            strokeLinecap="round"
                                            strokeLinejoin="round"
                                            d="M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12z"
                                        />
                                    </svg>
                                    "Sponsor"
                                </a>
                            </div>
                        </div>

                        <div class="ml-4 text-center text-sm text-gray-500 sm:text-right sm:ml-0">
                            "AvoRed Rust CMS"
                        </div>
                    </div>
                </div>
            </div>
    }





    // Creates a reactive value to update the button
    // let count = RwSignal::new(0);
    // let on_click = move |_| *count.write() += 1;

    // let (name, set_name) = signal("Leptos".to_string());


    // let greet_action = ServerAction::<GreetGrpc>::new();
    // let greet_result = greet_action.value();

    // let misc_action = ServerAction::<HealthCheckGrpc>::new();
    // let misc_result = misc_action.value();

    // let setup_action = ServerAction::<SetupGrpc>::new();
    // let setup_result = setup_action.value();

    // view! {
    //     <h1>"Welcome to Leptos!"</h1>
    //     <button on:click=on_click>"Click Me: " {count}</button>

    //     <hr/>

    //     <h2>"gRPC Integration"</h2>
    //     <input
    //         type="text"
    //         on:input=move |ev| set_name.set(event_target_value(&ev))
    //         prop:value=name
    //     />
    //     <ActionForm action=greet_action>
    //         <input type="hidden" name="name" value=move || name.get() />

    //         <button type="submit">
    //             "Call gRPC Greeter"
    //         </button>

    //     </ActionForm>

    //     <button
    //         type="button"
    //         on:click=move |_| {
    //             greet_action.dispatch(GreetGrpc { name: name.get() });
    //         }
    //     >
    //         "Another Button"
    //     </button>

    //     <p>
    //         {move || match greet_result.get() {
    //             None => "".to_string(),
    //             Some(Ok(msg)) => msg,
    //             Some(Err(err)) => format!("Error: {}", err),
    //         }}
    //     </p>

    //     <div>
    //         "New Health check Request"
    //         <ActionForm action=misc_action>

    //         <button type="submit">
    //             "Call gRPC Misc"
    //         </button>

    //     </ActionForm>
    //     </div>
    //     <p>
    //         {move || match misc_result.get() {
    //             None => "".to_string(),
    //             Some(Ok(msg)) => {
    //                 if msg {
    //                     "Health Check Passed".to_string()
    //                 } else {
    //                     "Health Check Failed".to_string()
    //                 }
    //             },
    //             Some(Err(err)) => format!("Error: {}", err),
    //         }}
    //     </p>

    //     <div>
    //         "New Setup Request"
    //         <ActionForm action=setup_action>

    //         <button type="submit">
    //             "Call gRPC Setup"
    //         </button>

    //     </ActionForm>
    //     </div>
    //     <p>
    //         {move || match setup_result.get() {
    //             None => "".to_string(),
    //             Some(Ok(msg)) => {
    //                 if msg {
    //                     "Setup Passed".to_string()
    //                 } else {
    //                     "Setup Failed".to_string()
    //                 }
    //             },
    //             Some(Err(err)) => format!("Error: {}", err),
    //         }}
    //     </p>
    // }
}
