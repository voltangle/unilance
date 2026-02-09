package com.arvenora.lancemate

import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Window
import androidx.compose.ui.window.application
import androidx.compose.ui.window.rememberWindowState
import com.formdev.flatlaf.FlatClientProperties
import com.formdev.flatlaf.FlatLightLaf

fun main() {
    FlatLightLaf.setup()
    application {
        val windowState = rememberWindowState(
            width = 1000.dp,
            height = 600.dp
        )
        Window(
            onCloseRequest = ::exitApplication,
            state = windowState,
            title = "lancemate",
        ) {
            // Only for macOS, so I have the nice merged titlebar
            LaunchedEffect(window) {
                window.rootPane.putClientProperty("apple.awt.fullWindowContent", true)
                window.rootPane.putClientProperty("apple.awt.transparentTitleBar", true)
                window.rootPane.putClientProperty("apple.awt.windowTitleVisible", false)
                window.rootPane.putClientProperty(FlatClientProperties.MACOS_WINDOW_BUTTONS_SPACING,
                    FlatClientProperties.MACOS_WINDOW_BUTTONS_SPACING_LARGE);
            }
            App()
        }
    }
}