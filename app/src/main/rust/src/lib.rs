#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use self::jni::objects::{JClass, JObject, JString};
    use self::jni::sys::jstring;
    use self::jni::JNIEnv;
    use std::os::raw::c_void;
    mod graphic;
    mod julia;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_arnabanimesh_demorsndk_MainActivity_imgCaptionRs(
        env: JNIEnv,
        _: JClass,
        input: JString,
    ) -> jstring {
        let input: String = env
            .get_string(input)
            .expect("Couldn't create java string!")
            .into();
        let output = env
            .new_string(format!("{} in 🦀", input))
            .expect("Couldn't create java string!");
        output.into_inner()
    }
    #[no_mangle]
    pub unsafe extern "C" fn Java_com_arnabanimesh_demorsndk_MainActivity_renderJulia(
        env: JNIEnv,
        _: JClass,
        bmp: JObject,
    ) {
        let mut info = graphic::AndroidBitmapInfo::new();
        let raw_env = env.get_native_interface();

        let bmp = bmp.into_inner();

        // Read bitmap info
        graphic::bitmap_get_info(raw_env, bmp, &mut info);
        let mut pixels = 0 as *mut c_void;

        // Lock pixel for draw
        graphic::bitmap_lock_pixels(raw_env, bmp, &mut pixels);

        let pixels =
            std::slice::from_raw_parts_mut(pixels as *mut u8, (info.stride * info.height) as usize);

        julia::render(pixels, info.width as u32, info.height as u32);
        graphic::bitmap_unlock_pixels(raw_env, bmp);
    }
}
