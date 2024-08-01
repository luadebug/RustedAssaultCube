#[allow(unused)]
pub(crate) fn set_style_unicore(ctx: &mut hudhook::imgui::Context) {
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
    use hudhook::imgui::sys::{
        ImGuiCol_Border, ImGuiCol_BorderShadow, ImGuiCol_Button, ImGuiCol_ButtonActive,
        ImGuiCol_ButtonHovered, ImGuiCol_CheckMark, ImGuiCol_ChildBg, ImGuiCol_FrameBg,
        ImGuiCol_FrameBgActive, ImGuiCol_FrameBgHovered, ImGuiCol_Header, ImGuiCol_HeaderActive,
        ImGuiCol_HeaderHovered, ImGuiCol_MenuBarBg, ImGuiCol_PopupBg, ImGuiCol_ResizeGrip,
        ImGuiCol_ResizeGripActive, ImGuiCol_ResizeGripHovered, ImGuiCol_ScrollbarBg,
        ImGuiCol_ScrollbarGrab, ImGuiCol_ScrollbarGrabActive, ImGuiCol_ScrollbarGrabHovered,
        ImGuiCol_Separator, ImGuiCol_SeparatorActive, ImGuiCol_SeparatorHovered,
        ImGuiCol_SliderGrab, ImGuiCol_SliderGrabActive, ImGuiCol_Tab, ImGuiCol_TabActive,
        ImGuiCol_TabHovered, ImGuiCol_Text, ImGuiCol_TextDisabled, ImGuiCol_TitleBg,
        ImGuiCol_TitleBgActive, ImGuiCol_TitleBgCollapsed, ImGuiCol_WindowBg,
    };
    let colors = &mut style.colors;

    colors[ImGuiCol_Text as usize] = [1.0, 1.0, 1.0, 1.0];
    colors[ImGuiCol_TextDisabled as usize] = [0.5, 0.5, 0.5, 1.0];
    colors[ImGuiCol_WindowBg as usize] = [0.039, 0.039, 0.078, 1.0];
    colors[ImGuiCol_ChildBg as usize] = [1.0, 1.0, 1.0, 0.0];
    colors[ImGuiCol_PopupBg as usize] = [0.039, 0.039, 0.078, 1.0];
    colors[ImGuiCol_Border as usize] = [0.80, 0.80, 0.83, 0.0];
    colors[ImGuiCol_BorderShadow as usize] = [0.92, 0.91, 0.88, 0.0];
    colors[ImGuiCol_FrameBg as usize] = [0.07, 0.07, 0.15, 1.0];
    colors[ImGuiCol_FrameBgHovered as usize] = [0.153, 0.157, 0.227, 1.0];
    colors[ImGuiCol_FrameBgActive as usize] = [0.176, 0.176, 0.247, 1.0];
    colors[ImGuiCol_TitleBg as usize] = [0.07, 0.07, 0.15, 1.0];
    colors[ImGuiCol_TitleBgCollapsed as usize] = [0.07, 0.07, 0.15, 1.0];
    colors[ImGuiCol_TitleBgActive as usize] = [0.07, 0.07, 0.15, 1.0];
    colors[ImGuiCol_MenuBarBg as usize] = [0.10, 0.09, 0.12, 1.0];
    colors[ImGuiCol_ScrollbarBg as usize] = [0.310, 0.310, 0.310, 0.0];
    colors[ImGuiCol_ScrollbarGrab as usize] = [0.310, 0.310, 0.310, 0.31];
    colors[ImGuiCol_ScrollbarGrabHovered as usize] = [0.410, 0.410, 0.410, 1.0];
    colors[ImGuiCol_ScrollbarGrabActive as usize] = [0.510, 0.510, 0.510, 1.0];
    colors[ImGuiCol_CheckMark as usize] = [0.800, 0.557, 0.00, 1.0];
    colors[ImGuiCol_SliderGrab as usize] = [0.800, 0.557, 0.00, 1.0];
    colors[ImGuiCol_SliderGrabActive as usize] = [0.800, 0.557, 0.00, 1.0];
    colors[ImGuiCol_Button as usize] = [0.070, 0.070, 0.130, 1.0];
    colors[ImGuiCol_ButtonHovered as usize] = [0.070, 0.070, 0.130, 1.0];
    colors[ImGuiCol_ButtonActive as usize] = [0.070, 0.070, 0.130, 1.0];
    colors[ImGuiCol_Header as usize] = [0.039, 0.039, 0.078, 1.0];
    colors[ImGuiCol_HeaderHovered as usize] = [0.070, 0.070, 0.130, 1.0];
    colors[ImGuiCol_HeaderActive as usize] = [0.070, 0.070, 0.130, 1.0];
    colors[ImGuiCol_Separator as usize] = [0.039, 0.039, 0.078, 0.0];
    colors[ImGuiCol_SeparatorHovered as usize] = [0.039, 0.039, 0.078, 0.0];
    colors[ImGuiCol_SeparatorActive as usize] = [0.039, 0.039, 0.078, 0.0];
    colors[ImGuiCol_ResizeGrip as usize] = [0.00, 0.00, 0.00, 0.0];
    colors[ImGuiCol_ResizeGripHovered as usize] = [0.56, 0.56, 0.58, 0.0];
    colors[ImGuiCol_ResizeGripActive as usize] = [0.06, 0.05, 0.07, 0.0];
    colors[ImGuiCol_Tab as usize] = [0.039, 0.039, 0.078, 1.0];
    colors[ImGuiCol_TabHovered as usize] = [0.054, 0.054, 0.104, 1.0];
    colors[ImGuiCol_TabActive as usize] = [0.070, 0.070, 0.130, 1.0];
}

