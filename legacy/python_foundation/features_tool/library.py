from __future__ import annotations

from dataclasses import dataclass
import json
from pathlib import Path
import re
from typing import Any


LIBRARY_SCHEMA_VERSION = "feature_library_item.v1"
DEFAULT_LIBRARY_ROOT = Path("library")
SCHEMA_RELATIVE_PATH = Path("schemas") / "library_item_schema_v1.json"
ITEMS_RELATIVE_ROOT = Path("items")
STATUS_VALUES = ("template", "draft", "tested", "stable", "experimental", "deprecated")
LEVEL_VALUES = ("low", "medium", "high")
ITEM_TYPES = (
    "feature_card",
    "ui_pattern",
    "logic_pattern",
    "component_contract",
    "app_archetype",
    "build_recipe",
    "spec_sheet",
    "dex_prompt",
    "review_packet_template",
    "decision_log",
    "teardown",
    "source_reference_rules",
    "app_idea_intake",
    "folder_skeleton",
)
ITEM_TYPE_DIRECTORIES = {
    "feature_card": "feature_cards",
    "ui_pattern": "ui_patterns",
    "logic_pattern": "logic_patterns",
    "component_contract": "component_contracts",
    "app_archetype": "app_archetypes",
    "build_recipe": "build_recipes",
    "spec_sheet": "spec_sheets",
    "dex_prompt": "dex_prompts",
    "review_packet_template": "review_packet_templates",
    "decision_log": "decision_logs",
    "teardown": "teardowns",
    "source_reference_rules": "source_reference_rules",
    "app_idea_intake": "app_idea_intakes",
    "folder_skeleton": "folder_skeletons",
}
SLUG_RE = re.compile(r"^[a-z0-9]+(?:_[a-z0-9]+)*$")
COMMON_REQUIRED_FIELDS = (
    "schema_version",
    "name",
    "slug",
    "type",
    "template",
    "status",
    "summary",
    "problem_solved",
    "when_to_use",
    "when_not_to_use",
    "required_inputs",
    "expected_outputs",
    "dependencies",
    "compatible_features",
    "implementation_notes",
    "known_failure_modes",
    "example_projects",
    "tags",
    "ui_surfaces",
    "logic_patterns",
    "app_archetypes",
    "languages",
    "frameworks",
    "complexity",
    "reusability",
    "risk",
)
STRING_LIST_FIELDS = (
    "when_to_use",
    "when_not_to_use",
    "dependencies",
    "compatible_features",
    "implementation_notes",
    "known_failure_modes",
    "example_projects",
    "tags",
    "ui_surfaces",
    "logic_patterns",
    "app_archetypes",
    "languages",
    "frameworks",
)
TYPE_PAYLOAD_KEYS = {
    "ui_pattern": "ui_pattern",
    "logic_pattern": "logic_pattern",
    "component_contract": "component_contract",
    "app_archetype": "app_archetype",
    "build_recipe": "build_recipe",
    "spec_sheet": "spec_sheet",
    "dex_prompt": "dex_prompt",
    "review_packet_template": "review_packet",
    "decision_log": "decision_log",
    "teardown": "teardown",
    "source_reference_rules": "source_reference_rules",
    "app_idea_intake": "app_idea_intake",
    "folder_skeleton": "folder_skeleton",
}
TYPE_LABELS = {
    "feature_card": "feature card",
    "ui_pattern": "UI pattern",
    "logic_pattern": "logic pattern",
    "component_contract": "component contract",
    "app_archetype": "app archetype",
    "build_recipe": "build recipe",
    "spec_sheet": "spec sheet",
    "dex_prompt": "Dex prompt template",
    "review_packet_template": "review packet template",
    "decision_log": "decision log template",
    "teardown": "teardown template",
    "source_reference_rules": "source reference rules template",
    "app_idea_intake": "app idea intake template",
    "folder_skeleton": "folder skeleton",
}


@dataclass(frozen=True)
class LibraryEntry:
    path: Path
    item: dict[str, Any] | None
    errors: tuple[str, ...]

    @property
    def valid(self) -> bool:
        return not self.errors


def slugify(value: str) -> str:
    slug = re.sub(r"[^a-z0-9]+", "_", value.strip().lower())
    slug = re.sub(r"_+", "_", slug).strip("_")
    if not slug:
        raise ValueError("Unable to derive a slug from the provided name")
    return slug


def relative_item_path(item_type: str, slug: str) -> Path:
    try:
        directory = ITEM_TYPE_DIRECTORIES[item_type]
    except KeyError as exc:
        raise ValueError(f"Unsupported library item type: {item_type}") from exc
    return ITEMS_RELATIVE_ROOT / directory / f"{slug}.json"


def build_library_item(
    item_type: str,
    name: str,
    *,
    slug: str | None = None,
    template: bool = False,
    status: str | None = None,
    summary: str | None = None,
    tags: list[str] | None = None,
    ui_surfaces: list[str] | None = None,
    logic_patterns: list[str] | None = None,
    app_archetypes: list[str] | None = None,
    example_projects: list[str] | None = None,
    dependencies: list[str] | None = None,
    languages: list[str] | None = None,
    frameworks: list[str] | None = None,
    complexity: str = "medium",
    reusability: str = "high",
    risk: str = "medium",
) -> dict[str, Any]:
    if item_type not in ITEM_TYPES:
        raise ValueError(f"Unsupported library item type: {item_type}")
    for label, value in (("complexity", complexity), ("reusability", reusability), ("risk", risk)):
        if value not in LEVEL_VALUES:
            raise ValueError(f"Unsupported {label}: {value}")

    slug_value = slug or slugify(name)
    item_status = status or ("template" if template else "draft")
    if item_status not in STATUS_VALUES:
        raise ValueError(f"Unsupported status: {item_status}")
    if template and item_status != "template":
        raise ValueError("Template items must use status 'template'")
    if not template and item_status == "template":
        raise ValueError("Non-template items cannot use status 'template'")

    item = {
        "schema_version": LIBRARY_SCHEMA_VERSION,
        "name": name,
        "slug": slug_value,
        "type": item_type,
        "template": template,
        "status": item_status,
        "summary": summary or default_summary(item_type, name),
        "problem_solved": default_problem_solved(item_type, name),
        "when_to_use": default_when_to_use(item_type, name),
        "when_not_to_use": default_when_not_to_use(item_type, name),
        "required_inputs": default_required_inputs(item_type, name),
        "expected_outputs": default_expected_outputs(item_type, name),
        "dependencies": unique_strings(dependencies or []),
        "compatible_features": [],
        "implementation_notes": default_implementation_notes(item_type, name),
        "known_failure_modes": default_failure_modes(item_type, name),
        "example_projects": unique_strings(example_projects or []),
        "tags": unique_strings(default_tags(item_type, template) + (tags or [])),
        "ui_surfaces": unique_strings(ui_surfaces or []),
        "logic_patterns": unique_strings(logic_patterns or []),
        "app_archetypes": unique_strings(app_archetypes or []),
        "languages": unique_strings(languages or []),
        "frameworks": unique_strings(frameworks or []),
        "complexity": complexity,
        "reusability": reusability,
        "risk": risk,
    }
    add_type_specific_fields(item)
    return item


def default_summary(item_type: str, name: str) -> str:
    return f"Reusable {TYPE_LABELS[item_type]} for {name}."


def default_problem_solved(item_type: str, name: str) -> str:
    problems = {
        "feature_card": f"Describe the reusable product problem that {name} solves.",
        "ui_pattern": f"Explain the interface problem that {name} solves and why this surface exists.",
        "logic_pattern": f"Explain the repeatable plumbing or control-flow problem that {name} solves.",
        "component_contract": f"Turns {name} into a buildable contract instead of a vague component request.",
        "app_archetype": f"Defines the recurring product shape that {name} represents.",
        "build_recipe": f"Bundles reusable library pieces so {name} can assemble a coherent app starter.",
        "spec_sheet": f"Locks the sections and decisions needed to turn {name} into a buildable spec.",
        "dex_prompt": f"Defines bounded prompt structure so {name} can drive narrow Dex or Codex tasks.",
        "review_packet_template": f"Standardizes how work built around {name} is reviewed and handed back.",
        "decision_log": f"Prevents architectural archaeology when {name} changes over time.",
        "teardown": f"Captures reusable decisions from {name} without cloning branding or assets.",
        "source_reference_rules": f"Defines what {name} may reference, adapt, or forbid when mining external sources.",
        "app_idea_intake": f"Turns vague ideas about {name} into structured build inputs.",
        "folder_skeleton": f"Prevents repo sprawl when building {name} by locking structure up front.",
    }
    return problems[item_type]


