use rocket::request::Form;

use crate::{errors::ApiResponse, SETTINGS};

#[post("/hwaccel/<enable>")]
pub async fn enable_hwaccel(enable: bool) -> ApiResponse {
    SETTINGS.write()?.set("HW_ACCEL", enable)?;
    Ok(json!({ "enable": enable }))
}

#[derive(FromForm)]
pub struct TranscodingConfig {
    path: String,
}

#[post("/transcoding", data = "<config>")]
pub async fn set_transcoding_path(config: Form<TranscodingConfig>) -> ApiResponse {
    SETTINGS.write()?.set("TRANSCODING", config.path.clone())?;
    Ok(json!({
        "path": config.path
    }))
}