#[allow(unused)]
pub(crate) fn set_style_minty_red(ctx: &mut hudhook::imgui::Context) {
    let style = ctx.style_mut();
    let colors = &mut style.colors;

    // General styling
    style.window_title_align = [0.5, 0.5];
    style.window_border_size = 0.0;
    style.frame_padding = [39.0, 6.0];
    style.window_padding = [10.0, 6.0];
    style.grab_min_size = 24.0;
    style.frame_rounding = 3.0;
    style.grab_rounding = 4.0;
    style.item_spacing = [6.0, 6.0];

    // Set colors
    use hudhook::imgui::sys::{
        ImGuiCol_Border, ImGuiCol_BorderShadow, ImGuiCol_Button, ImGuiCol_ButtonActive,
        ImGuiCol_ButtonHovered, ImGuiCol_CheckMark, ImGuiCol_ChildBg, ImGuiCol_FrameBg,
        ImGuiCol_FrameBgActive, ImGuiCol_FrameBgHovered, ImGuiCol_Header, ImGuiCol_HeaderActive,
        ImGuiCol_HeaderHovered, ImGuiCol_MenuBarBg, ImGuiCol_PopupBg, ImGuiCol_ResizeGrip,
        ImGuiCol_ResizeGripActive, ImGuiCol_ResizeGripHovered, ImGuiCol_ScrollbarBg,
        ImGuiCol_ScrollbarGrab, ImGuiCol_ScrollbarGrabActive, ImGuiCol_ScrollbarGrabHovered,
        ImGuiCol_Separator, ImGuiCol_SeparatorActive, ImGuiCol_SeparatorHovered,
        ImGuiCol_SliderGrab, ImGuiCol_SliderGrabActive, ImGuiCol_Tab, ImGuiCol_TabActive,
        ImGuiCol_TabHovered, ImGuiCol_Text, ImGuiCol_TextDisabled, ImGuiCol_TitleBg,
        ImGuiCol_TitleBgActive, ImGuiCol_TitleBgCollapsed, ImGuiCol_WindowBg,
        ImGuiCol_DragDropTarget, ImGuiCol_ModalWindowDimBg, ImGuiCol_NavHighlight,
        ImGuiCol_NavWindowingDimBg, ImGuiCol_NavWindowingHighlight, ImGuiCol_PlotHistogram,
        ImGuiCol_PlotHistogramHovered, ImGuiCol_PlotLines, ImGuiCol_PlotLinesHovered,
        ImGuiCol_TabUnfocused, ImGuiCol_TabUnfocusedActive, ImGuiCol_TableBorderLight,
        ImGuiCol_TableBorderStrong, ImGuiCol_TableHeaderBg, ImGuiCol_TableRowBg,
        ImGuiCol_TableRowBgAlt, ImGuiCol_TextSelectedBg
    };

    colors[ImGuiCol_Text as usize] = [0.75, 0.75, 0.75, 1.00];
    colors[ImGuiCol_TextDisabled as usize] = [0.35, 0.35, 0.35, 1.00];
    colors[ImGuiCol_WindowBg as usize] = [0.00, 0.00, 0.00, 0.94];
    colors[ImGuiCol_ChildBg as usize] = [0.00, 0.00, 0.00, 0.00];
    colors[ImGuiCol_PopupBg as usize] = [0.08, 0.08, 0.08, 0.94];
    colors[ImGuiCol_Border as usize] = [0.00, 0.00, 0.00, 0.50];
    colors[ImGuiCol_BorderShadow as usize] = [0.00, 0.00, 0.00, 0.00];
    colors[ImGuiCol_FrameBg as usize] = [0.00, 0.00, 0.00, 0.54];
    colors[ImGuiCol_FrameBgHovered as usize] = [0.37, 0.14, 0.14, 0.67];
    colors[ImGuiCol_FrameBgActive as usize] = [0.39, 0.20, 0.20, 0.67];
    colors[ImGuiCol_TitleBg as usize] = [0.04, 0.04, 0.04, 1.00];
    colors[ImGuiCol_TitleBgActive as usize] = [0.48, 0.16, 0.16, 1.00];
    colors[ImGuiCol_TitleBgCollapsed as usize] = [0.48, 0.16, 0.16, 1.00];
    colors[ImGuiCol_MenuBarBg as usize] = [0.14, 0.14, 0.14, 1.00];
    colors[ImGuiCol_ScrollbarBg as usize] = [0.02, 0.02, 0.02, 0.53];
    colors[ImGuiCol_ScrollbarGrab as usize] = [0.31, 0.31, 0.31, 1.00];
    colors[ImGuiCol_ScrollbarGrabHovered as usize] = [0.41, 0.41, 0.41, 1.00];
    colors[ImGuiCol_ScrollbarGrabActive as usize] = [0.51, 0.51, 0.51, 1.00];
    colors[ImGuiCol_CheckMark as usize] = [0.56, 0.10, 0.10, 1.00];
    colors[ImGuiCol_SliderGrab as usize] = [1.00, 0.19, 0.19, 0.40];
    colors[ImGuiCol_SliderGrabActive as usize] = [0.89, 0.00, 0.19, 1.00];
    colors[ImGuiCol_Button as usize] = [1.00, 0.19, 0.19, 0.40];
    colors[ImGuiCol_ButtonHovered as usize] = [0.80, 0.17, 0.00, 1.00];
    colors[ImGuiCol_ButtonActive as usize] = [0.89, 0.00, 0.19, 1.00];
    colors[ImGuiCol_Header as usize] = [0.33, 0.35, 0.36, 0.53];
    colors[ImGuiCol_HeaderHovered as usize] = [0.76, 0.28, 0.44, 0.67];
    colors[ImGuiCol_HeaderActive as usize] = [0.47, 0.47, 0.47, 0.67];
    colors[ImGuiCol_Separator as usize] = [0.32, 0.32, 0.32, 1.00];
    colors[ImGuiCol_SeparatorHovered as usize] = [0.32, 0.32, 0.32, 1.00];
    colors[ImGuiCol_SeparatorActive as usize] = [0.32, 0.32, 0.32, 1.00];
    colors[ImGuiCol_ResizeGrip as usize] = [1.00, 1.00, 1.00, 0.85];
    colors[ImGuiCol_ResizeGripHovered as usize] = [1.00, 1.00, 1.00, 0.60];
    colors[ImGuiCol_ResizeGripActive as usize] = [1.00, 1.00, 1.00, 0.90];
    colors[ImGuiCol_Tab as usize] = [0.07, 0.07, 0.07, 0.51];
    colors[ImGuiCol_TabHovered as usize] = [0.86, 0.23, 0.43, 0.67];
    colors[ImGuiCol_TabActive as usize] = [0.19, 0.19, 0.19, 0.57];
    colors[ImGuiCol_TabUnfocused as usize] = [0.05, 0.05, 0.05, 0.90];
    colors[ImGuiCol_TabUnfocusedActive as usize] = [0.13, 0.13, 0.13, 0.74];
    colors[ImGuiCol_PlotLines as usize] = [0.61, 0.61, 0.61, 1.00];
    colors[ImGuiCol_PlotLinesHovered as usize] = [1.00, 0.43, 0.35, 1.00];
    colors[ImGuiCol_PlotHistogram as usize] = [0.90, 0.70, 0.00, 1.00];
    colors[ImGuiCol_PlotHistogramHovered as usize] = [1.00, 0.60, 0.00, 1.00];
    colors[ImGuiCol_TableHeaderBg as usize] = [0.19, 0.19, 0.20, 1.00];
    colors[ImGuiCol_TableBorderStrong as usize] = [0.31, 0.31, 0.35, 1.00];
    colors[ImGuiCol_TableBorderLight as usize] = [0.23, 0.23, 0.25, 1.00];
    colors[ImGuiCol_TableRowBg as usize] = [0.00, 0.00, 0.00, 0.00];
    colors[ImGuiCol_TableRowBgAlt as usize] = [1.00, 1.00, 1.00, 0.07];
    colors[ImGuiCol_TextSelectedBg as usize] = [0.26, 0.59, 0.98, 0.35];
    colors[ImGuiCol_DragDropTarget as usize] = [1.00, 1.00, 0.00, 0.90];
    colors[ImGuiCol_NavHighlight as usize] = [0.26, 0.59, 0.98, 1.00];
    colors[ImGuiCol_NavWindowingHighlight as usize] = [1.00, 1.00, 1.00, 0.70];
    colors[ImGuiCol_NavWindowingDimBg as usize] = [0.80, 0.80, 0.80, 0.20];
    colors[ImGuiCol_ModalWindowDimBg as usize] = [0.80, 0.80, 0.80, 0.35];
}

