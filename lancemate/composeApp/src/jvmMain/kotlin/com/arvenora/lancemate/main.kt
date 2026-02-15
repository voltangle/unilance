package com.arvenora.lancemate

import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.text.input.delete
import androidx.compose.foundation.text.input.insert
import androidx.compose.foundation.text.input.rememberTextFieldState
import androidx.compose.foundation.text.input.selectAll
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.platform.LocalClipboardManager
import androidx.compose.ui.text.AnnotatedString
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Window
import androidx.compose.ui.window.application
import androidx.compose.ui.window.rememberWindowState
import com.formdev.flatlaf.FlatClientProperties
import com.formdev.flatlaf.FlatLightLaf
import dev.hansholz.advancedmenubar.AdvancedMacMenu
import dev.hansholz.advancedmenubar.CompatibilityMenu
import dev.hansholz.advancedmenubar.DefaultMacMenu
import dev.hansholz.advancedmenubar.MacCocoaMenu
import dev.hansholz.advancedmenubar.MenuVisibility

@OptIn(ExperimentalFoundationApi::class)
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
            @Suppress("Deprecation")
            val clipboard = LocalClipboardManager.current
            val textFieldState = rememberTextFieldState()

            CompatibilityMenu(window.title) {
                MacApplicationMenu {
                    About {}
                    Separator()
                    Services()
                    Separator()
                    Hide()
                    HideOthers()
                    ShowAll()
                    Separator()
                    Quit()
                }
                EditMenu {
                    Undo(enabled = textFieldState.undoState.canUndo) {
                        textFieldState.undoState.undo()
                    }
                    Redo(enabled = textFieldState.undoState.canRedo) {
                        textFieldState.undoState.redo()
                    }
                    Separator()
                    Cut(enabled = !textFieldState.selection.collapsed) {
                        val sel = textFieldState.selection
                        if (!sel.collapsed) {
                            clipboard.setText(AnnotatedString(textFieldState.text.substring(sel.start, sel.end)))
                            textFieldState.edit { delete(sel.start, sel.end) }
                        }
                    }
                    Copy(enabled = !textFieldState.selection.collapsed) {
                        val sel = textFieldState.selection
                        if (!sel.collapsed) {
                            clipboard.setText(AnnotatedString(textFieldState.text.substring(sel.start, sel.end)))
                        }
                    }
                    Paste(enabled = clipboard.hasText()) {
                        val paste = clipboard.getText()?.text ?: ""
                        val sel = textFieldState.selection
                        textFieldState.edit {
                            if (!sel.collapsed) delete(sel.start, sel.end)
                            insert(selection.start, paste)
                            placeCursorBeforeCharAt(sel.start + paste.length)
                        }
                    }
                    PasteAndMatchStyle(enabled = false) {}
                    Delete(enabled = !textFieldState.selection.collapsed) {
                        val sel = textFieldState.selection
                        if (!sel.collapsed) textFieldState.edit { delete(sel.start, sel.end) }
                    }
                    SelectAll(enabled = textFieldState.text.isNotEmpty()) {
                        textFieldState.edit { selectAll() }
                    }
                }
                ViewMenu(visibility = MenuVisibility.MACOS_ONLY) {
                    ShowToolbar(enabled = false) {}
                    CustomizeToolbar(enabled = false) {}
                    Separator()
                    ToggleFullScreen()
                }
                WindowMenu(visibility = MenuVisibility.MACOS_ONLY) {
                    Separator()
                    Close()
                    Minimize()
                    MinimizeAll()
                    Zoom()
                    BringAllToFront()
                }
            }
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