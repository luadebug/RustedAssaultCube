#[allow(unused)]
pub(crate) fn set_dark_red_style(ctx: &mut hudhook::imgui::Context) {
    let style = ctx.style_mut();

    style.use_dark_colors();

    style.window_rounding = 0.0;

    style.child_rounding = 0.0;

    style.popup_rounding = 0.0;

    style.frame_rounding = 0.0;

    style.scrollbar_rounding = 0.0;

    style.grab_rounding = 0.0;

    style.tab_rounding = 0.0;

    style.window_border_size = 0.0;

    style.cell_padding = [0.0, 0.0];

    style.window_padding = [0.0, 0.0];

    style.window_title_align = [0.5, 0.5];

    let sytle_colors = &mut ctx.style_mut().colors;

    sytle_colors[hudhook::imgui::sys::ImGuiCol_WindowBg as usize] = [0.1, 0.105, 0.11, 1.0];

    sytle_colors[hudhook::imgui::sys::ImGuiCol_NavHighlight as usize] = [0.3, 0.305, 0.31, 1.0];

    sytle_colors[hudhook::imgui::sys::ImGuiCol_PlotHistogram as usize] = [0.3, 0.305, 0.31, 1.0];

    sytle_colors[hudhook::imgui::sys::ImGuiCol_Header as usize] = [0.2, 0.205, 0.21, 1.0];

    sytle_colors[hudhook::imgui::sys::ImGuiCol_HeaderHovered as usize] = [0.3, 0.305, 0.31, 1.0];

    sytle_colors[hudhook::imgui::sys::ImGuiCol_HeaderActive as usize] = [0.55, 0.5505, 0.551, 1.0];

    sytle_colors[hudhook::imgui::sys::ImGuiCol_Button as usize] = [0.2, 0.205, 0.21, 1.0];

    sytle_colors[hudhook::imgui::sys::ImGuiCol_ButtonHovered as usize] = [0.3, 0.305, 0.31, 1.0];

    sytle_colors[hudhook::imgui::sys::ImGuiCol_ButtonActive as usize] = [0.55, 0.5505, 0.551, 1.0];

    sytle_colors[hudhook::imgui::sys::ImGuiCol_CheckMark as usize] = [0.55, 0.5505, 0.551, 1.0];

    sytle_colors[hudhook::imgui::sys::ImGuiCol_FrameBg as usize] = [0.211, 0.210, 0.25, 1.0];

    sytle_colors[hudhook::imgui::sys::ImGuiCol_FrameBgHovered as usize] = [0.3, 0.305, 0.31, 1.0];

    sytle_colors[hudhook::imgui::sys::ImGuiCol_FrameBgActive as usize] = [0.55, 0.5505, 0.551, 1.0];

    sytle_colors[hudhook::imgui::sys::ImGuiCol_Tab as usize] = [0.25, 0.2505, 0.251, 1.0];

    sytle_colors[hudhook::imgui::sys::ImGuiCol_TabHovered as usize] = [0.38, 0.3805, 0.381, 1.0];

    sytle_colors[hudhook::imgui::sys::ImGuiCol_TabActive as usize] = [0.28, 0.2805, 0.281, 1.0];

    sytle_colors[hudhook::imgui::sys::ImGuiCol_TabUnfocused as usize] = [0.25, 0.2505, 0.251, 1.0];

    sytle_colors[hudhook::imgui::sys::ImGuiCol_TabUnfocusedActive as usize] =
        [0.8, 0.805, 0.81, 1.0];

    sytle_colors[hudhook::imgui::sys::ImGuiCol_ResizeGrip as usize] = [0.2, 0.205, 0.21, 0.0];

    sytle_colors[hudhook::imgui::sys::ImGuiCol_ResizeGripHovered as usize] =
        [0.3, 0.305, 0.31, 1.0];

    sytle_colors[hudhook::imgui::sys::ImGuiCol_ResizeGripActive as usize] =
        [0.55, 0.5505, 0.551, 1.0];

    sytle_colors[hudhook::imgui::sys::ImGuiCol_TitleBg as usize] = [1.0, 0.0, 0.0, 1.0];

    sytle_colors[hudhook::imgui::sys::ImGuiCol_TitleBgActive as usize] = [1.0, 0.0, 0.0, 1.0];

    sytle_colors[hudhook::imgui::sys::ImGuiCol_TitleBgCollapsed as usize] =
        [0.25, 0.2505, 0.251, 1.0];
}