#[allow(unused)]
pub(crate) fn set_style_minty_light(ctx: &mut hudhook::imgui::Context)
{
    ctx.style_mut().use_light_colors();

    let style = ctx.style_mut();
    let colors = &mut style.colors;

    use hudhook::imgui::sys::{ImGuiCol_Button, ImGuiCol_ButtonActive, ImGuiCol_ButtonHovered,
                              ImGuiCol_CheckMark, ImGuiCol_FrameBg, ImGuiCol_FrameBgActive,
                              ImGuiCol_FrameBgHovered, ImGuiCol_Header, ImGuiCol_HeaderActive,
                              ImGuiCol_HeaderHovered, ImGuiCol_ResizeGrip,
                              ImGuiCol_ResizeGripActive, ImGuiCol_ResizeGripHovered,
                              ImGuiCol_SliderGrab, ImGuiCol_SliderGrabActive, ImGuiCol_Tab,
                              ImGuiCol_TabActive, ImGuiCol_TabHovered, ImGuiCol_TitleBg,
                              ImGuiCol_TitleBgActive, ImGuiCol_TitleBgCollapsed};

    colors[ImGuiCol_FrameBg as usize] = [0.25, 0.25, 0.25, 1.00];
    colors[ImGuiCol_FrameBgHovered as usize] = [0.05, 0.90, 0.54, 1.00];
    colors[ImGuiCol_FrameBgActive as usize] = [0.05, 0.90, 0.54, 1.00];
    colors[ImGuiCol_TitleBg as usize] = [0.01, 0.35, 0.20, 1.00];
    colors[ImGuiCol_TitleBgActive as usize] = [0.01, 0.69, 0.40, 1.00];
    colors[ImGuiCol_TitleBgCollapsed as usize] = [0.01, 0.48, 0.28, 1.00];
    colors[ImGuiCol_CheckMark as usize] = [0.00, 0.51, 0.29, 1.00];
    colors[ImGuiCol_SliderGrab as usize] = [0.08, 0.72, 0.48, 1.00];
    colors[ImGuiCol_SliderGrabActive as usize] = [0.05, 0.90, 0.54, 1.00];
    colors[ImGuiCol_Button as usize] = [0.13, 0.43, 0.31, 1.00];
    colors[ImGuiCol_ButtonHovered as usize] = [0.00, 0.82, 0.47, 1.00];
    colors[ImGuiCol_ButtonActive as usize] = [0.05, 0.90, 0.54, 1.00];
    colors[ImGuiCol_Header as usize] = [0.00, 0.43, 0.25, 1.00];
    colors[ImGuiCol_HeaderHovered as usize] = [0.00, 0.79, 0.45, 1.00];
    colors[ImGuiCol_HeaderActive as usize] = [0.00, 0.63, 0.36, 1.00];
    colors[ImGuiCol_ResizeGrip as usize] = [0.00, 0.55, 0.31, 1.00];
    colors[ImGuiCol_ResizeGripHovered as usize] = [0.05, 0.90, 0.54, 1.00];
    colors[ImGuiCol_ResizeGripActive as usize] = [0.05, 0.90, 0.54, 1.00];
    colors[ImGuiCol_Tab as usize] = [0.00, 0.40, 0.23, 1.00];
    colors[ImGuiCol_TabHovered as usize] = [0.00, 0.75, 0.43, 1.00];
    colors[ImGuiCol_TabActive as usize] = [0.00, 0.68, 0.39, 1.00];
}

