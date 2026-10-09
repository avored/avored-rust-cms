use leptos::prelude::*;

#[component]
pub fn WorkflowIndexPage() -> impl IntoView {
    view! {
        <div x-data="workflowIndexPage()" class="min-h-full bg-slate-50 px-4 py-6 sm:px-6 lg:px-8">
            <div class="mx-auto max-w-7xl">
                <h1 class="text-3xl font-semibold tracking-tight text-slate-900">"Workflows"</h1>
                <p class="mt-4 text-sm text-slate-600" x-text="statusMessage">
                    "Workflow management is not implemented yet."
                </p>
            </div>
        </div>
    }
}
