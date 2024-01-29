use burn::backend::NdArrayBackend;
use fsrs::{FSRSItem, FSRSReview, DEFAULT_WEIGHTS, FSRS};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Fsrs)]
pub struct FSRSwasm {
    model: FSRS<NdArrayBackend>,
}

impl Default for FSRSwasm {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen(js_class = Fsrs)]
impl FSRSwasm {
    #[cfg_attr(target_family = "wasm", wasm_bindgen(constructor))]
    pub fn new() -> Self {
        Self {
            model: FSRS::new(Some(&DEFAULT_WEIGHTS)).unwrap(),
        }
    }
    #[wasm_bindgen(js_name = memoryState)]
    pub fn memory_state(&self, ratings: &[u32], delta_ts: &[u32]) -> Vec<f32> {
        let item = FSRSItem {
            reviews: ratings
                .iter()
                .zip(delta_ts)
                .map(|(rating, delta_t)| FSRSReview {
                    rating: *rating,
                    delta_t: *delta_t,
                })
                .collect(),
        };
        let state = self.model.memory_state(item);
        vec![state.stability, state.difficulty]
    }
    #[wasm_bindgen(js_name = nextInterval)]
    pub fn next_interval(
        &self,
        stability: Option<f32>,
        desired_retention: f32,
        rating: u32,
    ) -> u32 {
        self.model
            .next_interval(stability, desired_retention, rating)
    }
    #[wasm_bindgen(js_name = computeWeights)]
    pub fn compute_weights(&self, fsrs_items: String) -> Vec<f32> {
        let fsrs_items: Vec<Vec<Vec<u32>>> = serde_json::from_str(&fsrs_items).unwrap();
        let fsrs_items: Vec<FSRSItem> = fsrs_items
            .into_iter()
            .map(|item| FSRSItem {
                reviews: item
                    .into_iter()
                    .map(|review| FSRSReview {
                        rating: review[0],
                        delta_t: review[1],
                    })
                    .collect(),
            })
            .collect();
        self.model.compute_weights(fsrs_items, None).unwrap()
    }
}

#[test]
fn test_compute_weights() {
    let fsrs_items = "[[[1, 3, 3, 1], [0, 2, 3, 7]], [[1, 3, 3, 3], [0, 2, 3, 5]]]";
    let fsrs = FSRSwasm::new();
    let weights = fsrs.compute_weights(fsrs_items.to_owned());
    dbg!(&weights);
}