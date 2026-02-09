package com.arvenora.lancemate.platform

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.ColorScheme
import androidx.compose.material3.ExperimentalMaterial3ExpressiveApi
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.expressiveLightColorScheme
import androidx.compose.runtime.Composable

@OptIn(ExperimentalMaterial3ExpressiveApi::class)
@Composable
actual fun platformSystemColorScheme(): ColorScheme {
    return if (isSystemInDarkTheme()) darkColorScheme() else expressiveLightColorScheme()
}
