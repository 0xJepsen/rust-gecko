pub(crate) mod gecko {

    use crate::types::Response;
    use serde::de::DeserializeOwned;

    const ORIGIN: &str = "https://api.coingecko.com/api/v3";

    pub fn vec_str_2_comma_str(vector: Vec<&str>) -> String {
        let vec_len = vector.len();
        let mut i = 0;
        let mut vector_str = String::new();

        for item in vector {
            i += 1;
            vector_str.push_str(item);
            if i < vec_len {
                vector_str.push_str(",");
            }
        }
        vector_str
    }

    pub fn get_request<T: DeserializeOwned>(endpoint: &str, params: &str) -> Response<T> {
        let url = [ORIGIN, endpoint, params].join("");
        dbg!("{}", &url);
        let resp = reqwest::blocking::get(url);

        match resp {
            Ok(res) => {
                let status = res.status();
                let payload = res.json::<T>(); // could be `Error` or `Response` but only parses to `Response`

                match payload {
                    Ok(json) => Response {
                        is_success: true,
                        status: status,
                        json: Some(json),
                        error: None,
                    },

                    Err(e) => Response {
                        is_success: false,
                        status: status,
                        json: None,
                        error: Some(e),
                    },
                }
            }
            Err(e) => Response {
                is_success: false,
                status: reqwest::StatusCode::EXPECTATION_FAILED,
                json: None,
                error: Some(e),
            },
        }
        // (status, resp_json)
    }
}
pub mod coins;
mod macros;
pub mod server;
pub mod simple;
pub mod types;
//     pub mod contract {}
//     pub mod asset_platform {}
//     pub mod categories {}
//     pub mod exchanges {}
//     pub mod indexes {}
//     pub mod derivatives {}
//     pub mod exchange_rates {}
//     pub mod search {}
//     pub mod trending {}
//     pub mod global {}
//     pub mod companies {}