def default_when_to_use(item_type: str, name: str) -> list[str]:
    uses = {
        "feature_card": [
            f"Use when {name} is a reusable feature that should survive beyond one repo.",
            "Use when Dex needs explicit problem boundaries, dependencies, and expected outputs.",
        ],
        "ui_pattern": [
            f"Use when {name} is a recurring surface or layout decision across tools.",
            "Use when Dex should reuse layout law instead of re-inventing navigation from scratch.",
        ],
        "logic_pattern": [
            f"Use when {name} is repeatable plumbing that appears across projects.",
            "Use when Dex needs stable logic seams for validators, queues, state machines, or indexing.",
        ],
        "component_contract": [
            f"Use when {name} must be built from a concrete contract rather than a vague component request.",
            "Use when props, state, events, keyboard rules, and tests need to be locked before coding.",
        ],
        "app_archetype": [
            f"Use when {name} is a common app shape that should replace blank-page project starts.",
            "Use when Dex should begin from a known dashboard, creative tool, browser, or workstation structure.",
        ],
        "build_recipe": [
            f"Use when {name} should bundle multiple patterns into a reusable starter for an app family.",
            "Use when Dex should auto-select compatible pieces instead of hand-assembling them each time.",
        ],
        "spec_sheet": [
            f"Use when {name} must turn an idea into a bounded product/build contract before implementation.",
            "Use when Dex needs a stable spec shape that can later feed prompt generation.",
        ],
        "dex_prompt": [
            f"Use when {name} should produce bounded implementation prompts instead of monster briefs.",
            "Use when Dex needs explicit scope, allowed files, acceptance criteria, and forbidden work.",
        ],
        "review_packet_template": [
            f"Use when work touching {name} should always return with the same audit fields.",
            "Use when completed Dex tasks need comparable evidence across projects.",
        ],
        "decision_log": [
            f"Use when {name} introduces a choice that future agents might otherwise unknowingly reverse.",
            "Use when the reason, rejected alternatives, and reversal condition need to remain queryable.",
        ],
        "teardown": [
            f"Use when {name} should be dissected into reusable workflow and layout decisions.",
            "Use when Dex needs reference mining without cargo-culting whole products.",
        ],
        "source_reference_rules": [
            f"Use when {name} needs explicit rules for what reference material may be adapted or only studied.",
            "Use when Dex should avoid copying protected branding, assets, or code.",
        ],
        "app_idea_intake": [
            f"Use when {name} starts as a vague app idea that needs structure before spec work.",
            "Use when Dex should capture surfaces, workflows, data, and constraints up front.",
        ],
        "folder_skeleton": [
            f"Use when {name} is a good starting repo shape for a new feature-library or app project.",
            "Use when structure should be created before code generation begins.",
        ],
    }
    return uses[item_type]


def default_when_not_to_use(item_type: str, name: str) -> list[str]:
    non_uses = {
        "feature_card": [
            f"Do not use when {name} is one-off glue with no reuse value.",
            "Do not use when the problem or outputs are still too vague to capture structurally.",
        ],
        "ui_pattern": [
            f"Do not use when {name} is only cosmetic and does not affect navigation, density, or workflow.",
            "Do not use when device constraints make the pattern actively hostile.",
        ],
        "logic_pattern": [
            f"Do not use when {name} hides product-specific rules that belong in a dedicated feature module.",
            "Do not use when a linear local function is enough and no reuse seam exists.",
        ],
        "component_contract": [
            f"Do not use when {name} is too vague to state props, state, and emitted events.",
            "Do not use when the true contract belongs at a larger feature boundary instead.",
        ],
        "app_archetype": [
            f"Do not use when {name} is too specific to one project to act as a recurring app shape.",
            "Do not use when the product is still too undefined to choose a stable shell.",
        ],
        "build_recipe": [
            f"Do not use when {name} combines incompatible patterns just to look comprehensive.",
            "Do not use when the recipe cannot yet state its guardrails and expected outputs.",
        ],
        "spec_sheet": [
            f"Do not use when {name} is still pure brainstorming and no build boundary exists.",
            "Do not use when the spec would duplicate an existing locked contract without adding structure.",
        ],
        "dex_prompt": [
            f"Do not use when {name} would still require heroic interpretation from Dex.",
            "Do not use when the prompt cannot yet state acceptance criteria or forbidden work.",
        ],
        "review_packet_template": [
            f"Do not use when {name} work is still exploratory and no deliverable boundary exists.",
            "Do not use when the report would repeat richer domain-specific audit artifacts.",
        ],
        "decision_log": [
            f"Do not use when {name} is trivial and no meaningful alternative or reversal condition exists.",
            "Do not use when a locked project-level decision log already captures the same choice.",
        ],
        "teardown": [
            f"Do not use when {name} is just a screenshot dump with no reusable decisions extracted.",
            "Do not use when source-reference rules are unclear and copying risk is unresolved.",
        ],
        "source_reference_rules": [
            f"Do not use when {name} tries to legalize direct copying of brand assets or proprietary code.",
            "Do not use when the rule set is too vague for Dex to act on deterministically.",
        ],
        "app_idea_intake": [
            f"Do not use when {name} is already a locked spec and no intake ambiguity remains.",
            "Do not use when the intake would omit key constraints like data, surfaces, or users.",
        ],
        "folder_skeleton": [
            f"Do not use when {name} needs a domain-specific layout that would instantly violate the skeleton.",
            "Do not use when reorganizing a mature repo with already-locked structure.",
        ],
    }
    return non_uses[item_type]


def default_required_inputs(item_type: str, name: str) -> list[dict[str, Any]]:
    inputs = {
        "feature_card": [
            field_spec("feature_name", "string", "Stable reusable feature name."),
            field_spec("problem_boundary", "string", "Concise statement of the problem the feature solves."),
        ],
        "ui_pattern": [
            field_spec("surface_name", "string", "Stable pattern name used in specs and contracts."),
            field_spec("data_dependencies", "object_list", "Required data the surface needs to render useful state."),
        ],
        "logic_pattern": [
            field_spec("trigger_points", "string_list", "Where the pattern enters the system."),
            field_spec("invariants", "string_list", "Rules the implementation must preserve."),
        ],
        "component_contract": [
            field_spec("component_name", "string", "Stable component identifier."),
            field_spec("state_dependencies", "object_list", "State and signals required to render and act."),
        ],
        "app_archetype": [
            field_spec("app_shape_name", "string", "Canonical name for the archetype."),
            field_spec("core_workflows", "string_list", "Primary workflows the archetype must support."),
        ],
        "build_recipe": [
            field_spec("recipe_name", "string", "Stable recipe name."),
            field_spec("selected_items", "string_list", "Library items to compose into the recipe."),
        ],
        "spec_sheet": [
            field_spec("idea_or_request", "string", "Initial idea or request being transformed into a build spec."),
            field_spec("selected_library_items", "string_list", "Items chosen to shape the spec."),
        ],
        "dex_prompt": [
            field_spec("task_kind", "string", "Prompt type such as skeleton, component, logic, test, or review."),
            field_spec("acceptance_criteria", "string_list", "Non-negotiable outcomes the prompt must enforce."),
        ],
        "review_packet_template": [
            field_spec("task_goal", "string", "Clear statement of what the completed task aimed to achieve."),
            field_spec("evidence_bundle", "object_list", "Commands, tests, screenshots, or notes captured during execution."),
        ],
        "decision_log": [
            field_spec("decision", "string", "The choice being locked."),
            field_spec("alternatives_rejected", "string_list", "Meaningful rejected alternatives."),
        ],
        "teardown": [
            field_spec("source_product", "string", "Product or site being studied."),
            field_spec("capture_scope", "string_list", "Which reusable decisions the teardown must extract."),
        ],
        "source_reference_rules": [
            field_spec("source_kind", "string", "Type of reference source being governed."),
            field_spec("copy_boundaries", "string_list", "Explicit boundaries for what may not be copied."),
        ],
        "app_idea_intake": [
            field_spec("idea_name", "string", "Working name of the app idea."),
            field_spec("constraints", "string_list", "Known constraints, users, and workflow requirements."),
        ],
        "folder_skeleton": [
            field_spec("repo_shape", "string", "Name of the repo shape the skeleton targets."),
            field_spec("required_surfaces", "string_list", "Core surfaces or artifacts the structure must accommodate."),
        ],
    }
    return inputs[item_type]


def default_expected_outputs(item_type: str, name: str) -> list[dict[str, Any]]:
    labels = {
        "feature_card": "feature_card",
        "ui_pattern": "ui_pattern_card",
        "logic_pattern": "logic_pattern_card",
        "component_contract": "component_contract",
        "app_archetype": "app_archetype",
        "build_recipe": "build_recipe",
        "spec_sheet": "spec_sheet",
        "dex_prompt": "dex_prompt",
        "review_packet_template": "review_packet_template",
        "decision_log": "decision_log",
        "teardown": "teardown",
        "source_reference_rules": "source_reference_rules",
        "app_idea_intake": "app_idea_intake",
        "folder_skeleton": "folder_skeleton",
    }
    return [output_spec(labels[item_type], "library_item", f"Validated {TYPE_LABELS[item_type]} for {name}.")]


