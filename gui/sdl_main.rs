#![crate_type = "bin"]

#![allow(dead_code)]
#![feature(slicing_syntax)]
#![feature(unboxed_closures)]
#![feature(phase)]
#![feature(globs)]

extern crate collections;
extern crate regex;
extern crate serialize;
extern crate curl;
extern crate core;
extern crate sdl2;
extern crate sdl2_image;
extern crate libc;

#[phase(plugin, link)]
extern crate gl3;

use libc::c_void;
use gl3::types::*;

fn sdl_main(file_name : &str) {
    sdl2::init(sdl2::INIT_VIDEO);
    sdl2_image::init(sdl2_image::INIT_PNG | sdl2_image::INIT_JPG);

    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextMajorVersion, 
                                  gl3::platform::GL_MAJOR_VERSION);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextMinorVersion, 
                                  gl3::platform::GL_MINOR_VERSION);   
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLDepthSize, 24);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLDoubleBuffer, 1);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextProfileMask, 0x0001);
    //sdl2::video::ll::SDL_GL_CONTEXT_PROFILE_CORE as int);

    let window = match sdl2::video::Window::new("mtg-rust", 
                                                sdl2::video::WindowPos::PosCentered, 
                                                sdl2::video::WindowPos::PosCentered, 
                                                0, 0, 
                                                sdl2::video::OPENGL |
                                                sdl2::video::FULLSCREEN_DESKTOP) {
        Ok(window) => window,
        Err(err) => panic!(format!("failed to create window: {}", err))
    };

    let context = match window.gl_create_context() {
        Ok(context) => context,
        Err(err) => panic!(format!("failed to create context: {}", err))
    };

    sdl2::clear_error();

    gl3::load_with(|name| {
        match sdl2::video::gl_get_proc_address(name) {
            Some(glproc) => glproc as *const libc::c_void,
            None => {
                println!("missing GL function: {}", name);
                std::ptr::null()
            }
        }
    });

    if !sdl2::video::gl_set_swap_interval(1) {
        panic!(format!("failed to set swap interval: {}", sdl2::get_error()))
    }

/*
    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED | sdl2::render::PRESENTVSYNC) {
        Ok(renderer) => renderer,
        Err(err) => panic!(format!("failed to create renderer: {}", err))
    };
*/

    let jpg = &Path::new(file_name
        //"pics/JOU-keranos, god of storms.jpg"
        //"pics/THS-swamp4.jpg"
        //"pics/KTK-sarkhan, the dragonspeaker.jpg" 
        /*"pics/THS-master of waves.jpg"*/);
    let mut surface = match sdl2_image::LoadSurface::from_file(jpg) {
        Ok(surface) => surface,
        Err(err) => panic!(format!("Failed to load png: {}", err))
    };

    let mut texture = 0;
    
    check_gl_unsafe!(gl3::GenTextures(1, &mut texture));

    check_gl_unsafe!(gl3::BindTexture(gl3::TEXTURE_2D, texture));
    {
        let w = surface.get_width() as GLsizei;
        let h = surface.get_height() as GLsizei;
        //let num_mips = 8u;
        surface.with_lock(|pixels : &mut [u8]| -> () {
            //check_gl_unsafe!(gl3::TexStorage2D(gl3::TEXTURE_2D, num_mips, gl3::RGBA8, w, h));
            //check_gl_unsafe!(gl3::TexSubImage2D(gl3::TEXTURE_2D, 0, 0, 0, w, h, gl3::BGRA, gl3::UNSIGNED_BYTE, pixels.as_ptr() as *const libc::c_void));
            //check_gl_unsafe!(gl3::GenerateMipmap(gl3::TEXTURE_2D));
            check_gl_unsafe!(gl3::TexImage2D(gl3::TEXTURE_2D, 0, 
                                            gl3::RGBA as GLint, 
                                            w, h,                                         
                                            0, gl3::RGB, gl3::UNSIGNED_BYTE, 
                                            pixels.as_ptr() as *const libc::c_void
                                            ));
            check_gl_unsafe!(gl3::GenerateMipmap(gl3::TEXTURE_2D));
        });
    }
    check_gl_unsafe!(gl3::TexParameteri(gl3::TEXTURE_2D, gl3::TEXTURE_MIN_FILTER, gl3::LINEAR_MIPMAP_LINEAR as GLint));
    check_gl_unsafe!(gl3::TexParameteri(gl3::TEXTURE_2D, gl3::TEXTURE_MAG_FILTER, gl3::LINEAR as GLint));

