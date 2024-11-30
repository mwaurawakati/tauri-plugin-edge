use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::EdgeExt;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.edge().ping(payload)
}