def default_implementation_notes(item_type: str, name: str) -> list[str]:
    notes = {
        "feature_card": [
            "Keep the card machine-first and bounded; do not hide key decisions in prose-only notes.",
            f"Treat {name} as reusable only if its inputs, outputs, and failure modes are explicit.",
        ],
        "ui_pattern": [
            "Record the data needed to make the pattern useful, not just visually complete.",
            "Name surfaces consistently across specs, prompts, and component contracts.",
        ],
        "logic_pattern": [
            "Capture invariants and failure signals before implementation details drift across repos.",
            "Prefer deterministic state and error boundaries over ambient side effects.",
        ],
        "component_contract": [
            "Keep the contract specific enough that Dex can implement without inventing behavior.",
            "State empty, loading, error, keyboard, and accessibility rules explicitly.",
        ],
        "app_archetype": [
            "Keep the archetype focused on recurring shell shape and workflows, not one project's branding.",
            "Use the archetype to narrow choice, not to force decorative sameness.",
        ],
        "build_recipe": [
            "List the exact pieces being bundled so assembly can be automated later.",
            "Document guardrails so recipes do not produce Franken-apps with incompatible seams.",
        ],
        "spec_sheet": [
            "Lock the first vertical slice and out-of-scope list so prompts stay bounded.",
            "Make sections stable enough that generators can fill them without rethinking the structure.",
        ],
        "dex_prompt": [
            "Prompt templates should define exact scope, acceptance, and forbidden work rather than motivational prose.",
            "Keep prompts short enough that Dex can act, not reinterpret a novel.",
        ],
        "review_packet_template": [
            "Keep evidence fields stable across tasks so review packets can be compared automatically later.",
            "Do not let screenshots replace commands, tests, or stated risks.",
        ],
        "decision_log": [
            "Record the reason and reversal condition in the same artifact so future agents can audit the choice.",
            "Rejected alternatives matter; otherwise the log becomes decorative metadata.",
        ],
        "teardown": [
            "Extract reusable layout, state, and workflow decisions instead of copying a product's surface style wholesale.",
            "Always pair teardown notes with source-reference rules to reduce copying drift.",
        ],
        "source_reference_rules": [
            "Keep the rules operational so Dex can tell what is allowed without human interpretation.",
            "Explicitly forbid copying branding, proprietary copy, or protected assets.",
        ],
        "app_idea_intake": [
            "Capture workflows, surfaces, data, and constraints before turning the intake into a spec.",
            "Intake should reduce ambiguity, not preserve it.",
        ],
        "folder_skeleton": [
            "Treat this as the canonical repo shape, not a hand-wavy suggestion.",
            "Include docs and tests from the start so future generators do not bolt them on later.",
        ],
    }
    return notes[item_type]


def default_failure_modes(item_type: str, name: str) -> list[str]:
    failures = {
        "feature_card": [
            f"{name} becomes a junk-drawer card because the problem boundary is still vague.",
            "Inputs or outputs are missing, so Dex has to invent behavior during implementation.",
        ],
        "ui_pattern": [
            f"{name} describes appearance but not navigation, data, or collapse behavior.",
            "The pattern is reused in a context where density or device constraints make it hostile.",
        ],
        "logic_pattern": [
            f"{name} is treated as generic infrastructure even though product rules are embedded inside it.",
            "Failure modes are undocumented, so validation and retries diverge between repos.",
        ],
        "component_contract": [
            f"{name} omits state or event contracts, so the implementation guesses behavior.",
            "Keyboard or accessibility rules are left implicit and regress later.",
        ],
        "app_archetype": [
            f"{name} is too vague to guide shell or workflow selection.",
            "The archetype smuggles project-specific decoration instead of reusable structure.",
        ],
        "build_recipe": [
            f"{name} bundles pieces that fight each other and still calls itself reusable.",
            "The recipe omits guardrails, so generators produce incompatible starters.",
        ],
        "spec_sheet": [
            f"{name} leaves core decisions unstated, so downstream prompts grow vague again.",
            "Out-of-scope boundaries are missing, so work expands unpredictably.",
        ],
        "dex_prompt": [
            f"{name} still requires heroic interpretation because acceptance criteria are weak.",
            "Forbidden work is absent, so prompt-driven edits drift into adjacent lanes.",
        ],
        "review_packet_template": [
            f"{name} packets omit tests or commands, so review turns into archaeology.",
            "Merge readiness is stated without enough evidence to justify it.",
        ],
        "decision_log": [
            f"{name} records the decision but not the reason, so future agents reverse it casually.",
            "No reversal condition is captured, so the log cannot age gracefully.",
        ],
        "teardown": [
            f"{name} collapses into screenshots and vibes instead of reusable extracted decisions.",
            "Source boundaries are unclear, so copied assets slip in with the teardown.",
        ],
        "source_reference_rules": [
            f"{name} is too vague to constrain real reference extraction work.",
            "The rule set forgets to forbid copying brand assets or proprietary copy.",
        ],
        "app_idea_intake": [
            f"{name} captures excitement but not constraints, data, or workflows.",
            "The intake jumps to implementation before the idea is structurally understood.",
        ],
        "folder_skeleton": [
            f"{name} is too vague to drive consistent repo setup.",
            "The skeleton ignores docs or tests, so later tasks reintroduce ad hoc sprawl.",
        ],
    }
    return failures[item_type]


def default_tags(item_type: str, template: bool) -> list[str]:
    tags = [item_type]
    if template:
        tags.append("template")
    if item_type == "folder_skeleton":
        tags.append("repo_shape")
    return tags


def add_type_specific_fields(item: dict[str, Any]) -> None:
    item_type = item["type"]
    if item_type == "ui_pattern":
        item["ui_pattern"] = {
            "why_exists": "Describe why the pattern exists and what interface pressure it relieves.",
            "data_needs": [
                field_spec("selection", "object", "Current selection or focus target required by the surface."),
            ],
            "layout_regions": ["primary_region", "secondary_region"],
            "interaction_notes": ["Describe primary interactions, collapse behavior, and navigation flow."],
            "accessibility_notes": ["Describe focus order, labels, and keyboard affordances."],
        }
        return
    if item_type == "logic_pattern":
        item["logic_pattern"] = {
            "trigger_points": ["Describe where the pattern starts or is invoked."],
            "data_flow": ["Describe the ordered data flow through the pattern."],
            "invariants": ["Describe one invariant the implementation must preserve."],
            "extension_points": ["Describe where the pattern is safe to extend without breaking contract."],
            "failure_signals": ["Describe how the pattern reports invalid input or downstream failure."],
        }
        return
    if item_type == "component_contract":
        item["component_contract"] = {
            "props_needed": [field_spec("value", "object", "Primary props required to render the component.")],
            "state_needed": [field_spec("view_state", "string", "Primary state required to drive view behavior.")],
            "events_emitted": [output_spec("on_submit", "event", "Primary event the component emits to its owner.")],
            "empty_state": "Describe the empty state and what the user should understand from it.",
            "loading_state": "Describe the loading state and any placeholders or disabled actions.",
            "error_state": "Describe the error state and available recovery path.",
            "keyboard_behavior": ["Describe keyboard shortcuts, focus movement, and submit/cancel behavior."],
            "responsive_behavior": ["Describe breakpoint, pane-collapse, or overflow behavior."],
            "accessibility_rules": ["Describe labels, roles, announcements, and focus expectations."],
            "test_cases": ["Component renders with valid props."],
        }
        return
    if item_type == "app_archetype":
        item["app_archetype"] = {
            "intent": "Describe the product category and why this archetype exists.",
            "primary_surfaces": ["left_rail", "main_workspace", "right_inspector"],
            "primary_workflows": ["Describe one core workflow the archetype supports well."],
            "suggested_patterns": ["command_palette"],
            "suggested_logic": ["settings_persistence"],
            "common_risks": ["Describe a common overbuild or workflow mismatch risk."],
        }
        return
    if item_type == "build_recipe":
        item["build_recipe"] = {
            "recipe_goal": "Describe the app or workflow this recipe assembles.",
            "uses_features": ["feature_card_template"],
            "uses_ui_patterns": ["ui_pattern_template"],
            "uses_logic_patterns": ["logic_pattern_template"],
            "outputs": ["Describe the starter app or asset bundle this recipe should yield."],
            "guardrails": ["Describe one incompatibility or anti-pattern the recipe should avoid."],
        }
        return
    if item_type == "spec_sheet":
        item["spec_sheet"] = {
            "sections": [
                section_spec("product_intent", "Describe the user-facing purpose of the app."),
                section_spec("required_surfaces", "List the required surfaces."),
                section_spec("selected_patterns", "List selected patterns and why they are included."),
                section_spec("first_vertical_slice", "Describe the narrowest end-to-end slice."),
                section_spec("out_of_scope", "State what this spec intentionally excludes."),
            ],
            "required_decisions": ["surfaces", "data_model", "first_vertical_slice", "out_of_scope"],
            "acceptance_gates": ["Describe the evidence required before the spec is considered implementation-ready."],
            "forbidden_drift": ["Describe a class of spec drift the implementation must not introduce."],
        }
        return
    if item_type == "dex_prompt":
        item["dex_prompt"] = {
            "prompt_kinds": [
                "repo_skeleton",
                "ui_fixture_prototype",
                "component_build",
                "logic_module",
                "test_hardening",
                "review_pass",
            ],
            "required_fields": [
                section_spec("goal", "State the exact task goal."),
                section_spec("scope", "State the allowed paths and responsibility boundary."),
                section_spec("acceptance", "State what done looks like."),
                section_spec("forbidden_work", "State what the agent must not touch."),
            ],
            "acceptance_rules": ["Every prompt should define files, checks, and bounded outcomes."],
            "forbidden_work": ["Do not spill into adjacent features or repos without being asked."],
        }
        return
    if item_type == "review_packet_template":
        item["review_packet"] = {
            "sections": [
                section_spec("goal", "State the completed task goal."),
                section_spec("files_changed", "List the files changed."),
                section_spec("commands_run", "List the commands executed."),
                section_spec("tests_run", "List tests executed and outcomes."),
                section_spec("screenshots", "List screenshot evidence or explicitly say none."),
                section_spec("risks", "Capture shipping risks."),
                section_spec("known_issues", "Capture known unresolved issues."),
                section_spec("follow_up_tasks", "Capture immediate next tasks."),
                section_spec("merge_recommendation", "State whether the change is ready to merge."),
            ],
            "required_artifacts": ["files_changed", "commands_run", "tests_run", "merge_recommendation"],
            "merge_recommendation_values": ["merge", "hold", "needs_follow_up"],
        }
        return
    if item_type == "decision_log":
        item["decision_log"] = {
            "decision_fields": [
                section_spec("decision", "State the decision."),
                section_spec("reason", "State why the decision was made."),
                section_spec("alternatives_rejected", "List rejected alternatives."),
                section_spec("reversal_condition", "State when the decision should be revisited."),
            ],
            "reversal_conditions": ["A real blocker appears, or new evidence invalidates the original choice."],
            "evidence_expectations": ["Each entry should reference the feature, project, and affected surfaces."],
        }
        return
    if item_type == "teardown":
        item["teardown"] = {
            "capture_fields": [
                section_spec("layout", "Describe pane structure and hierarchy."),
                section_spec("navigation_model", "Describe how users move through the product."),
                section_spec("state_behavior", "Describe selection, collapse, and inspector behavior."),
                section_spec("settings_structure", "Describe how preferences are organized."),
            ],
            "reusable_decisions": ["Describe one reusable decision extracted from the teardown."],
            "do_not_copy": ["List branding, copy, visual assets, or code that must not be copied."],
            "source_reference_rules": ["Reference the rule set that governs this teardown."],
        }
        return
    if item_type == "source_reference_rules":
        item["source_reference_rules"] = {
            "allowed_reference_types": ["layout_structure", "workflow_logic", "navigation_model", "state_patterns"],
            "forbidden_copying": ["branding", "logos", "illustrations", "proprietary_copy", "source_code"],
            "attribution_rules": ["Record source product and purpose of the teardown or reference."],
            "review_triggers": ["Escalate when a reference risks copying protected assets or distinctive branding."],
        }
        return
    if item_type == "app_idea_intake":
        item["app_idea_intake"] = {
            "capture_fields": [
                section_spec("idea", "State the app idea in plain language."),
                section_spec("users", "State the intended user or operator."),
                section_spec("core_workflows", "State the main workflows."),
                section_spec("constraints", "State hard constraints, risks, or non-goals."),
            ],
            "required_constraints": ["data_shape", "surfaces", "non_goals", "success_signal"],
            "output_targets": ["spec_sheet", "build_recipe", "dex_prompt"],
        }
        return
    if item_type == "folder_skeleton":
        item["folder_skeleton"] = {
            "root": ".",
            "directories": [
                "features_tool",
                "library/items",
                "library/schemas",
                "tests",
                "docs",
                "state",
            ],
            "files": [
                file_spec("README.md", "High-level repo entry point and launch commands."),
                file_spec("pyproject.toml", "Packaging and CLI entrypoint."),
                file_spec("docs/contract.md", "Lock interface and metadata contracts before growth."),
                file_spec("docs/roadmap.md", "Track phases and what is intentionally deferred."),
                file_spec("docs/decisions.md", "Record key design decisions and reversal conditions."),
                file_spec("tests/test_library.py", "Minimal proof that the library foundation works."),
            ],
            "first_vertical_slice": [
                "Lock the schema and template set.",
                "Provide create-item, validate-library, list, and search CLI commands.",
                "Prove the repo with tests before adding richer content or UI layers.",
            ],
        }


