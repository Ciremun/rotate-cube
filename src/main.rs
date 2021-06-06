use std::{convert::TryInto, ffi::*, mem};

use gl::types::*;
use glfw::{Action, Context, Key};

unsafe fn compile_shader(shader_id: GLuint, shader_c_string: &CStr) {
    let mut compiled = 0;

    gl::ShaderSource(shader_id, 1, &shader_c_string.as_ptr(), std::ptr::null());
    gl::CompileShader(shader_id);

    gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut compiled);

    if compiled != gl::TRUE.into() {
        let mut buffer: [u8; 1024] = [0; 1024];
        let mut length: GLsizei = 0;
        gl::GetShaderInfoLog(
            shader_id,
            buffer.len().try_into().unwrap(),
            &mut length,
            buffer.as_mut_ptr() as *mut i8,
        );
        panic!(
            "Could not compile shader: {}",
            std::str::from_utf8(&buffer[0..length as usize]).unwrap()
        );
    }
}

unsafe fn load_shaders() -> GLuint {
    let vertex_shader_bytes = include_bytes!("shaders/vertex.glsl");
    let fragment_shader_bytes = include_bytes!("shaders/fragment.glsl");

    let mut vertex_shader_string = String::from_utf8_lossy(vertex_shader_bytes);
    let mut fragment_shader_string = String::from_utf8_lossy(fragment_shader_bytes);

    let vertex_shader_id = gl::CreateShader(gl::VERTEX_SHADER);
    let fragment_shader_id = gl::CreateShader(gl::FRAGMENT_SHADER);

    vertex_shader_string.to_mut().push('\0');
    let vertex_shader_c_str = CStr::from_bytes_with_nul_unchecked(vertex_shader_string.as_bytes());
    fragment_shader_string.to_mut().push('\0');
    let fragment_shader_c_str =
        CStr::from_bytes_with_nul_unchecked(fragment_shader_string.as_bytes());

    compile_shader(vertex_shader_id, vertex_shader_c_str);
    compile_shader(fragment_shader_id, fragment_shader_c_str);

    let program_id = gl::CreateProgram();
    gl::AttachShader(program_id, vertex_shader_id);
    gl::AttachShader(program_id, fragment_shader_id);
    gl::LinkProgram(program_id);

    let mut linked = 0;
    gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut linked);

    if linked != gl::TRUE.into() {
        let mut buffer: [u8; 1024] = [0; 1024];
        let mut length: GLsizei = 0;
        gl::GetProgramInfoLog(
            program_id,
            buffer.len().try_into().unwrap(),
            &mut length,
            buffer.as_mut_ptr() as *mut i8,
        );
        panic!(
            "Could not link shader: {}",
            std::str::from_utf8(&buffer[0..length as usize]).unwrap()
        );
    }

    gl::DetachShader(program_id, vertex_shader_id);
    gl::DetachShader(program_id, fragment_shader_id);

    gl::DeleteShader(vertex_shader_id);
    gl::DeleteShader(fragment_shader_id);

    return program_id;
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(1920, 1080, "uwu window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    gl::load_with(|s| window.get_proc_address(s) as *const _);
    gl::Viewport::load_with(|s| window.get_proc_address(s) as *const _);

    unsafe {
        let mut vertex_array_id = 0;
        gl::GenVertexArrays(1, &mut vertex_array_id);
        gl::BindVertexArray(vertex_array_id);
    }

    let vertex_buffer_data = vec![-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0];

    let mut vertex_buffer = 0;
    unsafe {
        gl::GenBuffers(1, &mut vertex_buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (mem::size_of::<f32>() * vertex_buffer_data.len()) as isize,
            vertex_buffer_data.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );
    }

    let program_id: GLuint;
    unsafe {
        program_id = load_shaders();
        gl::UseProgram(program_id);
    }

    let projection = glm::ext::perspective(glm::radians(90.0), (1920 / 1080) as f32, 0.1, 100.0);
    let view = glm::ext::look_at(
        glm::vec3(4.0, 3.0, 3.0),
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(0.0, 1.0, 0.0)
    );
    let model = glm::mat4(
        1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0
    );
    let mvp = projection * view;

    let matrix_id: GLint;
    unsafe {
        let u_name = CStr::from_bytes_with_nul_unchecked("MVP\0".as_bytes()).as_ptr();
        matrix_id = gl::GetUniformLocation(program_id, u_name);
    }

    while !window.should_close() {
        window.swap_buffers();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::UniformMatrix4fv(matrix_id, 1, gl::FALSE, &mvp[0][0]);

            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, 0 as *const c_void);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::DisableVertexAttribArray(0);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
