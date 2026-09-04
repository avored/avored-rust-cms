---
name: plan-task
description: >-
  Analyzes an incoming feature description or issue across an Axum backend and a Leptos/Alpine frontend.
  Generates an incremental skeleton-first execution plan using todo!() primitives. Activate this skill
  when the user asks to plan, map out, or build full-stack features involving Axum endpoints, Leptos
  view components, Alpine.js client manipulations, or Tailwind UI layers.
---

# Skill: Plan and Implement Full-Stack Rust Tasks

## Description
Analyzes an incoming feature description or issue across an Axum backend and a Leptos/Alpine frontend. Generates an incremental skeleton-first execution plan using `todo!()` primitives.

## Triggers & Intent
Activate this skill when a user asks to plan, map out, or build full-stack features involving Axum endpoints, Leptos view components, Alpine.js client manipulations, or Tailwind UI layers.

---

## Execution Guardrails

> ⚠️ **CRITICAL RULES FOR GEMINI EDITOR:**
> 1. **No Blind Implementation:** You must present the implementation plan and pause for user validation. Do not generate code until the design layout is signed off.
> 2. **Rust Compile Integrity:** When writing code files later, you must always scaffold empty structures with native `todo!()` macros or comments first. Do not break compiler definitions mid-flight.

---

## Phase 1: Context Grounding & Dependency Discovery
1. **Analyze Requirements:** Ingest API request payloads, query contracts, frontend state expectations, and Tailwind UI specs.
2. **Scan Codebase:** Audit your Rust workspace structures (e.g., `Cargo.toml` dependencies, router bindings, Leptos server functions, styling entrypoints).
3. **Map Dependencies:** Check existing shared types, data wrappers, and state extractors to keep configurations homogenous.

## Phase 2: Technical Plan Generation
Using the layout rules from [technical plan template](./templates/tech-plan-template.md), generate a scannable implementation brief featuring these exact steps:

1. **Scaffolding Sequence:** Define where Rust `todo!()` structures will be set up across Axum handlers and Leptos views alongside Alpine binding declarations.
2. **Database & Infrastructure Layer:** Map updates to the data access tier.
3. **Axum Backend Layer:** Detail the removal of `todo!()` macros inside backend handling loops.
4. **Leptos & Alpine Frontend Layer:** Finalise client-side components and styling logic using Tailwind utilities.

## Phase 3: The Validation Halt
Stop execution entirely at the end of Phase 2. Present the plan clearly and output this exact closing call-to-action string to hand control back to the developer:

*"Please review the layered technical plan above. If this accurately maps out the implementation sequence, reply with **'Approved'** to unblock file editing."*