def field_spec(name: str, field_type: str, description: str, *, required: bool = True) -> dict[str, Any]:
    return {
        "name": name,
        "type": field_type,
        "required": required,
        "description": description,
    }


def output_spec(name: str, field_type: str, description: str) -> dict[str, Any]:
    return {
        "name": name,
        "type": field_type,
        "description": description,
    }


def section_spec(name: str, description: str, *, required: bool = True) -> dict[str, Any]:
    return {
        "name": name,
        "required": required,
        "description": description,
    }


def file_spec(path: str, purpose: str, *, starter_content: str = "") -> dict[str, Any]:
    spec = {
        "path": path,
        "purpose": purpose,
    }
    if starter_content:
        spec["starter_content"] = starter_content
    return spec


def unique_strings(values: list[str]) -> list[str]:
    seen: set[str] = set()
    result: list[str] = []
    for value in values:
        text = value.strip()
        if not text:
            continue
        key = text.casefold()
        if key in seen:
            continue
        seen.add(key)
        result.append(text)
    return result


def library_item_schema_payload() -> dict[str, Any]:
    type_specific_definitions = {
        "ui_pattern": ["why_exists", "data_needs", "layout_regions", "interaction_notes", "accessibility_notes"],
        "logic_pattern": ["trigger_points", "data_flow", "invariants", "extension_points", "failure_signals"],
        "component_contract": [
            "props_needed",
            "state_needed",
            "events_emitted",
            "empty_state",
            "loading_state",
            "error_state",
            "keyboard_behavior",
            "responsive_behavior",
            "accessibility_rules",
            "test_cases",
        ],
        "app_archetype": [
            "intent",
            "primary_surfaces",
            "primary_workflows",
            "suggested_patterns",
            "suggested_logic",
            "common_risks",
        ],
        "build_recipe": [
            "recipe_goal",
            "uses_features",
            "uses_ui_patterns",
            "uses_logic_patterns",
            "outputs",
            "guardrails",
        ],
        "spec_sheet": ["sections", "required_decisions", "acceptance_gates", "forbidden_drift"],
        "dex_prompt": ["prompt_kinds", "required_fields", "acceptance_rules", "forbidden_work"],
        "review_packet": ["sections", "required_artifacts", "merge_recommendation_values"],
        "decision_log": ["decision_fields", "reversal_conditions", "evidence_expectations"],
        "teardown": ["capture_fields", "reusable_decisions", "do_not_copy", "source_reference_rules"],
        "source_reference_rules": ["allowed_reference_types", "forbidden_copying", "attribution_rules", "review_triggers"],
        "app_idea_intake": ["capture_fields", "required_constraints", "output_targets"],
        "folder_skeleton": ["root", "directories", "files", "first_vertical_slice"],
    }
    properties = {
        "schema_version": {"const": LIBRARY_SCHEMA_VERSION},
        "name": {"type": "string", "minLength": 1},
        "slug": {"type": "string", "pattern": SLUG_RE.pattern},
        "type": {"enum": list(ITEM_TYPES)},
        "template": {"type": "boolean"},
        "status": {"enum": list(STATUS_VALUES)},
        "summary": {"type": "string", "minLength": 1},
        "problem_solved": {"type": "string", "minLength": 1},
        "when_to_use": {"type": "array", "items": {"type": "string"}},
        "when_not_to_use": {"type": "array", "items": {"type": "string"}},
        "required_inputs": {"type": "array", "items": {"$ref": "#/$defs/input_field"}},
        "expected_outputs": {"type": "array", "items": {"$ref": "#/$defs/output_field"}},
        "dependencies": {"type": "array", "items": {"type": "string"}},
        "compatible_features": {"type": "array", "items": {"type": "string"}},
        "implementation_notes": {"type": "array", "items": {"type": "string"}},
        "known_failure_modes": {"type": "array", "items": {"type": "string"}},
        "example_projects": {"type": "array", "items": {"type": "string"}},
        "tags": {"type": "array", "items": {"type": "string"}},
        "ui_surfaces": {"type": "array", "items": {"type": "string"}},
        "logic_patterns": {"type": "array", "items": {"type": "string"}},
        "app_archetypes": {"type": "array", "items": {"type": "string"}},
        "languages": {"type": "array", "items": {"type": "string"}},
        "frameworks": {"type": "array", "items": {"type": "string"}},
        "complexity": {"enum": list(LEVEL_VALUES)},
        "reusability": {"enum": list(LEVEL_VALUES)},
        "risk": {"enum": list(LEVEL_VALUES)},
    }
    definitions: dict[str, Any] = {
        "input_field": {
            "type": "object",
            "required": ["name", "type", "required", "description"],
            "properties": {
                "name": {"type": "string"},
                "type": {"type": "string"},
                "required": {"type": "boolean"},
                "description": {"type": "string"},
            },
        },
        "output_field": {
            "type": "object",
            "required": ["name", "type", "description"],
            "properties": {
                "name": {"type": "string"},
                "type": {"type": "string"},
                "description": {"type": "string"},
            },
        },
    }
    all_of: list[dict[str, Any]] = []
    for item_type, payload_key in TYPE_PAYLOAD_KEYS.items():
        definitions[payload_key] = {"type": "object", "required": type_specific_definitions[payload_key]}
        properties[payload_key] = {"$ref": f"#/$defs/{payload_key}"}
        all_of.append(
            {
                "if": {"properties": {"type": {"const": item_type}}},
                "then": {"required": [payload_key]},
            }
        )
    return {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "library_item_schema_v1.json",
        "title": "Dex Feature Library Item",
        "type": "object",
        "required": list(COMMON_REQUIRED_FIELDS),
        "properties": properties,
        "allOf": all_of,
        "$defs": definitions,
    }


