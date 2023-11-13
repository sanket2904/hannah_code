#[macro_export]
macro_rules! get_function_string {
    ($func: ident) => {
        stringify!($func)
    };
}


#[macro_use]
mod ai_functions;
mod apis;
mod helpers;
mod models;
use helpers::command_line::get_user_response;

#[tokio::main]
async fn main() {


// need to add a frontend agent
// need to refactor the code a bit which gives out the file path or name of the written code
    let usr_req = get_user_response("What software are we building today?");
    let mut managing_agent: models::agents_manager::ManagingAgent = models::agents_manager::ManagingAgent::new(usr_req).await.expect("Failed to create managing agent");
    managing_agent.execute_project().await;
    dbg!(managing_agent);
    
}   
