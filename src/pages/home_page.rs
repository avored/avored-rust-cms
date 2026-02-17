use leptos::prelude::*;


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
        use crate::infra::grpc::helloworld::greeter_client::GreeterClient;
        use crate::infra::grpc::helloworld::HelloRequest;

        let mut client: GreeterClient<tonic::transport::Channel> =
            GreeterClient::connect("http://127.0.0.1:3000")
                .await
                .map_err(|e| ServerFnError::new(format!("Connection failed: {}", e)))?;

        let request = tonic::Request::new(HelloRequest { name });

        let response = client
            .say_hello(request)
            .await
            .map_err(|e| ServerFnError::new(format!("gRPC call failed: {}", e)))?;

        Ok(response.into_inner().message)
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = name;
        Err(ServerFnError::new(
            "Server function body should not be called on client",
        ))
    }
}

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    let (name, set_name) = signal("Leptos".to_string());


    let greet_action = ServerAction::<GreetGrpc>::new();
    let greet_result = greet_action.value();

    let misc_action = ServerAction::<HealthCheckGrpc>::new();
    let misc_result = misc_action.value();

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>

        <hr/>

        <h2>"gRPC Integration"</h2>
        <input
            type="text"
            on:input=move |ev| set_name.set(event_target_value(&ev))
            prop:value=name
        />
        <ActionForm action=greet_action>
            <input type="hidden" name="name" value=move || name.get() />

            <button type="submit">
                "Call gRPC Greeter"
            </button>

        </ActionForm>

        <button
            type="button"
            on:click=move |_| {
                greet_action.dispatch(GreetGrpc { name: name.get() });
            }
        >
            "Another Button"
        </button>

        <p>
            {move || match greet_result.get() {
                None => "".to_string(),
                Some(Ok(msg)) => msg,
                Some(Err(err)) => format!("Error: {}", err),
            }}
        </p>

        <div>
            "New Health check Request"
            <ActionForm action=misc_action>

            <button type="submit">
                "Call gRPC Misc"
            </button>

        </ActionForm>
        </div>
        <p>
            {move || match misc_result.get() {
                None => "".to_string(),
                Some(Ok(msg)) => {
                    if msg {
                        "Health Check Passed".to_string()
                    } else {
                        "Health Check Failed".to_string()
                    }
                },
                Some(Err(err)) => format!("Error: {}", err),
            }}
        </p>

        
    }
}
