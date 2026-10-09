use leptos::prelude::*;

#[component]
pub fn WorkflowEditPage() -> impl IntoView {
    view! {
        <div x-data="workflowFormPage()" class="min-h-full bg-slate-50 px-4 py-6 sm:px-6 lg:px-8">
            <div class="mx-auto max-w-3xl">
                <h1 class="text-3xl font-semibold tracking-tight text-slate-900">"Edit workflow"</h1>
                <p class="mt-4 text-sm text-slate-600" x-text="statusMessage">
                    "Workflow editing is not implemented yet."
                </p>
            </div>
        </div>
    }
}