def seed_library_items() -> list[dict[str, Any]]:
    templates: list[dict[str, Any]] = []
    template_specs = [
        {
            "type": "feature_card",
            "name": "Feature Card Template",
            "slug": "feature_card_template",
            "summary": "Machine-first template for reusable feature cards.",
            "dependencies": ["library_item_schema_v1"],
            "example_projects": ["codex_shell", "wiki_browser"],
            "compatible_features": ["ui_pattern_template", "logic_pattern_template", "component_contract_template"],
        },
        {
            "type": "ui_pattern",
            "name": "UI Pattern Template",
            "slug": "ui_pattern_template",
            "summary": "Machine-first template for reusable UI pattern cards.",
            "dependencies": ["feature_card_template"],
            "ui_surfaces": ["surface_template"],
            "example_projects": ["codex_shell", "wiki_browser"],
            "compatible_features": ["feature_card_template", "component_contract_template"],
        },
        {
            "type": "logic_pattern",
            "name": "Logic Pattern Template",
            "slug": "logic_pattern_template",
            "summary": "Machine-first template for reusable logic pattern cards.",
            "dependencies": ["feature_card_template"],
            "logic_patterns": ["pipeline_template"],
            "example_projects": ["codex_shell", "automation_dashboard"],
            "compatible_features": ["feature_card_template", "component_contract_template"],
        },
        {
            "type": "component_contract",
            "name": "Component Contract Template",
            "slug": "component_contract_template",
            "summary": "Machine-first template for buildable component contracts.",
            "dependencies": ["feature_card_template", "ui_pattern_template"],
            "ui_surfaces": ["component_surface"],
            "example_projects": ["codex_shell", "review_console"],
            "compatible_features": ["ui_pattern_template", "logic_pattern_template"],
        },
        {
            "type": "app_archetype",
            "name": "App Archetype Template",
            "slug": "app_archetype_template",
            "summary": "Machine-first template for recurring app shapes.",
            "dependencies": ["ui_pattern_template", "logic_pattern_template"],
            "app_archetypes": ["archetype_template"],
            "example_projects": ["codex_shell", "dashboard_suite"],
            "compatible_features": ["build_recipe_template", "spec_sheet_template"],
        },
        {
            "type": "build_recipe",
            "name": "Build Recipe Template",
            "slug": "build_recipe_template",
            "summary": "Machine-first template for bundling reusable app pieces.",
            "dependencies": ["feature_card_template", "ui_pattern_template", "logic_pattern_template"],
            "app_archetypes": ["archetype_template"],
            "example_projects": ["visual_lab", "automation_dashboard"],
            "compatible_features": ["app_archetype_template", "spec_sheet_template"],
        },
        {
            "type": "spec_sheet",
            "name": "Spec Sheet Template",
            "slug": "spec_sheet_template",
            "summary": "Machine-first template for implementation-ready app specs.",
            "dependencies": ["feature_card_template", "build_recipe_template"],
            "example_projects": ["codex_shell", "scanner_console"],
            "compatible_features": ["dex_prompt_template", "app_idea_intake_template"],
        },
        {
            "type": "dex_prompt",
            "name": "Dex Prompt Template",
            "slug": "dex_prompt_template",
            "summary": "Machine-first template for bounded Dex or Codex prompts.",
            "dependencies": ["spec_sheet_template"],
            "example_projects": ["codex_shell", "wiki_browser"],
            "compatible_features": ["review_packet_template", "spec_sheet_template"],
        },
        {
            "type": "review_packet_template",
            "name": "Review Packet Template",
            "slug": "review_packet_template",
            "summary": "Standard review packet template for completed Dex or Codex tasks.",
            "dependencies": ["dex_prompt_template"],
            "example_projects": ["codex_shell", "wiki_browser"],
            "compatible_features": ["decision_log_template", "component_contract_template"],
        },
        {
            "type": "decision_log",
            "name": "Decision Log Template",
            "slug": "decision_log_template",
            "summary": "Machine-first template for durable design decisions.",
            "dependencies": ["review_packet_template"],
            "example_projects": ["skynet_shell", "wiki_browser"],
            "compatible_features": ["spec_sheet_template", "feature_card_template"],
        },
        {
            "type": "teardown",
            "name": "Teardown Template",
            "slug": "teardown_template",
            "summary": "Machine-first template for extracting reusable decisions from reference products.",
            "dependencies": ["source_reference_rules_template"],
            "example_projects": ["codex_shell", "figma_study"],
            "compatible_features": ["ui_pattern_template", "app_archetype_template"],
        },
        {
            "type": "source_reference_rules",
            "name": "Source Reference Rules Template",
            "slug": "source_reference_rules_template",
            "summary": "Machine-first template for reference and adaptation boundaries.",
            "dependencies": [],
            "example_projects": ["codex_shell", "figma_study"],
            "compatible_features": ["teardown_template"],
        },
        {
            "type": "app_idea_intake",
            "name": "App Idea Intake Template",
            "slug": "app_idea_intake_template",
            "summary": "Machine-first template for turning vague app ideas into structured intake.",
            "dependencies": [],
            "example_projects": ["new_project_queue", "automation_dashboard"],
            "compatible_features": ["spec_sheet_template", "app_archetype_template"],
        },
        {
            "type": "folder_skeleton",
            "name": "Folder Skeleton Template",
            "slug": "folder_skeleton_template",
            "summary": "Machine-first template for standard repo skeletons.",
            "dependencies": ["review_packet_template"],
            "example_projects": ["codex_shell", "wiki_browser"],
            "compatible_features": ["spec_sheet_template", "feature_card_template"],
        },
    ]
    for spec in template_specs:
        item = build_library_item(
            spec["type"],
            spec["name"],
            slug=spec["slug"],
            template=True,
            summary=spec["summary"],
            tags=spec.get("tags"),
            ui_surfaces=spec.get("ui_surfaces"),
            logic_patterns=spec.get("logic_patterns"),
            app_archetypes=spec.get("app_archetypes"),
            example_projects=spec.get("example_projects"),
            dependencies=spec.get("dependencies"),
            languages=spec.get("languages"),
            frameworks=spec.get("frameworks"),
            complexity=spec.get("complexity", "medium"),
            reusability=spec.get("reusability", "high"),
            risk=spec.get("risk", "low"),
        )
        item["compatible_features"] = spec["compatible_features"]
        templates.append(item)

    concrete_skeleton = build_library_item(
        "folder_skeleton",
        "Dex Feature Library Repo v1",
        slug="dex_feature_library_repo_v1",
        summary="Concrete starter skeleton for a Dex-queryable feature library repo.",
        tags=["starter", "repo_foundation"],
        ui_surfaces=["wiki_browser", "universal_inspector"],
        logic_patterns=["validation_pipeline", "search_index"],
        app_archetypes=["wiki_browser", "agent_dashboard"],
        example_projects=["features", "codex_shell"],
        dependencies=["folder_skeleton_template", "review_packet_template", "decision_log_template"],
        languages=["python"],
        frameworks=["cli"],
        complexity="medium",
        reusability="high",
        risk="low",
    )
    concrete_skeleton["compatible_features"] = [
        "feature_card_template",
        "source_reference_rules_template",
        "review_packet_template",
    ]
    concrete_skeleton["folder_skeleton"] = {
        "root": ".",
        "directories": [
            "features_tool",
            "library/items",
            "library/schemas",
            "tests",
            "docs",
            "state",
        ],
        "files": [
            file_spec("README.md", "High-level repo entry point and launch commands."),
            file_spec("pyproject.toml", "Packaging and CLI entrypoint."),
            file_spec("docs/contract.md", "Lock metadata and CLI contracts."),
            file_spec("docs/roadmap.md", "Track phases and what is intentionally deferred."),
            file_spec("docs/decisions.md", "Record design decisions and reversal conditions."),
            file_spec("tests/test_library.py", "Regression proof for schema, templates, and CLI."),
        ],
        "first_vertical_slice": [
            "Seed the library schema and core templates.",
            "Ship create-item, validate-library, list, and search CLI commands.",
            "Keep the repo queryable before adding richer UI or command-centre layers.",
        ],
    }
    return templates + [concrete_skeleton]


def seed_asset_payloads() -> dict[Path, dict[str, Any]]:
    assets: dict[Path, dict[str, Any]] = {
        SCHEMA_RELATIVE_PATH: library_item_schema_payload(),
    }
    for item in seed_library_items():
        assets[relative_item_path(item["type"], item["slug"])] = item
    return assets


def seed_library_root(root: Path, *, overwrite: bool = False) -> dict[str, Any]:
    written: list[str] = []
    for relative_path, payload in seed_asset_payloads().items():
        path = root / relative_path
        if path.exists() and not overwrite:
            continue
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
        written.append(str(path))
    return {"root": str(root), "asset_count": len(written), "written": written}


