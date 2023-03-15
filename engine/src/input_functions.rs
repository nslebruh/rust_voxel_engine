use super::{keybinds::InputFunctionArguments, camera::CameraMovement};

pub fn set_window_should_close(args: InputFunctionArguments) {
    args.window.unwrap().set_should_close(true)
}

pub fn camera_forward(args: InputFunctionArguments) {
    args.camera.unwrap().process_action_input(CameraMovement::Forward, args.delta_time.unwrap())
}

pub fn camera_backward(args: InputFunctionArguments) {
    args.camera.unwrap().process_action_input(CameraMovement::Backward, args.delta_time.unwrap())
}

pub fn camera_left(args: InputFunctionArguments) {
    args.camera.unwrap().process_action_input(CameraMovement::Left, args.delta_time.unwrap())
}

pub fn camera_right(args: InputFunctionArguments) {
    args.camera.unwrap().process_action_input(CameraMovement::Right, args.delta_time.unwrap())
}

pub fn camera_up(args: InputFunctionArguments) {
    args.camera.unwrap().process_action_input(CameraMovement::Up, args.delta_time.unwrap())
}

pub fn camera_down(args: InputFunctionArguments) {
    args.camera.unwrap().process_action_input(CameraMovement::Down, args.delta_time.unwrap())
}

pub fn toggle_cursor_mode(args: InputFunctionArguments) {
    
    args.window.unwrap().set_cursor_mode(if args.action.unwrap() == glfw::Action::Press {glfw::CursorMode::Normal} else {glfw::CursorMode::Disabled})
}

pub fn print_camera_pos(args: InputFunctionArguments) {
    if args.action.unwrap() == glfw::Action::Press {
        println!("{:?}", args.camera.unwrap().position)
    }
}

pub fn increase_movement_speed(args: InputFunctionArguments) {
    args.camera.unwrap().movement_speed = if args.action.unwrap() == glfw::Action::Press {10.0} else {5.0};
}

pub fn toggle_fullscreen(args: InputFunctionArguments) {
    if args.action.unwrap() == glfw::Action::Press {
        args.window.unwrap().toggle_fullscreen()
    }
}
