use crate::{
    ai_functions::aifunc_backend::{
        print_backend_webserver_code, print_fixed_code, print_improved_webserver_code,
        print_rest_api_endpoints,
    },
    helpers::{
        command_line::{confirm_safe_to_proceed, PrintCommand},
        general::{
            ai_task_request, check_status_code, read_code_template, read_exec_main_contents,
            save_backend_code,
        },
    },
    models::agent_basic::basic_agent::{AgentState, BasicAgent},
};
use async_trait::async_trait;
use std::{
    process::{Command, Stdio},
    time,
};

use super::agent_traits::{FactSheet, RouteObject, SpecialFunctions};

#[derive(Debug)]
pub struct AgentBackendDeveloper {
    attributes: BasicAgent,
    bug_errors: Option<String>,
    bug_count: u8,
}

impl AgentBackendDeveloper {
    pub fn new() -> Self {
        let attributes = BasicAgent {
            objective: "Develops the backend code for the webserver and mongodb database"
                .to_string(),
            position: "Backend Developer".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };
        Self {
            attributes: attributes,
            bug_errors: None,
            bug_count: 0,
        }
    }
    async fn call_initial_backend_code(&mut self, factsheet: &mut FactSheet) {
        let code_template = read_code_template();
        let msg_context = format!(
            "CODE TEMPLATE: {} \n PROJECT_DESCRIPTION: {} \n",
            code_template, factsheet.project_description
        );
        let ai_response = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_backend_webserver_code),
            print_backend_webserver_code,
        )
        .await;
        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }
    async fn call_improved_backend_code(&mut self, factsheet: &mut FactSheet) {
        let code_template = read_code_template();
        let msg_context = format!(
            "CODE TEMPLATE: {:?} \n PROJECT_DESCRIPTION: {:?} \n",
            factsheet.backend_code, factsheet
        );
        let ai_response = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_improved_webserver_code),
            print_improved_webserver_code,
        )
        .await;
        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }

    async fn call_fix_code_bugs(&mut self, factsheet: &mut FactSheet) {
        let code_template = read_code_template();
        let msg_context = format!(
            "BROKEN_CODE: {:?} \n ERROR_BUGS: {:?} \n
            THIS FUNCTION ONLY OUTPUTS CODE. JUST OUTPUT THE CODE.",
            factsheet.backend_code, self.bug_errors
        );
        let ai_response = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_fixed_code),
            print_fixed_code,
        )
        .await;
        save_backend_code(&ai_response);
        factsheet.backend_code = Some(ai_response);
    }
    async fn call_exact_rest_api_endpoints(&self) -> String {
        let backend_code = read_exec_main_contents();
        let msg_context = format!("CODE_INPUT: {:?} \n", backend_code);
        let ai_response = ai_task_request(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_rest_api_endpoints),
            print_rest_api_endpoints,
        )
        .await;
        ai_response
    }
}

#[async_trait]
impl SpecialFunctions for AgentBackendDeveloper {
    fn get_attributes_from_agents(&self) -> &BasicAgent {
        &self.attributes
    }
    async fn execute(
        &mut self,
        factsheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while self.attributes.state != AgentState::Finished {
            match &self.attributes.state {
                AgentState::Discovery => {
                    self.call_initial_backend_code(factsheet).await;
                    self.attributes.state = AgentState::Working;
                    continue;
                }
                AgentState::Working => {
                    if self.bug_count == 0 {
                        self.call_improved_backend_code(factsheet).await;
                    } else {
                        self.call_fix_code_bugs(factsheet).await;
                    }
                    self.attributes.state = AgentState::UnitTesting;
                    continue;
                }
                AgentState::UnitTesting => {
                    PrintCommand::UnitTest.print_agent_message(
                        &self.attributes.position,
                        "Backend code unit testing",
                    );
                    let is_safe_to_proceed = confirm_safe_to_proceed();
                    if !is_safe_to_proceed {
                        panic!("Work on AI alignment");
                    }
                    PrintCommand::UnitTest.print_agent_message(
                        &self.attributes.position,
                        "Backend code unit testing: building web server...",
                    );
                    let build_backend_server = Command::new("cargo")
                        .arg("build")
                        .current_dir("/home/ssa006/data_sync/")
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .output()
                        .expect("failed to execute process");

                    if build_backend_server.status.success() {
                        self.bug_count = 0;
                        PrintCommand::UnitTest.print_agent_message(
                            &self.attributes.position,
                            "Backend code unit testing: web server built successfully",
                        );
                    } else {
                        let error_arr: Vec<u8> = build_backend_server.stderr;
                        let error_str = String::from_utf8(error_arr).unwrap();
                        self.bug_count += 1;
                        self.bug_errors = Some(error_str);
                        if self.bug_count > 2 {
                            panic!("Too many bugs");
                        }
                        self.attributes.state = AgentState::Working;
                        continue;
                    }

                    let api_endpoints = self.call_exact_rest_api_endpoints().await;
                    let api_endpoints: Vec<RouteObject> = serde_json::from_str(&api_endpoints)
                        .expect("Failed to decode api endpoints");
                    let check_endpoints: Vec<RouteObject> = api_endpoints
                        .iter()
                        .filter(|&route_object| {
                            route_object.method == "get" && route_object.is_route_dynamic == "false"
                        })
                        .cloned()
                        .collect();
                    factsheet.api_endpoints_schema = Some(check_endpoints.clone());
                    // run backend application
                    PrintCommand::UnitTest.print_agent_message(
                        &self.attributes.position,
                        "Backend Code Unit Testing: Running web server...",
                    );
                    let mut run_backend_server = Command::new("cargo")
                        .arg("run")
                        .current_dir("/home/ssa006/data_sync/")
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                        .expect("failed to run backend application");

                    PrintCommand::UnitTest.print_agent_message(
                        &self.attributes.position,
                        "Backend Code Unit Testing: Launching tests on server in 5 seconds",
                    );
                    tokio::time::sleep(time::Duration::from_secs(5)).await;

                    for endpoint in check_endpoints {
                        let testing_msg = format!(
                            "Testing endpoint {} with method {}",
                            endpoint.route, endpoint.method
                        );
                        PrintCommand::UnitTest
                            .print_agent_message(&self.attributes.position, &testing_msg);
                        let url = format!("http://localhost:1337{}", endpoint.route);
                        match check_status_code(&url).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    panic!("Failed to get status code 200");
                                }
                            } ,
                            Err(_e) => {
                                panic!("Failed to get status code 200"); 
                            },
                        }
                    }

                    self.attributes.state = AgentState::Finished;
                }
                _ => {}
            }
        }
        Ok(())
    }
}
