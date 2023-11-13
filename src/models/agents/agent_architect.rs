// Solution Architect

use async_trait::async_trait;
use reqwest::Client;

use crate::{
    ai_functions::aifunc_architect::{print_project_scope, print_site_urls},
    helpers::{general::{ai_task_request_decoded, check_status_code}, command_line::PrintCommand},
    models::agent_basic::{
        basic_agent::{AgentState, BasicAgent},
        basic_traits::BasicTraits,
    },
};

use super::agent_traits::{FactSheet, ProjectScope, SpecialFunctions};

#[derive(Debug)]
pub struct AgentSolutionArchitect {
    pub attributes: BasicAgent,
}

impl AgentSolutionArchitect {
    pub fn new() -> Self {
        let attributes = BasicAgent {
            objective: "Gathers information and design solutions for software development"
                .to_string(),
            position: "Solutions Architect".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };
        Self {
            attributes: attributes,
        }
    }
    async fn call_project_scope(&mut self, factsheet: &mut FactSheet) -> ProjectScope {
        let msg = format!("{:?}", factsheet.project_description);
        let ai_response = ai_task_request_decoded::<ProjectScope>(
            msg,
            &self.attributes.position,
            get_function_string!(print_project_scope),
            print_project_scope,
        )
        .await;
        factsheet.project_scope = Some(ai_response);
        self.attributes.update_state(AgentState::Finished);
        ai_response
    }
    async fn call_determine_external_urls(
        &mut self,
        factsheet: &mut FactSheet,
        msg_context: String,
    ) {
        let msg = format!("{:?}", factsheet.project_description);
        let ai_response = ai_task_request_decoded::<Vec<String>>(
            msg,
            &self.attributes.position,
            get_function_string!(print_site_urls),
            print_site_urls,
        )
        .await;
        factsheet.external_urls = Some(ai_response);
        self.attributes.state = AgentState::UnitTesting;
    }
}

#[async_trait]
impl SpecialFunctions for AgentSolutionArchitect {
    fn get_attributes_from_agents(&self) -> &BasicAgent {
        &self.attributes
    }
    async fn execute(&mut self, factsheet: &mut FactSheet) -> Result<(), Box<dyn std::error::Error>> {
        while self.attributes.state != AgentState::Finished {
            match self.attributes.state {
                AgentState::Discovery => {
                    let project_scope = self.call_project_scope(factsheet).await;
                    if project_scope.is_external_urls_required {
                        self.call_determine_external_urls(factsheet, factsheet.project_description.clone()).await;
                        self.attributes.state = AgentState::UnitTesting;
                    }
                },
                AgentState::UnitTesting => {
                    let mut excluded_urls:Vec<String> = vec![];
                    let urls: &Vec<String> = factsheet.external_urls.as_ref().expect("No url object on factsheet");
                    for url in urls {
                        let endpoint_str = format!("Testing URL endpoint: {}", url);
                        PrintCommand::UnitTest.print_agent_message(self.attributes.position.as_str() , endpoint_str.as_str());
                        match check_status_code(url).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    excluded_urls.push(url.clone());
                                }
                            },
                            Err(_) => {
                                println!("Failed to check status code for url: {}", url);
                            }
                        }
                    }
                    if excluded_urls.len() > 0 {
                        let new_urls: Vec<String> = factsheet.external_urls.as_ref().unwrap().iter().filter(|url| !excluded_urls.contains(&url)).cloned().collect();
                        factsheet.external_urls = Some(new_urls);
                    }
                    self.attributes.state = AgentState::Finished;
                }, 
                _ => {
                    self.attributes.state = AgentState::Finished;
                }
            }
        }
        Ok(())
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_solution_architect() {

//     }

// }