#[allow(unused)]
pub(crate) fn set_style_minty_mint(ctx: &mut hudhook::imgui::Context)
{
    ctx.style_mut().use_dark_colors();

    let style = ctx.style_mut();
    let colors = &mut style.colors;

    use hudhook::imgui::sys::{ImGuiCol_Button, ImGuiCol_ButtonActive, ImGuiCol_ButtonHovered,
                              ImGuiCol_CheckMark, ImGuiCol_FrameBg, ImGuiCol_FrameBgActive,
                              ImGuiCol_FrameBgHovered, ImGuiCol_Header, ImGuiCol_HeaderActive,
                              ImGuiCol_HeaderHovered, ImGuiCol_ResizeGrip,
                              ImGuiCol_ResizeGripActive, ImGuiCol_ResizeGripHovered,
                              ImGuiCol_SliderGrab, ImGuiCol_SliderGrabActive, ImGuiCol_Tab,
                              ImGuiCol_TabActive, ImGuiCol_TabHovered, ImGuiCol_TitleBg,
                              ImGuiCol_TitleBgActive, ImGuiCol_TitleBgCollapsed};

    colors[ImGuiCol_FrameBg as usize] = [0.25, 0.25, 0.25, 1.00];
    colors[ImGuiCol_FrameBgHovered as usize] = [0.05, 0.90, 0.54, 1.00];
    colors[ImGuiCol_FrameBgActive as usize] = [0.05, 0.90, 0.54, 1.00];
    colors[ImGuiCol_TitleBg as usize] = [0.01, 0.35, 0.20, 1.00];
    colors[ImGuiCol_TitleBgActive as usize] = [0.01, 0.69, 0.40, 1.00];
    colors[ImGuiCol_TitleBgCollapsed as usize] = [0.01, 0.48, 0.28, 1.00];
    colors[ImGuiCol_CheckMark as usize] = [0.00, 0.51, 0.29, 1.00];
    colors[ImGuiCol_SliderGrab as usize] = [0.08, 0.72, 0.48, 1.00];
    colors[ImGuiCol_SliderGrabActive as usize] = [0.05, 0.90, 0.54, 1.00];
    colors[ImGuiCol_Button as usize] = [0.13, 0.43, 0.31, 1.00];
    colors[ImGuiCol_ButtonHovered as usize] = [0.00, 0.82, 0.47, 1.00];
    colors[ImGuiCol_ButtonActive as usize] = [0.05, 0.90, 0.54, 1.00];
    colors[ImGuiCol_Header as usize] = [0.00, 0.43, 0.25, 1.00];
    colors[ImGuiCol_HeaderHovered as usize] = [0.00, 0.79, 0.45, 1.00];
    colors[ImGuiCol_HeaderActive as usize] = [0.00, 0.63, 0.36, 1.00];
    colors[ImGuiCol_ResizeGrip as usize] = [0.00, 0.55, 0.31, 1.00];
    colors[ImGuiCol_ResizeGripHovered as usize] = [0.05, 0.90, 0.54, 1.00];
    colors[ImGuiCol_ResizeGripActive as usize] = [0.05, 0.90, 0.54, 1.00];
    colors[ImGuiCol_Tab as usize] = [0.00, 0.40, 0.23, 1.00];
    colors[ImGuiCol_TabHovered as usize] = [0.00, 0.75, 0.43, 1.00];
    colors[ImGuiCol_TabActive as usize] = [0.00, 0.68, 0.39, 1.00];
}

