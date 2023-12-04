
//
// pub fn ui_event_handler<R, V, T>(
//     mut ui_event: EventReader<UiEvent<R, V>>,
//     window: Res<GeneratorWindow>,
//     mut asset: ResMut<Assets<Image>>,
// )
//     where R: 'static + RgbaData + Send + Sync,
//           V: 'static + Viewport + Send + Sync,
// {
//     for e in ui_event.iter() {
//         match &e.0 {
//             GeneratorOutputMessage::Loading(is_loading) => {}
//             GeneratorOutputMessage::Image(image) => {
//                 let image: Vec<_> = image
//                     .into_iter()
//                     .flat_map(|row| row
//                         .into_iter()
//                         .flat_map(|pixel| pixel.data().map(|v| v)))
//                     .collect();
//                 let image = RgbaImage::from_raw(window.width as u32, window.height as u32, image)
//                     .unwrap();
//                 let image = image::DynamicImage::from(image);
//                 let handle = asset.add(Image::from_dynamic(image, false));
//             }
//             GeneratorOutputMessage::Viewport(viewport) => {}
//         }
//     }
// }