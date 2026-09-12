use pipewire as pw;
use pw::properties::properties;
use pw::spa;
use spa::param::format::{MediaSubtype, MediaType};
use spa::param::format_utils;
use spa::pod::Pod;
use std::convert::TryInto;
use std::mem;
use std::sync::{Arc, Mutex};

pub struct UserData<F> {
    pub format: spa::param::audio::AudioInfoRaw,
    pub callback: F,
}

pub fn start_audio_capture<F>(callback: F, stop_flag: Arc<Mutex<bool>>) -> Result<(), anyhow::Error>
where
    F: Fn(&[f32]) + 'static,
{
    pw::init();

    let mainloop = pw::main_loop::MainLoopRc::new(None)?;
    let context = pw::context::ContextRc::new(&mainloop, None)?;
    let core = context.connect_rc(None)?;

    let data = UserData {
        format: Default::default(),
        callback,
    };

    let props = properties! {
        *pw::keys::MEDIA_TYPE => "Audio",
        *pw::keys::MEDIA_CATEGORY => "Capture",
        *pw::keys::MEDIA_ROLE => "Music",
    };

    let stream = pw::stream::StreamBox::new(&core, "audioshare-capture", props)?;

    let _listener = stream
        .add_local_listener_with_user_data(data)
        .param_changed(|_, user_data, id, param| {
            let Some(param) = param else {
                return;
            };
            if id != pw::spa::param::ParamType::Format.as_raw() {
                return;
            }

            let (media_type, media_subtype) = match format_utils::parse_format(param) {
                Ok(v) => v,
                Err(_) => return,
            };

            if media_type != MediaType::Audio || media_subtype != MediaSubtype::Raw {
                return;
            }

            user_data.format.parse(param).unwrap();
        })
        .process(|stream, user_data| match stream.dequeue_buffer() {
            None => {}
            Some(mut buffer) => {
                let datas = buffer.datas_mut();
                if datas.is_empty() {
                    return;
                }

                let data = &mut datas[0];
                let n_channels = user_data.format.channels();
                let n_samples = data.chunk().size() / (mem::size_of::<f32>() as u32);

                if let Some(samples) = data.data() {
                    // Extract f32 samples
                    let mut float_samples = Vec::with_capacity(n_samples as usize);
                    for n in (0..n_samples).step_by(1) {
                        let start = n as usize * mem::size_of::<f32>();
                        let end = start + mem::size_of::<f32>();
                        let chan = &samples[start..end];
                        let f = f32::from_le_bytes(chan.try_into().unwrap());
                        float_samples.push(f);
                    }
                    (user_data.callback)(&float_samples);
                }
            }
        })
        .register()?;

    let mut audio_info = spa::param::audio::AudioInfoRaw::new();
    audio_info.set_format(spa::param::audio::AudioFormat::F32LE);
    let obj = pw::spa::pod::Object {
        type_: pw::spa::utils::SpaTypes::ObjectParamFormat.as_raw(),
        id: pw::spa::param::ParamType::EnumFormat.as_raw(),
        properties: audio_info.into(),
    };
    let values: Vec<u8> = pw::spa::pod::serialize::PodSerializer::serialize(
        std::io::Cursor::new(Vec::new()),
        &pw::spa::pod::Value::Object(obj),
    )
    .unwrap()
    .0
    .into_inner();

    let mut params = [Pod::from_bytes(&values).unwrap()];

    stream.connect(
        spa::utils::Direction::Input,
        None,
        pw::stream::StreamFlags::AUTOCONNECT
            | pw::stream::StreamFlags::MAP_BUFFERS
            | pw::stream::StreamFlags::RT_PROCESS,
        &mut params,
    )?;

    // Use a PipeWire timer to check the stop flag instead of spawning a thread
    let ml_clone = mainloop.clone();
    let timer = mainloop.loop_().add_timer(move |_| {
        if *stop_flag.lock().unwrap() {
            ml_clone.quit();
        }
    });
    timer.update_timer(
        Some(std::time::Duration::from_millis(100)),
        Some(std::time::Duration::from_millis(100)),
    )
    .into_result()?;


    mainloop.run();

    unsafe {
        pw::deinit();
    }

    Ok(())
}
