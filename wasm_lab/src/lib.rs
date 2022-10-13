use wasm_bindgen::prelude::*;
use tch::vision::imagenet;
use tch::*;

#[wasm_bindgen]
pub fn input() -> String {

    let image_file = "/Users/chester/PycharmProjects/Gobblet/playground/img/img.png".to_owned();
    let model_file = "/Users/chester/PycharmProjects/Gobblet/wasm_lab/model.pt".to_owned();
    let image = imagenet::load_image_and_resize(image_file,224,224).unwrap();
    let model = tch::CModule::load(model_file).unwrap();
    let output = model.forward_ts(&[image.unsqueeze(0)]).unwrap().softmax(-1,tch::Kind::Float);
    let binding = imagenet::top(&output, 5)
        .iter()
        .map(|(probability, class)| format!("{:50} {:5.2}%", class, 100.0 * probability))
        .collect::<Vec<_>>()
        .join(" ");
    return binding;
}