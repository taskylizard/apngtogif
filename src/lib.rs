use image::{
    codecs::{
        gif::{GifEncoder, Repeat},
        png::PngDecoder,
    },
    AnimationDecoder,
};
use pyo3::prelude::*;
use std::io::Cursor;

#[pyfunction]
fn convert(apng_bytes: &[u8], speed: Option<u8>) -> PyResult<Vec<u8>> {
    let cursor = Cursor::new(apng_bytes);

    let decoder = PngDecoder::new(cursor)
        .map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to read APNG: {}", e))
        })?
        .apng()
        .map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to decode APNG: {}", e))
        })?;
    let frames = decoder.into_frames().collect_frames().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to decode APNG: {}", e))
    })?;

    let mut gif_bytes = Vec::new();
    {
        let mut gif_encoder =
            GifEncoder::new_with_speed(&mut gif_bytes, speed.unwrap_or(30).into());
        gif_encoder.set_repeat(Repeat::Infinite).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to set repeat: {}", e))
        })?;

        // Encode each frame of the APNG into the GIF
        for frame in frames {
            gif_encoder.encode_frame(frame).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Failed to encode frame: {}",
                    e
                ))
            })?;
        }
    }

    Ok(gif_bytes)
}

#[pymodule]
fn apngtogif(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(convert, m)?)?;
    Ok(())
}
