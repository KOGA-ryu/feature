# Schematic Design Language

This repo needs a consistent way to describe software structure without
re-explaining the drawing style every time.

Use the following design languages as standard schematic families.

## Architectural Floor Plans

Best for:

- app layout
- command centre surfaces
- wiki browser structure
- dashboards and multi-pane shells

Software translation:

- rooms = panels or views
- hallways = workflows
- doors = navigation paths
- walls = boundaries
- dimensions = layout constraints
- room labels = surface purpose

Use when the main question is: `where does this live?`

## Electrical Schematics

Best for:

- logic flow
- event flow
- data flow
- pipeline design

Software translation:

- components = modules
- wires = data or event paths
- power source = upstream input
- switch = branching logic
- fuse = validation or protection step
- ground = fallback or default state
- test point = log or assertion hook

Use when the main question is: `how does this signal move?`

## Mechanical Exploded Views

Best for:

- reusable component decomposition
- parts lists
- assembly order
- replaceable modules

Software translation:

- parts = structs, stores, fixtures, tests, helper modules
- connectors = interfaces or shared state
- assembly order = implementation order
- fasteners = integration seams

Use when the main question is: `what is this made of?`

## PCB Routing

Best for:

- file ownership
- Dex lane boundaries
- integration choke points
- forbidden-write zones

Software translation:

- traces = allowed write or data paths
- vias = controlled cross-layer handoffs
- keep-out zones = forbidden files
- routing noise = dependency mess
- test pads = integration checkpoints

Use when the main question is: `where is change allowed to travel?`

## Subway Maps

Best for:

- workflow navigation
- wiki browsing
- build and review pipelines

Software translation:

- stations = docs, features, or outputs
- lines = workflows or categories
- transfer stations = shared components
- terminal stations = final artifacts
- closures = deprecated or blocked routes

Use when the main question is: `how does work move across the system?`

## Control Room Schematics

Best for:

- multi-Dex monitoring
- operator dashboards
- alert routing
- attention management

Software translation:

- status lights = state indicators
- alarms = blockers or failures
- main board = overview panel
- operator actions = allowed interventions
- drill-down panels = details on demand

Use when the main question is: `what needs attention right now?`

## State Machine Diagrams

Best for:

- feature maturity
- lifecycle design
- Dex session states
- approval flows

Use when the main question is: `what states exist and what transitions are
legal?`

## ERD and Narrow UML Use

Use only the parts that answer real implementation questions:

- entity relationships for library or review-packet data shape
- component diagrams for module boundaries
- sequence diagrams for runtime interactions
- state diagrams for lifecycle rules

Do not use ceremonial UML for its own sake.

## Choosing the Right Template

Use:

- `feature_blueprint_template` for the full buildable feature packet
- `ui_floorplan_template` for layout-heavy surfaces
- `logic_flow_schematic_template` for pipelines and logic crates
- `component_exploded_view_template` for parts decomposition
- `state_machine_template` for lifecycle rules
- `test_bench_template` for fixture-driven proof
- `integration_boundary_template` for allowed and forbidden connections
- `dex_lane_pcb_routing_template` for parallel ownership rules
- `wiki_subway_map_template` for workflow navigation
- `command_centre_control_room_template` for operator-facing monitoring

## Repo-Specific Rule

Pick the minimum drawing language that answers the real question.

Examples:

- `ui.left_rail` should start with a floor plan and exploded view.
- `logic.validation_pipeline` should start with a logic flow and test bench.
- multi-Dex ownership should start with PCB routing, not a floor plan.
- feature maturity should start with a state machine, not a control room.