/*
    let texture = match renderer.create_texture_from_surface(&surface) {
        Ok(texture) => texture,
        Err(err) => panic!(format!("Failed to create surface: {}", err))
    };
*/

    let surf_rect = surface.get_rect();

    let mut speed = -0.005f32;
    let mut sc = 1.00f32;
    //let mut timer = sdl2::timer::get_ticks();

    'main : loop {
        'event : loop {
            match sdl2::event::poll_event() {
                sdl2::event::Event::Quit(_) => break 'main,
                sdl2::event::Event::KeyDown(_, _, key, _, _, _) => {
                    if key == sdl2::keycode::KeyCode::Escape {
                        break 'main
                    }
                },
                sdl2::event::Event::None => break 'event,
                _ => {}
            };
            //sdl2::timer::delay(1u)
        }

        sc += speed;
        if sc > 3.0 { speed = -speed }
        if sc < 0.0 { speed = -speed }        

        check_gl_unsafe!(gl3::MatrixMode(gl3::PROJECTION));
        check_gl_unsafe!(gl3::LoadIdentity());
        check_gl_unsafe!(gl3::MatrixMode(gl3::MODELVIEW));
        check_gl_unsafe!(gl3::LoadIdentity());
        check_gl_unsafe!(gl3::ClearColor(0.0, 0.25, 0.25, 1.0));
        check_gl_unsafe!(gl3::Clear(gl3::COLOR_BUFFER_BIT));

        let (ww, wh) = window.get_size();
        let (tw, th) = (surf_rect.w, surf_rect.h);

        let w = (tw as f32 / ww as f32) * sc;
        let h = (th as f32 / wh as f32) * sc;

        
        unsafe {
            // 56mm x 81mm
            let w = w * 1.025;
            let h = h * 1.025;
            gl3::Color3f(0.0f32, 0.0f32, 0.0f32);

            gl3::Begin(gl3::QUADS);
            gl3::Vertex3f( -w, -h, 0.0f32);
            gl3::Vertex3f(  w, -h, 0.0f32);
            gl3::Vertex3f(  w,  h, 0.0f32);
            gl3::Vertex3f( -w,  h, 0.0f32);
            gl3::End();
        }
        

        check_gl_unsafe!(gl3::Enable(gl3::TEXTURE_2D));

        unsafe {
            gl3::Color3f(1.0f32, 1.0f32, 1.0f32);

            gl3::Begin(gl3::QUADS);
            gl3::TexCoord2f(0.0f32, 1.0f32);
            gl3::Vertex3f( -w, -h, 0.0f32);

            gl3::TexCoord2f(1.0f32, 1.0f32);
            gl3::Vertex3f(  w, -h, 0.0f32);

            gl3::TexCoord2f(1.0f32, 0.0f32);
            gl3::Vertex3f(  w,  h, 0.0f32);

            gl3::TexCoord2f(0.0f32, 0.0f32);
            gl3::Vertex3f( -w,  h, 0.0f32);

            gl3::End();
        }
        check_gl_unsafe!(gl3::Disable(gl3::TEXTURE_2D));

        window.gl_swap_window();

        //let _ = renderer.copy(&texture, None, Some(rect));
        //renderer.present();
    }

    sdl2_image::quit();
    sdl2::quit();    
}

#[main]
fn main() {
    let args = std::os::args();

    if args.len() == 2 {
        sdl_main(args[1].as_slice())
    }
}
