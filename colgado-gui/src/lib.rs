use std::sync::Arc;
use tokio::task::JoinHandle;

use colgado_logic::{errors::ColgadoLogicError, models::handles::Handles};

pub async fn init_flow() -> Result<(Handles, Arc<[JoinHandle<()>]>), Arc<ColgadoLogicError>> {
    let result = colgado_logic::init_flow().await;
    match result {
        Err(err) => {
            let arc = Arc::new(err);
            Err(arc)
        }
        Ok(value) => Ok(value),
    }
}
