mod auth;

use auth::CheckReply;
use protobuf::Message;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::time::Duration;

use crate::auth::CheckRequest;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(CustomAuthz) });
    proxy_wasm::set_http_context(|_, _| -> Box<dyn HttpContext> { Box::new(CustomAuthz) });
}}

struct CustomAuthz;

impl Context for CustomAuthz {
    fn on_grpc_call_response(&mut self, token_id: u32, status_code: u32, response_size: usize) {
        log::info!(
            "grpc_call received: {} : {} : {}",
            token_id.to_string(),
            status_code.to_string(),
            response_size.to_string()
        );
        match self.get_grpc_call_response_body(0, response_size) {
            Some(bytes) => {
                let reply_res = CheckReply::parse_from_bytes(&bytes);
                log::info!("response: {:?}", reply_res);
                match reply_res {
                    Ok(rep) => {
                        log::info!("rep: {}", &rep.success);
                        if rep.success {
                            log::info!("Access granted.");
                            self.resume_http_request();
                            return;
                        }
                    }
                    Err(e) => {
                        log::error!("reply_res err {:?}", e);
                    }
                }
            }
            None => {
                log::error!("Empty response received");
            }
        }

        self.send_http_response(
            403,
            vec![("Powered-By", "proxy-wasm")],
            Some(b"Access forbidden."),
        );
    }
}

impl RootContext for CustomAuthz {
    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        log::info!("CustomAuthz::on_vm_start");
        true
    }
}

impl HttpContext for CustomAuthz {
    fn on_http_request_headers(
        &mut self,
        _num_headers: usize,
        _end_of_stream: bool,
    ) -> proxy_wasm::types::Action {
        let token = self
            .get_http_request_header("token")
            .unwrap_or_else(|| String::from(""));

        let mut req = CheckRequest::new();
        req.set_token(token.to_string());
        let message = req.write_to_bytes().unwrap();

        log::info!("CustomAuthz::checkToken {}", token);

        match self.dispatch_grpc_call(
            "grpc_cluster",
            "auth.Auth",
            "checkToken",
            Vec::<(&str, &[u8])>::new(),
            Some(message.as_slice()),
            Duration::from_secs(5),
        ) {
            Ok(_) => log::info!("gprc_call success"),
            Err(e) => {
                log::error!("grpc_call failed {:?}", e);
                self.send_http_response(
                    403,
                    vec![("Powered-By", "proxy-wasm")],
                    Some(b"Access forbidden."),
                );
            }
        }

        Action::Pause
    }
}
