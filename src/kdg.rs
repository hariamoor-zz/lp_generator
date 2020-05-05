use crate::Resource;

fn model_set_objective_function(resource: &Resource) -> String {
    resource
        .knobs
        .iter()
        .flat_map(|knob| {
            knob.layers.iter().flat_map(|layer| {
                layer
                    .basic_nodes
                    .iter()
                    .map(|node| format!("{} {}", node.quality, node.name))
            })
        })
        .collect::<Vec<_>>()
        .join(" + ")
}

fn model_set_budget_constraint(resource: &Resource, budget: f64) -> String {
    let constraint = resource
        .knobs
        .iter()
        .flat_map(|knob| {
            knob.layers.iter().flat_map(|layer| {
                layer
                    .basic_nodes
                    .iter()
                    .map(|node| format!("{} {}", node.cost, node.name))
            })
        })
        .collect::<Vec<_>>()
        .join(" + ");

    format!("cost: {} <= {}", constraint, budget)
}

fn model_set_knob_constraints(resource: &Resource) -> Vec<String> {
    resource
        .knobs
        .iter()
        .map(|knob| {
            let constraint = knob
                .layers
                .iter()
                .flat_map(|layer| layer.basic_nodes.iter().map(|node| node.name.clone()))
                .collect::<Vec<_>>()
                .join(" + ");

            format!("{}.Select: {} = 1", knob.knob_name, constraint)
        })
        .collect()
}

fn model_set_edge_constraints(resource: &Resource) -> Vec<String> {
    resource
        .knobs
        .iter()
        .flat_map(|knob| {
            knob.layers.iter().flat_map(|layer| {
                layer.basic_nodes.iter().flat_map(|node| {
                    node.edges.iter().map(move |edge| {
                        format!(
                            "{source}.{sink}: {sink} - {source} <= 0",
                            source = edge,
                            sink = node.name,
                        )
                    })
                })
            })
        })
        .collect()
}

pub fn build_model(resource: Resource, budget: f64) -> String {
    let objective = model_set_objective_function(&resource);
    let budget = model_set_budget_constraint(&resource, budget);
    let knob = model_set_knob_constraints(&resource);

    let edge = model_set_edge_constraints(&resource);

    let binaries = resource
        .knobs
        .iter()
        .flat_map(|knob| {
            knob.layers
                .iter()
                .flat_map(|layer| layer.basic_nodes.iter().map(|node| node.name.clone()))
        })
        .collect::<Vec<_>>()
        .join(" ");

    format!(
        "Maximize\n\t{}\nSubject To\n\t{}\n\t{}\n\t{}\nBinary\n\t{}\nEnd",
        objective,
        budget,
        knob.join("\n\t"),
        edge.join("\n\t"),
        binaries
    )
}
