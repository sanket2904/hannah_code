use crate::{ai_functions::{aifunc_architect::print_project_scope, aifunc_managing::convert_user_input_to_goal}, helpers::general::ai_task_request};

use super::{agent_basic::basic_agent::{BasicAgent, AgentState}, agents::{agent_traits::{FactSheet, SpecialFunctions}, agent_architect::AgentSolutionArchitect, agent_backend::AgentBackendDeveloper}};
#[derive(Debug)]
pub struct ManagingAgent {
    attributes: BasicAgent,
    factsheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunctions>>
}
impl ManagingAgent {
    pub async fn new(usr_req: String) -> Result<Self, Box<dyn std::error::Error>>  {
        let attributes = BasicAgent {
            objective: "Manage agents who are building an excellent software product".to_string(),
            position: "Project Manager".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };
        let project_description = ai_task_request(usr_req, &attributes.position, get_function_string!(convert_user_input_to_goal), convert_user_input_to_goal).await;
        let agents: Vec<Box<dyn SpecialFunctions>> = vec![];
        let factsheet = FactSheet {
            project_description,
            project_scope: None,
            external_urls: None,
            backend_code: None,
            api_endpoints_schema: None,
        };
        Ok(Self {
            attributes,
            factsheet,
            agents
        })
    }
    fn add_agent(&mut self, agent: Box<dyn SpecialFunctions>) {
        self.agents.push(agent);
    }
    fn create_agent(&mut self) {
        self.add_agent(Box::new(AgentSolutionArchitect::new()));
        self.add_agent(Box::new(AgentBackendDeveloper::new()))
        // add backend agent and frontend agent
    }
    pub async fn execute_project(&mut self)  {
        self.create_agent();
        for agent in self.agents.iter_mut() {
            let agent_res =  agent.execute(&mut self.factsheet).await;
            // let agent_info = agent.get_attributes_from_agents();
            // dbg!(agent_info);
        }
    }
}