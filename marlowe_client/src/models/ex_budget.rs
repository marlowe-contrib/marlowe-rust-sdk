/*
 * Marlowe Runtime REST API
 *
 * REST API for Marlowe Runtime
 *
 * The version of the OpenAPI document: 0.0.5.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExBudget {
    #[serde(rename = "exBudgetCPU")]
    pub ex_budget_cpu: f32,
    #[serde(rename = "exBudgetMemory")]
    pub ex_budget_memory: f32,
}

impl ExBudget {
    pub fn new(ex_budget_cpu: f32, ex_budget_memory: f32) -> ExBudget {
        ExBudget {
            ex_budget_cpu,
            ex_budget_memory,
        }
    }
}


