//! Onboarding wizard component for authenticated users.
//!
//! Multi-step wizard allowing users to select their interests:
//! - Step 1: Projects (Global Goals)
//! - Step 2: Campaigns (Personal Interests)
//! - Step 3: Clusters (Platform Features)

use std::collections::HashSet;

use yew::prelude::*;

use crate::api::{JoinItemsRequest, OnboardingItem, OnboardingItems};

/// Current step in the onboarding wizard
#[derive(Debug, Clone, PartialEq)]
enum WizardStep {
    Projects,
    Campaigns,
    Clusters,
}

impl WizardStep {
    fn next(&self) -> Option<WizardStep> {
        match self {
            WizardStep::Projects => Some(WizardStep::Campaigns),
            WizardStep::Campaigns => Some(WizardStep::Clusters),
            WizardStep::Clusters => None,
        }
    }

    fn prev(&self) -> Option<WizardStep> {
        match self {
            WizardStep::Projects => None,
            WizardStep::Campaigns => Some(WizardStep::Projects),
            WizardStep::Clusters => Some(WizardStep::Campaigns),
        }
    }

    fn title(&self) -> &'static str {
        match self {
            WizardStep::Projects => "Global Goals",
            WizardStep::Campaigns => "Personal Interests",
            WizardStep::Clusters => "Platform Features",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            WizardStep::Projects => {
                "Select projects that align with your vision for a better world"
            }
            WizardStep::Campaigns => "Choose topics you care about most",
            WizardStep::Clusters => "Pick the platform features you want to explore",
        }
    }

    fn step_number(&self) -> usize {
        match self {
            WizardStep::Projects => 1,
            WizardStep::Campaigns => 2,
            WizardStep::Clusters => 3,
        }
    }
}

/// Props for the OnboardingWizard component
#[derive(Properties, PartialEq)]
pub struct OnboardingWizardProps {
    /// Onboarding items to display
    pub items: OnboardingItems,
    /// Callback fired when the user completes the wizard
    pub on_complete: Callback<JoinItemsRequest>,
}

/// Multi-step onboarding wizard component
#[function_component(OnboardingWizard)]
pub fn onboarding_wizard(props: &OnboardingWizardProps) -> Html {
    let step = use_state(|| WizardStep::Projects);
    let selected_projects: UseStateHandle<HashSet<String>> = use_state(HashSet::new);
    let selected_campaigns: UseStateHandle<HashSet<String>> = use_state(HashSet::new);
    let selected_clusters: UseStateHandle<HashSet<String>> = use_state(HashSet::new);

    let current_items: Vec<&OnboardingItem> = match *step {
        WizardStep::Projects => props.items.projects.iter().collect(),
        WizardStep::Campaigns => props.items.campaigns.iter().collect(),
        WizardStep::Clusters => props.items.clusters.iter().collect(),
    };

    let on_toggle_item = {
        let step = step.clone();
        let selected_projects = selected_projects.clone();
        let selected_campaigns = selected_campaigns.clone();
        let selected_clusters = selected_clusters.clone();

        Callback::from(move |id: String| {
            let toggle = |set: &UseStateHandle<HashSet<String>>, id: &str| {
                let mut new_set = (**set).clone();
                if new_set.contains(id) {
                    new_set.remove(id);
                } else {
                    new_set.insert(id.to_string());
                }
                set.set(new_set);
            };

            match *step {
                WizardStep::Projects => toggle(&selected_projects, &id),
                WizardStep::Campaigns => toggle(&selected_campaigns, &id),
                WizardStep::Clusters => toggle(&selected_clusters, &id),
            }
        })
    };

    let on_next = {
        let step = step.clone();
        let selected_projects = selected_projects.clone();
        let selected_campaigns = selected_campaigns.clone();
        let selected_clusters = selected_clusters.clone();
        let on_complete = props.on_complete.clone();

        Callback::from(move |_: MouseEvent| {
            if let Some(next_step) = step.next() {
                step.set(next_step);
            } else {
                // Final step - submit
                let request = JoinItemsRequest {
                    project_ids: selected_projects.iter().cloned().collect(),
                    campaign_ids: selected_campaigns.iter().cloned().collect(),
                    cluster_ids: selected_clusters.iter().cloned().collect(),
                };
                on_complete.emit(request);
            }
        })
    };

    let on_prev = {
        let step = step.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(prev_step) = step.prev() {
                step.set(prev_step);
            }
        })
    };

    let current_selected = match *step {
        WizardStep::Projects => &*selected_projects as &HashSet<String>,
        WizardStep::Campaigns => &*selected_campaigns as &HashSet<String>,
        WizardStep::Clusters => &*selected_clusters as &HashSet<String>,
    };

    let is_last_step = step.next().is_none();
    let is_first_step = step.prev().is_none();

    html! {
        <div class="onboarding-wizard">
            // Progress indicator
            <div class="wizard-progress">
                { for [WizardStep::Projects, WizardStep::Campaigns, WizardStep::Clusters].iter().map(|s| {
                    let is_active = s.step_number() == step.step_number();
                    let is_done = s.step_number() < step.step_number();
                    html! {
                        <div
                            class={classes!(
                                "wizard-step-dot",
                                is_active.then_some("wizard-step-dot--active"),
                                is_done.then_some("wizard-step-dot--done"),
                            )}
                            key={s.step_number()}
                        >
                            if is_done {
                                { "✓" }
                            } else {
                                { s.step_number().to_string() }
                            }
                        </div>
                    }
                })}
            </div>

            <div class="wizard-header">
                <h2 class="wizard-title">
                    { format!("Step {}/3: {}", step.step_number(), step.title()) }
                </h2>
                <p class="wizard-description">{ step.description() }</p>
            </div>

            // Item selection grid
            <div class="wizard-items">
                { for current_items.iter().map(|item| {
                    let is_selected = current_selected.contains(&item.id);
                    let item_id = item.id.clone();
                    let on_toggle = on_toggle_item.clone();

                    html! {
                        <button
                            class={classes!(
                                "item-card",
                                is_selected.then_some("item-card--selected")
                            )}
                            onclick={Callback::from(move |_: MouseEvent| {
                                on_toggle.emit(item_id.clone());
                            })}
                            key={item.id.clone()}
                        >
                            <div class="item-card-check">
                                if is_selected { { "✓" } }
                            </div>
                            <h3 class="item-card-title">{ &item.title }</h3>
                            <p class="item-card-desc">{ &item.description }</p>
                        </button>
                    }
                })}
            </div>

            // Navigation buttons
            <div class="wizard-actions">
                if !is_first_step {
                    <button class="btn btn-outline" onclick={on_prev}>
                        { "← Back" }
                    </button>
                }
                <button class="btn btn-primary" onclick={on_next}>
                    if is_last_step {
                        { "Complete Setup →" }
                    } else {
                        { "Next →" }
                    }
                </button>
            </div>
        </div>
    }
}
