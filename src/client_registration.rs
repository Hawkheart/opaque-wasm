use crate::hash_methods::Default;
use opaque_ke::{ClientRegistration, ClientRegistrationFinishParameters, RegistrationResponse};
use rand::rngs::OsRng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Registration {
    state: Option<ClientRegistration<Default>>,
    rng: OsRng,
}

#[wasm_bindgen]
impl Registration {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Registration {
        Registration {
            rng: OsRng,
            state: None,
        }
    }

    pub fn start(&mut self, password: &str) -> Result<Vec<u8>, JsValue> {
        let client_registration_start_result =
            match ClientRegistration::<Default>::start(&mut self.rng, &password.as_bytes()) {
                Ok(reply) => reply,
                Err(_e) => return Err("Start failed".into()),
            };
        self.state = Some(client_registration_start_result.state);

        return Ok(client_registration_start_result.message.serialize());
    }

    pub fn finish(self, message: Vec<u8>) -> Result<Vec<u8>, JsValue> {
        let message = match RegistrationResponse::deserialize(&message[..]) {
            Ok(message) => message,
            Err(_e) => return Err("Message deserialize failed".into()),
        };
        let mut rng = self.rng;

        let client_finish_registration_result = match self.state.unwrap().finish(
            &mut rng,
            message,
            ClientRegistrationFinishParameters::default(),
        ) {
            Ok(reply) => reply,
            Err(_e) => return Err("Mismatch messagess".into()),
        };
        return Ok(client_finish_registration_result.message.serialize());
    }
}
