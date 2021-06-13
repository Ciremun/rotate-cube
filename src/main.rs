use std::{convert::TryInto, ffi::*, mem};

use gl::types::*;
use glfw::{Action, Context, Key, OpenGlProfileHint, WindowHint};
use glfw::ffi::*;

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

    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::ContextVersionMajor(3));
    glfw.window_hint(WindowHint::ContextVersionMinor(2));

    let (mut window, events) = glfw
        .create_window(1024, 768, "uwu window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    unsafe {
        glfwSetInputMode(window.window_ptr(), CURSOR, CURSOR_HIDDEN);
    }

    gl::load_with(|s| window.get_proc_address(s) as *const _);
    gl::Viewport::load_with(|s| window.get_proc_address(s) as *const _);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LESS);
    }

    unsafe {
        let mut vertex_array_id = 0;
        gl::GenVertexArrays(1, &mut vertex_array_id);
        gl::BindVertexArray(vertex_array_id);
    }

    let vertex_buffer_data = vec![
        -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0,
        1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0,
        -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0,
        -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0,
        1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0,
        1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, -1.0, 1.0,
    ];

    let color_buffer_data = vec![
        0.583, 0.771, 0.014, 0.609, 0.115, 0.436, 0.327, 0.483, 0.844, 0.822, 0.569, 0.201, 0.435,
        0.602, 0.223, 0.310, 0.747, 0.185, 0.597, 0.770, 0.761, 0.559, 0.436, 0.730, 0.359, 0.583,
        0.152, 0.483, 0.596, 0.789, 0.559, 0.861, 0.639, 0.195, 0.548, 0.859, 0.014, 0.184, 0.576,
        0.771, 0.328, 0.970, 0.406, 0.615, 0.116, 0.676, 0.977, 0.133, 0.971, 0.572, 0.833, 0.140,
        0.616, 0.489, 0.997, 0.513, 0.064, 0.945, 0.719, 0.592, 0.543, 0.021, 0.978, 0.279, 0.317,
        0.505, 0.167, 0.620, 0.077, 0.347, 0.857, 0.137, 0.055, 0.953, 0.042, 0.714, 0.505, 0.345,
        0.783, 0.290, 0.734, 0.722, 0.645, 0.174, 0.302, 0.455, 0.848, 0.225, 0.587, 0.040, 0.517,
        0.713, 0.338, 0.053, 0.959, 0.120, 0.393, 0.621, 0.362, 0.673, 0.211, 0.457, 0.820, 0.883,
        0.371, 0.982, 0.099, 0.879,
    ];

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

    let mut color_buffer = 0;
    unsafe {
        gl::GenBuffers(1, &mut color_buffer);
        gl::BindBuffer(gl::ARRAY_BUFFER, color_buffer);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (mem::size_of::<f32>() * color_buffer_data.len()) as isize,
            color_buffer_data.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );
    }

    let program_id: GLuint;
    unsafe {
        program_id = load_shaders();
        gl::UseProgram(program_id);
    }

    let projection = glm::ext::perspective(glm::radians(45.0), (1024 / 768) as f32, 0.1, 100.0);
    let view = glm::ext::look_at(
        glm::vec3(4.0, 3.0, 3.0),
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(0.0, 1.0, 0.0),
    );
    let model = glm::mat4(
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );
    let mvp = projection * view * model;

    let matrix_id: GLint;
    unsafe {
        let u_name = CStr::from_bytes_with_nul_unchecked("MVP\0".as_bytes()).as_ptr();
        matrix_id = gl::GetUniformLocation(program_id, u_name);
    }

    let mut position = glm::vec3(0.0, 0.0, 5.0);
    let mut horizontal_angle = 3.14;
    let mut vertical_angle = 0.0;
    let initial_field_of_view = 45.0;

    let speed = 3.0;
    let mouse_speed = 0.05;

    let mut delta_time: f32;
    let mut last_frame = 0.0;

    while !window.should_close() {
        let current_frame = glfw.get_time();
        delta_time = (current_frame - last_frame) as f32;
        last_frame = current_frame;
        window.swap_buffers();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
        let (xpos, ypos) = window.get_cursor_pos();
        horizontal_angle += mouse_speed * delta_time * (1024.0 / 2.0 - xpos) as f32;
        vertical_angle += mouse_speed * delta_time * (768.0 / 2.0 - ypos) as f32;
        let direction = glm::vec3(
            (vertical_angle.cos() * horizontal_angle.sin()) as f32,
            vertical_angle.sin() as f32,
            (vertical_angle.cos() * horizontal_angle.cos()) as f32,
        );
        let right = glm::vec3(
            (horizontal_angle - 3.14 / 2.0).sin() as f32,
            0.0,
            (horizontal_angle - 3.14 / 2.0).cos() as f32
        );
        let up = glm::cross(right, direction);
        // Move forward
        if window.get_key(glfw::Key::W) == glfw::Action::Press {
            position = position + direction * delta_time * speed;
        }
        // Move backward
        if window.get_key(glfw::Key::S) == glfw::Action::Press {
            position = position - direction * delta_time * speed;
        }
        // Strafe right
        if window.get_key(glfw::Key::D) == glfw::Action::Press {
            position = position + right * delta_time * speed;
        }
        // Strafe left
        if window.get_key(glfw::Key::A) == glfw::Action::Press {
            position = position - right * delta_time * speed;
        }
        let projection_matrix = glm::ext::perspective(glm::radians(initial_field_of_view), 4.0 / 3.0, 0.1, 100.0);
        let view_matrix = glm::ext::look_at(
            position,           // Camera is here
            position+direction, // and looks here : at the same position, plus "direction"
            up                  // Head is up (set to 0,-1,0 to look upside-down)
        );
        let new_mvp = projection_matrix * view_matrix * model;
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            
            gl::UniformMatrix4fv(matrix_id, 1, gl::FALSE, &new_mvp[0][0]);
            
            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, 0 as *const c_void);
            gl::DrawArrays(gl::TRIANGLES, 0, 12 * 3);
            gl::DisableVertexAttribArray(0);
            
            gl::EnableVertexAttribArray(1);
            gl::BindBuffer(gl::ARRAY_BUFFER, color_buffer);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 0, 0 as *const c_void);
            gl::DrawArrays(gl::TRIANGLES, 0, 12 * 3);
        }
        window.set_cursor_pos(1024.0 / 2.0, 768.0 / 2.0);
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