#[allow(unused)]
pub(crate) fn set_dark_style(ctx: &mut hudhook::imgui::Context) {
    let style = ctx.style_mut();

    // Set alignment and padding
    style.window_title_align = [0.5, 0.5]; // Align window title
    style.window_border_size = 0.0; // No window border
    style.frame_padding = [39.0, 6.0]; // Frame padding
    style.window_padding = [10.0, 6.0]; // Window padding
    style.grab_min_size = 24.0; // Minimum size for grab
    style.frame_rounding = 3.0; // Frame rounding
    style.grab_rounding = 4.0; // Grab rounding
    style.item_spacing = [6.0, 6.0]; // Item spacing

    // Set colors
    let colors = &mut style.colors;

    colors[hudhook::imgui::sys::ImGuiCol_Text as usize] = [1.0, 1.0, 1.0, 1.0];
    colors[hudhook::imgui::sys::ImGuiCol_TextDisabled as usize] = [0.5, 0.5, 0.5, 1.0];
    colors[hudhook::imgui::sys::ImGuiCol_WindowBg as usize] = [0.039, 0.039, 0.078, 1.0];
    colors[hudhook::imgui::sys::ImGuiCol_ChildBg as usize] = [1.0, 1.0, 1.0, 0.0];
    colors[hudhook::imgui::sys::ImGuiCol_PopupBg as usize] = [0.039, 0.039, 0.078, 1.0];
    colors[hudhook::imgui::sys::ImGuiCol_Border as usize] = [0.80, 0.80, 0.83, 0.0];
    colors[hudhook::imgui::sys::ImGuiCol_BorderShadow as usize] = [0.92, 0.91, 0.88, 0.0];
    colors[hudhook::imgui::sys::ImGuiCol_FrameBg as usize] = [0.07, 0.07, 0.15, 1.0];
    colors[hudhook::imgui::sys::ImGuiCol_FrameBgHovered as usize] = [0.153, 0.157, 0.227, 1.0];
    colors[hudhook::imgui::sys::ImGuiCol_FrameBgActive as usize] = [0.176, 0.176, 0.247, 1.0];
    colors[hudhook::imgui::sys::ImGuiCol_TitleBg as usize] = [0.07, 0.07, 0.15, 1.0];
    colors[hudhook::imgui::sys::ImGuiCol_TitleBgCollapsed as usize] = [0.07, 0.07, 0.15, 1.0];
    colors[hudhook::imgui::sys::ImGuiCol_TitleBgActive as usize] = [0.07, 0.07, 0.15, 1.0];
    colors[hudhook::imgui::sys::ImGuiCol_MenuBarBg as usize] = [0.10, 0.09, 0.12, 1.0];
    colors[hudhook::imgui::sys::ImGuiCol_ScrollbarBg as usize] = [0.310, 0.310, 0.310, 0.0];
    colors[hudhook::imgui::sys::ImGuiCol_ScrollbarGrab as usize] = [0.310, 0.310, 0.310, 0.31];
    colors[hudhook::imgui::sys::ImGuiCol_ScrollbarGrabHovered as usize] = [0.410, 0.410, 0.410, 1.0];
    colors[hudhook::imgui::sys::ImGuiCol_ScrollbarGrabActive as usize] = [0.510, 0.510, 0.510, 1.0];
    colors[hudhook::imgui::sys::ImGuiCol_CheckMark as usize] = [0.800, 0.557, 0.00, 1.0];
    colors[hudhook::imgui::sys::ImGuiCol_SliderGrab as usize] = [0.800, 0.557, 0.00, 1.0];
    colors[hudhook::imgui::sys::ImGuiCol_SliderGrabActive as usize] = [0.800, 0.557, 0.00, 1.0];
    colors[hudhook::imgui::sys::ImGuiCol_Button as usize] = [0.070, 0.070, 0.130, 1.0];
    colors[hudhook::imgui::sys::ImGuiCol_ButtonHovered as usize] = [0.070, 0.070, 0.130, 1.0];
    colors[hudhook::imgui::sys::ImGuiCol_ButtonActive as usize] = [0.070, 0.070, 0.130, 1.0];
    colors[hudhook::imgui::sys::ImGuiCol_Header as usize] = [0.039, 0.039, 0.078, 1.0];
    colors[hudhook::imgui::sys::ImGuiCol_HeaderHovered as usize] = [0.070, 0.070, 0.130, 1.0];
    colors[hudhook::imgui::sys::ImGuiCol_HeaderActive as usize] = [0.070, 0.070, 0.130, 1.0];
    colors[hudhook::imgui::sys::ImGuiCol_Separator as usize] = [0.039, 0.039, 0.078, 0.0];
    colors[hudhook::imgui::sys::ImGuiCol_SeparatorHovered as usize] = [0.039, 0.039, 0.078, 0.0];
    colors[hudhook::imgui::sys::ImGuiCol_SeparatorActive as usize] = [0.039, 0.039, 0.078, 0.0];
    colors[hudhook::imgui::sys::ImGuiCol_ResizeGrip as usize] = [0.00, 0.00, 0.00, 0.0];
    colors[hudhook::imgui::sys::ImGuiCol_ResizeGripHovered as usize] = [0.56, 0.56, 0.58, 0.0];
    colors[hudhook::imgui::sys::ImGuiCol_ResizeGripActive as usize] = [0.06, 0.05, 0.07, 0.0];
    colors[hudhook::imgui::sys::ImGuiCol_Tab as usize] = [0.039, 0.039, 0.078, 1.0];
    colors[hudhook::imgui::sys::ImGuiCol_TabHovered as usize] = [0.054, 0.054, 0.104, 1.0];
    colors[hudhook::imgui::sys::ImGuiCol_TabActive as usize] = [0.070, 0.070, 0.130, 1.0];
}