def validate_library(root_or_path: Path) -> dict[str, Any]:
    entries = load_library_entries(root_or_path)
    errors: list[str] = []
    warnings: list[str] = []

    if root_or_path.is_file():
        if not entries:
            errors.append(f"No library item found at {root_or_path}")
        else:
            errors.extend(entries[0].errors)
        return {
            "status": "pass" if not errors else "fail",
            "root": str(root_or_path),
            "item_count": len(entries),
            "error_count": len(errors),
            "warning_count": len(warnings),
            "errors": errors,
            "warnings": warnings,
            "items": [entry_summary(entry) for entry in entries],
        }

    items_root = root_or_path / ITEMS_RELATIVE_ROOT
    schema_path = root_or_path / SCHEMA_RELATIVE_PATH
    if not root_or_path.exists():
        errors.append(f"Library root does not exist: {root_or_path}")
    elif not items_root.exists():
        errors.append(f"Library item directory does not exist: {items_root}")
    if not schema_path.exists():
        errors.append(f"Library schema does not exist: {schema_path}")
    if not entries:
        errors.append(f"No library items found under {items_root}")

    slug_to_paths: dict[str, list[str]] = {}
    for entry in entries:
        errors.extend(entry.errors)
        if entry.item:
            slug_to_paths.setdefault(entry.item["slug"], []).append(str(entry.path))
    for slug, paths in slug_to_paths.items():
        if len(paths) > 1:
            errors.append(f"Duplicate slug {slug!r} found in: {', '.join(paths)}")

    known_slugs = set(slug_to_paths)
    for entry in entries:
        if not entry.item:
            continue
        for compatible_slug in entry.item.get("compatible_features", []):
            if compatible_slug not in known_slugs:
                warnings.append(
                    f"{entry.item['slug']}: compatible_features references unknown slug {compatible_slug!r}"
                )

    return {
        "status": "pass" if not errors else "fail",
        "root": str(root_or_path),
        "item_count": len(entries),
        "template_count": sum(1 for entry in entries if entry.item and entry.item.get("template")),
        "error_count": len(errors),
        "warning_count": len(warnings),
        "errors": errors,
        "warnings": warnings,
        "items": [entry_summary(entry) for entry in entries],
    }


def load_library_entries(root_or_path: Path) -> list[LibraryEntry]:
    if root_or_path.is_file():
        return [load_library_entry(root_or_path)]
    items_root = root_or_path / ITEMS_RELATIVE_ROOT
    if not items_root.exists():
        return []
    return [load_library_entry(path) for path in sorted(items_root.rglob("*.json"))]


def load_library_entry(path: Path) -> LibraryEntry:
    try:
        payload = json.loads(path.read_text())
    except Exception as exc:
        return LibraryEntry(path=path, item=None, errors=(f"{path}: invalid JSON ({exc})",))
    errors = tuple(validate_library_item(payload, path=path))
    return LibraryEntry(path=path, item=payload, errors=errors)


def validate_library_item(item: dict[str, Any], *, path: Path | None = None) -> list[str]:
    prefix = f"{path}: " if path is not None else ""
    if not isinstance(item, dict):
        return [f"{prefix}item must be a JSON object"]
    errors: list[str] = []

    for field in COMMON_REQUIRED_FIELDS:
        if field not in item:
            errors.append(f"{prefix}missing required field {field!r}")

    if item.get("schema_version") != LIBRARY_SCHEMA_VERSION:
        errors.append(f"{prefix}schema_version must be {LIBRARY_SCHEMA_VERSION!r}")
    if item.get("type") not in ITEM_TYPES:
        errors.append(f"{prefix}type must be one of {', '.join(ITEM_TYPES)}")
    if not isinstance(item.get("name"), str) or not item.get("name", "").strip():
        errors.append(f"{prefix}name must be a non-empty string")
    slug = item.get("slug")
    if not isinstance(slug, str) or not SLUG_RE.match(slug):
        errors.append(f"{prefix}slug must match {SLUG_RE.pattern}")
    if not isinstance(item.get("template"), bool):
        errors.append(f"{prefix}template must be a boolean")
    if item.get("status") not in STATUS_VALUES:
        errors.append(f"{prefix}status must be one of {', '.join(STATUS_VALUES)}")
    if item.get("template") is True and item.get("status") != "template":
        errors.append(f"{prefix}template items must use status 'template'")
    if item.get("template") is False and item.get("status") == "template":
        errors.append(f"{prefix}non-template items cannot use status 'template'")
    for field in ("summary", "problem_solved"):
        if not isinstance(item.get(field), str) or not item.get(field, "").strip():
            errors.append(f"{prefix}{field} must be a non-empty string")
    for field in STRING_LIST_FIELDS:
        errors.extend(validate_string_list(item.get(field), f"{prefix}{field}"))
    errors.extend(validate_field_list(item.get("required_inputs"), f"{prefix}required_inputs", require_required=True))
    errors.extend(validate_field_list(item.get("expected_outputs"), f"{prefix}expected_outputs"))
    for field in ("complexity", "reusability", "risk"):
        if item.get(field) not in LEVEL_VALUES:
            errors.append(f"{prefix}{field} must be one of {', '.join(LEVEL_VALUES)}")

    item_type = item.get("type")
    if item_type == "ui_pattern":
        errors.extend(validate_ui_pattern(item.get("ui_pattern"), prefix))
    elif item_type == "logic_pattern":
        errors.extend(validate_logic_pattern(item.get("logic_pattern"), prefix))
    elif item_type == "component_contract":
        errors.extend(validate_component_contract(item.get("component_contract"), prefix))
    elif item_type == "app_archetype":
        errors.extend(validate_app_archetype(item.get("app_archetype"), prefix))
    elif item_type == "build_recipe":
        errors.extend(validate_build_recipe(item.get("build_recipe"), prefix))
    elif item_type == "spec_sheet":
        errors.extend(validate_spec_sheet(item.get("spec_sheet"), prefix))
    elif item_type == "dex_prompt":
        errors.extend(validate_dex_prompt(item.get("dex_prompt"), prefix))
    elif item_type == "review_packet_template":
        errors.extend(validate_review_packet(item.get("review_packet"), prefix))
    elif item_type == "decision_log":
        errors.extend(validate_decision_log(item.get("decision_log"), prefix))
    elif item_type == "teardown":
        errors.extend(validate_teardown(item.get("teardown"), prefix))
    elif item_type == "source_reference_rules":
        errors.extend(validate_source_reference_payload(item.get("source_reference_rules"), prefix))
    elif item_type == "app_idea_intake":
        errors.extend(validate_app_idea_intake(item.get("app_idea_intake"), prefix))
    elif item_type == "folder_skeleton":
        errors.extend(validate_folder_skeleton(item.get("folder_skeleton"), prefix))
    return errors


def validate_string_list(value: Any, field_name: str) -> list[str]:
    if not isinstance(value, list):
        return [f"{field_name} must be a list of strings"]
    errors: list[str] = []
    seen: set[str] = set()
    for index, item in enumerate(value):
        if not isinstance(item, str) or not item.strip():
            errors.append(f"{field_name}[{index}] must be a non-empty string")
            continue
        key = item.casefold()
        if key in seen:
            errors.append(f"{field_name}[{index}] duplicates an earlier value")
        seen.add(key)
    return errors


def validate_field_list(value: Any, field_name: str, *, require_required: bool = False) -> list[str]:
    if not isinstance(value, list):
        return [f"{field_name} must be a list of objects"]
    errors: list[str] = []
    for index, item in enumerate(value):
        if not isinstance(item, dict):
            errors.append(f"{field_name}[{index}] must be an object")
            continue
        for required_key in ("name", "type", "description"):
            if not isinstance(item.get(required_key), str) or not item.get(required_key, "").strip():
                errors.append(f"{field_name}[{index}].{required_key} must be a non-empty string")
        if require_required and not isinstance(item.get("required"), bool):
            errors.append(f"{field_name}[{index}].required must be a boolean")
    return errors


def validate_section_list(value: Any, field_name: str) -> list[str]:
    if not isinstance(value, list):
        return [f"{field_name} must be a list of objects"]
    errors: list[str] = []
    for index, item in enumerate(value):
        if not isinstance(item, dict):
            errors.append(f"{field_name}[{index}] must be an object")
            continue
        if not isinstance(item.get("name"), str) or not item.get("name", "").strip():
            errors.append(f"{field_name}[{index}].name must be a non-empty string")
        if not isinstance(item.get("description"), str) or not item.get("description", "").strip():
            errors.append(f"{field_name}[{index}].description must be a non-empty string")
        if not isinstance(item.get("required"), bool):
            errors.append(f"{field_name}[{index}].required must be a boolean")
    return errors


def validate_file_specs(value: Any, field_name: str) -> list[str]:
    if not isinstance(value, list):
        return [f"{field_name} must be a list of objects"]
    errors: list[str] = []
    for index, item in enumerate(value):
        if not isinstance(item, dict):
            errors.append(f"{field_name}[{index}] must be an object")
            continue
        for required_key in ("path", "purpose"):
            if not isinstance(item.get(required_key), str) or not item.get(required_key, "").strip():
                errors.append(f"{field_name}[{index}].{required_key} must be a non-empty string")
    return errors


def validate_ui_pattern(value: Any, prefix: str) -> list[str]:
    if not isinstance(value, dict):
        return [f"{prefix}ui_pattern must be an object"]
    errors: list[str] = []
    if not isinstance(value.get("why_exists"), str) or not value.get("why_exists", "").strip():
        errors.append(f"{prefix}ui_pattern.why_exists must be a non-empty string")
    errors.extend(validate_field_list(value.get("data_needs"), f"{prefix}ui_pattern.data_needs", require_required=True))
    for field in ("layout_regions", "interaction_notes", "accessibility_notes"):
        errors.extend(validate_string_list(value.get(field), f"{prefix}ui_pattern.{field}"))
    return errors


def validate_logic_pattern(value: Any, prefix: str) -> list[str]:
    if not isinstance(value, dict):
        return [f"{prefix}logic_pattern must be an object"]
    errors: list[str] = []
    for field in ("trigger_points", "data_flow", "invariants", "extension_points", "failure_signals"):
        errors.extend(validate_string_list(value.get(field), f"{prefix}logic_pattern.{field}"))
    return errors


def validate_component_contract(value: Any, prefix: str) -> list[str]:
    if not isinstance(value, dict):
        return [f"{prefix}component_contract must be an object"]
    errors: list[str] = []
    errors.extend(validate_field_list(value.get("props_needed"), f"{prefix}component_contract.props_needed", require_required=True))
    errors.extend(validate_field_list(value.get("state_needed"), f"{prefix}component_contract.state_needed", require_required=True))
    errors.extend(validate_field_list(value.get("events_emitted"), f"{prefix}component_contract.events_emitted"))
    for field in ("empty_state", "loading_state", "error_state"):
        if not isinstance(value.get(field), str) or not value.get(field, "").strip():
            errors.append(f"{prefix}component_contract.{field} must be a non-empty string")
    for field in ("keyboard_behavior", "responsive_behavior", "accessibility_rules", "test_cases"):
        errors.extend(validate_string_list(value.get(field), f"{prefix}component_contract.{field}"))
    return errors


def validate_app_archetype(value: Any, prefix: str) -> list[str]:
    if not isinstance(value, dict):
        return [f"{prefix}app_archetype must be an object"]
    errors: list[str] = []
    if not isinstance(value.get("intent"), str) or not value.get("intent", "").strip():
        errors.append(f"{prefix}app_archetype.intent must be a non-empty string")
    for field in ("primary_surfaces", "primary_workflows", "suggested_patterns", "suggested_logic", "common_risks"):
        errors.extend(validate_string_list(value.get(field), f"{prefix}app_archetype.{field}"))
    return errors


def validate_build_recipe(value: Any, prefix: str) -> list[str]:
    if not isinstance(value, dict):
        return [f"{prefix}build_recipe must be an object"]
    errors: list[str] = []
    if not isinstance(value.get("recipe_goal"), str) or not value.get("recipe_goal", "").strip():
        errors.append(f"{prefix}build_recipe.recipe_goal must be a non-empty string")
    for field in ("uses_features", "uses_ui_patterns", "uses_logic_patterns", "outputs", "guardrails"):
        errors.extend(validate_string_list(value.get(field), f"{prefix}build_recipe.{field}"))
    return errors


def validate_spec_sheet(value: Any, prefix: str) -> list[str]:
    if not isinstance(value, dict):
        return [f"{prefix}spec_sheet must be an object"]
    errors: list[str] = []
    errors.extend(validate_section_list(value.get("sections"), f"{prefix}spec_sheet.sections"))
    for field in ("required_decisions", "acceptance_gates", "forbidden_drift"):
        errors.extend(validate_string_list(value.get(field), f"{prefix}spec_sheet.{field}"))
    return errors


def validate_dex_prompt(value: Any, prefix: str) -> list[str]:
    if not isinstance(value, dict):
        return [f"{prefix}dex_prompt must be an object"]
    errors: list[str] = []
    errors.extend(validate_string_list(value.get("prompt_kinds"), f"{prefix}dex_prompt.prompt_kinds"))
    errors.extend(validate_section_list(value.get("required_fields"), f"{prefix}dex_prompt.required_fields"))
    errors.extend(validate_string_list(value.get("acceptance_rules"), f"{prefix}dex_prompt.acceptance_rules"))
    errors.extend(validate_string_list(value.get("forbidden_work"), f"{prefix}dex_prompt.forbidden_work"))
    return errors


def validate_review_packet(value: Any, prefix: str) -> list[str]:
    if not isinstance(value, dict):
        return [f"{prefix}review_packet must be an object"]
    errors: list[str] = []
    errors.extend(validate_section_list(value.get("sections"), f"{prefix}review_packet.sections"))
    errors.extend(validate_string_list(value.get("required_artifacts"), f"{prefix}review_packet.required_artifacts"))
    errors.extend(
        validate_string_list(
            value.get("merge_recommendation_values"),
            f"{prefix}review_packet.merge_recommendation_values",
        )
    )
    return errors


def validate_decision_log(value: Any, prefix: str) -> list[str]:
    if not isinstance(value, dict):
        return [f"{prefix}decision_log must be an object"]
    errors: list[str] = []
    errors.extend(validate_section_list(value.get("decision_fields"), f"{prefix}decision_log.decision_fields"))
    errors.extend(validate_string_list(value.get("reversal_conditions"), f"{prefix}decision_log.reversal_conditions"))
    errors.extend(validate_string_list(value.get("evidence_expectations"), f"{prefix}decision_log.evidence_expectations"))
    return errors


def validate_teardown(value: Any, prefix: str) -> list[str]:
    if not isinstance(value, dict):
        return [f"{prefix}teardown must be an object"]
    errors: list[str] = []
    errors.extend(validate_section_list(value.get("capture_fields"), f"{prefix}teardown.capture_fields"))
    errors.extend(validate_string_list(value.get("reusable_decisions"), f"{prefix}teardown.reusable_decisions"))
    errors.extend(validate_string_list(value.get("do_not_copy"), f"{prefix}teardown.do_not_copy"))
    errors.extend(validate_string_list(value.get("source_reference_rules"), f"{prefix}teardown.source_reference_rules"))
    return errors


def validate_source_reference_payload(value: Any, prefix: str) -> list[str]:
    if not isinstance(value, dict):
        return [f"{prefix}source_reference_rules must be an object"]
    errors: list[str] = []
    for field in ("allowed_reference_types", "forbidden_copying", "attribution_rules", "review_triggers"):
        errors.extend(validate_string_list(value.get(field), f"{prefix}source_reference_rules.{field}"))
    return errors


def validate_app_idea_intake(value: Any, prefix: str) -> list[str]:
    if not isinstance(value, dict):
        return [f"{prefix}app_idea_intake must be an object"]
    errors: list[str] = []
    errors.extend(validate_section_list(value.get("capture_fields"), f"{prefix}app_idea_intake.capture_fields"))
    errors.extend(validate_string_list(value.get("required_constraints"), f"{prefix}app_idea_intake.required_constraints"))
    errors.extend(validate_string_list(value.get("output_targets"), f"{prefix}app_idea_intake.output_targets"))
    return errors


def validate_folder_skeleton(value: Any, prefix: str) -> list[str]:
    if not isinstance(value, dict):
        return [f"{prefix}folder_skeleton must be an object"]
    errors: list[str] = []
    if not isinstance(value.get("root"), str) or not value.get("root", "").strip():
        errors.append(f"{prefix}folder_skeleton.root must be a non-empty string")
    errors.extend(validate_string_list(value.get("directories"), f"{prefix}folder_skeleton.directories"))
    errors.extend(validate_file_specs(value.get("files"), f"{prefix}folder_skeleton.files"))
    errors.extend(validate_string_list(value.get("first_vertical_slice"), f"{prefix}folder_skeleton.first_vertical_slice"))
    return errors


def create_library_item(
    root: Path,
    *,
    item_type: str,
    name: str,
    slug: str | None = None,
    summary: str | None = None,
    template: bool = False,
    status: str | None = None,
    tags: list[str] | None = None,
    ui_surfaces: list[str] | None = None,
    logic_patterns: list[str] | None = None,
    app_archetypes: list[str] | None = None,
    example_projects: list[str] | None = None,
    dependencies: list[str] | None = None,
    languages: list[str] | None = None,
    frameworks: list[str] | None = None,
    complexity: str = "medium",
    reusability: str = "high",
    risk: str = "medium",
    force: bool = False,
) -> dict[str, Any]:
    seeded = seed_library_root(root)
    slug_value = slug or slugify(name)
    duplicate = find_entry_by_slug(root, slug_value)
    if duplicate is not None and not force:
        raise ValueError(f"Slug {slug_value!r} already exists at {duplicate.path}")

    payload = build_library_item(
        item_type,
        name,
        slug=slug_value,
        template=template,
        status=status,
        summary=summary,
        tags=tags,
        ui_surfaces=ui_surfaces,
        logic_patterns=logic_patterns,
        app_archetypes=app_archetypes,
        example_projects=example_projects,
        dependencies=dependencies,
        languages=languages,
        frameworks=frameworks,
        complexity=complexity,
        reusability=reusability,
        risk=risk,
    )
    path = root / relative_item_path(item_type, slug_value)
    if path.exists() and not force:
        raise ValueError(f"Target path already exists: {path}")
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
    entry = load_library_entry(path)
    return {
        "root": str(root),
        "path": str(path),
        "seeded_asset_count": seeded["asset_count"],
        "item": payload,
        "valid": entry.valid,
        "errors": list(entry.errors),
    }


def list_library_items(
    root: Path,
    *,
    item_type: str | None = None,
    status: str | None = None,
    complexity: str | None = None,
    reusability: str | None = None,
    risk: str | None = None,
    template_mode: str = "all",
    tags: list[str] | None = None,
    ui_surfaces: list[str] | None = None,
    logic_patterns: list[str] | None = None,
    app_archetypes: list[str] | None = None,
    example_projects: list[str] | None = None,
    dependencies: list[str] | None = None,
    languages: list[str] | None = None,
    frameworks: list[str] | None = None,
) -> dict[str, Any]:
    entries = [
        entry
        for entry in load_library_entries(root)
        if matches_filters(
            entry,
            item_type=item_type,
            status=status,
            complexity=complexity,
            reusability=reusability,
            risk=risk,
            template_mode=template_mode,
            tags=tags,
            ui_surfaces=ui_surfaces,
            logic_patterns=logic_patterns,
            app_archetypes=app_archetypes,
            example_projects=example_projects,
            dependencies=dependencies,
            languages=languages,
            frameworks=frameworks,
        )
    ]
    return {
        "root": str(root),
        "count": len(entries),
        "items": [entry_summary(entry) for entry in entries],
    }


def search_library_items(
    root: Path,
    query: str,
    *,
    limit: int = 10,
    item_type: str | None = None,
    status: str | None = None,
    complexity: str | None = None,
    reusability: str | None = None,
    risk: str | None = None,
    template_mode: str = "all",
    tags: list[str] | None = None,
    ui_surfaces: list[str] | None = None,
    logic_patterns: list[str] | None = None,
    app_archetypes: list[str] | None = None,
    example_projects: list[str] | None = None,
    dependencies: list[str] | None = None,
    languages: list[str] | None = None,
    frameworks: list[str] | None = None,
) -> dict[str, Any]:
    terms = [term.casefold() for term in query.split() if term.strip()]
    if not terms:
        raise ValueError("Search query must contain at least one non-whitespace term")
    matches: list[dict[str, Any]] = []
    for entry in load_library_entries(root):
        if not matches_filters(
            entry,
            item_type=item_type,
            status=status,
            complexity=complexity,
            reusability=reusability,
            risk=risk,
            template_mode=template_mode,
            tags=tags,
            ui_surfaces=ui_surfaces,
            logic_patterns=logic_patterns,
            app_archetypes=app_archetypes,
            example_projects=example_projects,
            dependencies=dependencies,
            languages=languages,
            frameworks=frameworks,
        ):
            continue
        if not entry.item:
            continue
        score, matched_fields = score_item(entry.item, terms)
        if score <= 0:
            continue
        summary = entry_summary(entry)
        summary["score"] = score
        summary["matched_fields"] = matched_fields
        matches.append(summary)
    matches.sort(key=lambda item: (-item["score"], item["type"], item["name"].casefold()))
    return {
        "root": str(root),
        "query": query,
        "count": len(matches[:limit]),
        "items": matches[:limit],
    }


def show_library_item(root: Path, identifier: str, *, item_type: str | None = None) -> dict[str, Any]:
    path_candidate = Path(identifier)
    entry: LibraryEntry | None = None
    if path_candidate.exists():
        entry = load_library_entry(path_candidate)
    else:
        entry = find_entry_by_slug(root, identifier, item_type=item_type)
    if entry is None:
        raise ValueError(f"Unable to find library item {identifier!r}")
    return {
        "path": str(entry.path),
        "valid": entry.valid,
        "errors": list(entry.errors),
        "item": entry.item,
    }


def find_entry_by_slug(root: Path, slug: str, *, item_type: str | None = None) -> LibraryEntry | None:
    matches = [
        entry
        for entry in load_library_entries(root)
        if entry.item
        and entry.item.get("slug") == slug
        and (item_type is None or entry.item.get("type") == item_type)
    ]
    if not matches:
        return None
    if len(matches) > 1:
        raise ValueError(f"Multiple library items share slug {slug!r}; narrow by --type")
    return matches[0]


def matches_filters(
    entry: LibraryEntry,
    *,
    item_type: str | None,
    status: str | None,
    complexity: str | None,
    reusability: str | None,
    risk: str | None,
    template_mode: str,
    tags: list[str] | None,
    ui_surfaces: list[str] | None,
    logic_patterns: list[str] | None,
    app_archetypes: list[str] | None,
    example_projects: list[str] | None,
    dependencies: list[str] | None,
    languages: list[str] | None,
    frameworks: list[str] | None,
) -> bool:
    if not entry.item:
        return False
    item = entry.item
    if item_type and item.get("type") != item_type:
        return False
    if status and item.get("status") != status:
        return False
    if complexity and item.get("complexity") != complexity:
        return False
    if reusability and item.get("reusability") != reusability:
        return False
    if risk and item.get("risk") != risk:
        return False
    if template_mode == "only" and not item.get("template"):
        return False
    if template_mode == "exclude" and item.get("template"):
        return False
    filter_values = {
        "tags": tags,
        "ui_surfaces": ui_surfaces,
        "logic_patterns": logic_patterns,
        "app_archetypes": app_archetypes,
        "example_projects": example_projects,
        "dependencies": dependencies,
        "languages": languages,
        "frameworks": frameworks,
    }
    for field_name, expected_values in filter_values.items():
        if expected_values and not contains_all(item.get(field_name, []), expected_values):
            return False
    return True


def contains_all(actual_values: list[str], expected_values: list[str]) -> bool:
    actual = {value.casefold() for value in actual_values}
    return all(value.casefold() in actual for value in expected_values)


def score_item(item: dict[str, Any], terms: list[str]) -> tuple[int, list[str]]:
    fields = {
        "name": item.get("name"),
        "slug": item.get("slug"),
        "type": item.get("type"),
        "summary": item.get("summary"),
        "problem_solved": item.get("problem_solved"),
        "when_to_use": item.get("when_to_use"),
        "when_not_to_use": item.get("when_not_to_use"),
        "dependencies": item.get("dependencies"),
        "compatible_features": item.get("compatible_features"),
        "implementation_notes": item.get("implementation_notes"),
        "known_failure_modes": item.get("known_failure_modes"),
        "example_projects": item.get("example_projects"),
        "tags": item.get("tags"),
        "ui_surfaces": item.get("ui_surfaces"),
        "logic_patterns": item.get("logic_patterns"),
        "app_archetypes": item.get("app_archetypes"),
        "languages": item.get("languages"),
        "frameworks": item.get("frameworks"),
        "type_details": item.get(TYPE_PAYLOAD_KEYS.get(item.get("type"), "")),
    }
    weights = {
        "name": 5,
        "slug": 4,
        "summary": 4,
        "problem_solved": 3,
        "tags": 3,
        "ui_surfaces": 3,
        "logic_patterns": 3,
        "app_archetypes": 3,
        "type": 2,
        "dependencies": 2,
        "compatible_features": 2,
        "type_details": 2,
    }
    score = 0
    matched_fields: list[str] = []
    for field_name, value in fields.items():
        text = " ".join(flatten_strings(value)).casefold()
        if not text:
            continue
        field_score = sum(text.count(term) for term in terms)
        if field_score:
            score += field_score * weights.get(field_name, 1)
            matched_fields.append(field_name)
    return score, matched_fields


def flatten_strings(value: Any) -> list[str]:
    if value is None:
        return []
    if isinstance(value, str):
        return [value]
    if isinstance(value, bool):
        return []
    if isinstance(value, dict):
        strings: list[str] = []
        for item in value.values():
            strings.extend(flatten_strings(item))
        return strings
    if isinstance(value, list):
        strings: list[str] = []
        for item in value:
            strings.extend(flatten_strings(item))
        return strings
    return [str(value)]


def entry_summary(entry: LibraryEntry) -> dict[str, Any]:
    if not entry.item:
        return {
            "path": str(entry.path),
            "valid": False,
            "error_count": len(entry.errors),
            "errors": list(entry.errors),
        }
    item = entry.item
    return {
        "name": item["name"],
        "slug": item["slug"],
        "type": item["type"],
        "status": item["status"],
        "template": item["template"],
        "summary": item["summary"],
        "path": str(entry.path),
        "tags": item["tags"],
        "ui_surfaces": item["ui_surfaces"],
        "logic_patterns": item["logic_patterns"],
        "app_archetypes": item["app_archetypes"],
        "example_projects": item["example_projects"],
        "dependencies": item["dependencies"],
        "languages": item["languages"],
        "frameworks": item["frameworks"],
        "complexity": item["complexity"],
        "reusability": item["reusability"],
        "risk": item["risk"],
        "valid": entry.valid,
        "error_count": len(entry.errors),
    }
